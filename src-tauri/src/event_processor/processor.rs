use crate::parser::LogEvent;
use crate::db::operations;
use rusqlite::Connection;
use std::collections::HashMap;
use chrono::{DateTime, Utc};

/// プロセッサーが発行するイベント
#[derive(Debug, Clone, serde::Serialize)]
#[serde(tag = "type")]
pub enum ProcessedEvent {
    LocalPlayerUpdated,  // ローカルプレイヤー（アカウント）が追加・更新された
    InstanceCreated { instance_id: i64 },
    InstanceEnded { instance_id: i64, ended_at: String },
    PlayerJoined { instance_id: i64 },
    PlayerLeft { instance_id: i64 },
}

/// イベントプロセッサー：LogEventをデータベースに保存
pub struct EventProcessor {
    current_local_player_id: Option<i64>,  // ローカルプレイヤー (is_local=1)
    current_instance_id: Option<i64>,
    player_ids: HashMap<String, i64>, // user_id -> player_id のマッピング
    pending_avatars: HashMap<String, (i64, DateTime<Utc>)>, // display_name -> (avatar_id, timestamp) のマッピング（PlayerJoined前のアバター情報）
}

impl EventProcessor {
    pub fn new() -> Self {
        Self {
            current_local_player_id: None,
            current_instance_id: None,
            player_ids: HashMap::new(),
            pending_avatars: HashMap::new(),
        }
    }

    /// データベースから最新のローカルプレイヤーとインスタンス情報を取得して状態を初期化
    pub fn initialize_from_db(&mut self, conn: &Connection) -> Result<(), rusqlite::Error> {
        // 1. 最後に認証されたローカルプレイヤーを取得
        let local_player_result = conn.query_row(
            "SELECT id FROM players WHERE is_local = 1 ORDER BY last_authenticated_at DESC LIMIT 1",
            [],
            |row| row.get::<_, i64>(0),
        );

        match local_player_result {
            Ok(player_id) => {
                self.current_local_player_id = Some(player_id);
                println!("EventProcessor initialized with local player ID: {}", player_id);

                // 2. 進行中のインスタンスを取得（そのプレイヤーの最新の in_progress インスタンス）
                let instance_result = conn.query_row(
                    "SELECT id FROM instances WHERE player_id = ?1 AND status = 'active' ORDER BY started_at DESC LIMIT 1",
                    [player_id],
                    |row| row.get::<_, i64>(0),
                );

                match instance_result {
                    Ok(instance_id) => {
                        self.current_instance_id = Some(instance_id);
                        println!("Found in-progress instance: {}", instance_id);

                        // 3. インスタンスに参加中のプレイヤー（left_atがNULL）のマッピングを復元
                        let mut stmt = conn.prepare(
                            "SELECT p.user_id, ip.player_id
                             FROM instance_players ip
                             JOIN players p ON ip.player_id = p.id
                             WHERE ip.instance_id = ?1 AND ip.left_at IS NULL"
                        )?;

                        let player_rows = stmt.query_map([instance_id], |row| {
                            Ok((
                                row.get::<_, String>(0)?,  // user_id
                                row.get::<_, i64>(1)?      // player_id
                            ))
                        })?;

                        for row in player_rows {
                            let (user_id, player_id) = row?;
                            self.player_ids.insert(user_id.clone(), player_id);
                        }

                        println!("Restored {} players in current instance", self.player_ids.len());
                    }
                    Err(rusqlite::Error::QueryReturnedNoRows) => {
                        println!("No in-progress instance found. Ready to start new instance.");
                    }
                    Err(e) => return Err(e),
                }
            }
            Err(rusqlite::Error::QueryReturnedNoRows) => {
                println!("No local player found in database. Waiting for authentication event.");
            }
            Err(e) => return Err(e),
        }

        Ok(())
    }

    /// LogEventを処理してデータベースに保存し、フロントエンドに通知すべきイベントを返す
    pub fn process_event(&mut self, conn: &Connection, event: LogEvent) -> Result<Option<ProcessedEvent>, rusqlite::Error> {
        let processed_event = match event {
            LogEvent::UserAuthenticated { timestamp, display_name, user_id } => {
                // ローカルプレイヤー（自分）を作成または更新
                let local_player_id = operations::upsert_local_player(
                    conn,
                    &display_name,
                    &user_id,
                    timestamp,
                )?;
                self.current_local_player_id = Some(local_player_id);
                println!("User authenticated: {} ({})", display_name, user_id);
                Some(ProcessedEvent::LocalPlayerUpdated)  // サイドバーのアカウントリストを更新
            }

            LogEvent::JoiningWorld { timestamp, world_id, instance_id, world_name } => {
                if let Some(local_player_id) = self.current_local_player_id {
                    // 前のインスタンスが終了していない場合、interrupted状態にする
                    if let Some(previous_instance_id) = self.current_instance_id {
                        conn.execute(
                            "UPDATE instances SET status = 'interrupted' WHERE id = ?1",
                            [previous_instance_id],
                        )?;
                        println!("Previous instance {} marked as interrupted", previous_instance_id);
                    }

                    // 新しいインスタンスを作成
                    let world_name_opt = if world_name.is_empty() { None } else { Some(world_name.as_str()) };
                    let instance_id_db = operations::create_instance(
                        conn,
                        local_player_id,
                        timestamp,
                        &world_id,
                        world_name_opt,
                        &instance_id,
                    )?;
                    self.current_instance_id = Some(instance_id_db);
                    self.player_ids.clear(); // 新しいインスタンスなのでプレイヤーマップをクリア
                    self.pending_avatars.clear(); // 保留中のアバター情報もクリア
                    println!("Joined world: {} (instance: {})", world_id, instance_id_db);

                    Some(ProcessedEvent::InstanceCreated { instance_id: instance_id_db })
                } else {
                    eprintln!("Warning: JoiningWorld event without authenticated user");
                    None
                }
            }

            LogEvent::EnteringRoom { world_name, .. } => {
                // 最後に作成したインスタンスのworld_nameを更新
                if let Some(instance_id) = self.current_instance_id {
                    conn.execute(
                        "UPDATE instances SET world_name = ?1 WHERE id = ?2",
                        rusqlite::params![world_name, instance_id],
                    )?;
                    println!("World name updated: {} (instance: {})", world_name, instance_id);
                }
                None  // ワールド名更新は通知不要（インスタンス作成時に既に通知済み）
            }

            LogEvent::PlayerJoined { timestamp, display_name, user_id } => {
                if let Some(instance_id) = self.current_instance_id {
                    // プレイヤーを作成または更新
                    let player_id = operations::upsert_player(
                        conn,
                        &display_name,
                        &user_id,
                        timestamp,
                    )?;

                    // 名前履歴を作成または更新
                    let display_name_history_id = operations::upsert_player_name_history(
                        conn,
                        player_id,
                        &display_name,
                        timestamp,
                    )?;

                    // インスタンスにプレイヤーを追加（名前履歴IDと共に）
                    operations::add_player_to_instance(
                        conn,
                        instance_id,
                        player_id,
                        display_name_history_id,
                        timestamp,
                    )?;

                    self.player_ids.insert(user_id.clone(), player_id);

                    // 保留中のアバター情報があれば記録
                    if let Some((avatar_id, avatar_timestamp)) = self.pending_avatars.remove(&display_name) {
                        operations::record_avatar_usage(
                            conn,
                            instance_id,
                            player_id,
                            avatar_id,
                            avatar_timestamp,
                        )?;
                        println!("Player joined: {} ({}) with pending avatar", display_name, user_id);
                    } else {
                        println!("Player joined: {} ({})", display_name, user_id);
                    }

                    Some(ProcessedEvent::PlayerJoined { instance_id })
                } else {
                    eprintln!("Warning: PlayerJoined event without active instance");
                    None
                }
            }

            LogEvent::ScreenshotTaken { timestamp, file_path } => {
                if let Some(instance_id) = self.current_instance_id {
                    operations::record_screenshot(
                        conn,
                        instance_id,
                        &file_path,
                        &timestamp,
                    )?;
                    println!("Screenshot taken: {}", file_path);
                } else {
                    eprintln!("Warning: Screenshot taken without active instance");
                }
                None  // スクリーンショットはリアルタイム通知不要（将来的にUI表示するかも）
            }

            LogEvent::DestroyingPlayer { timestamp, display_name } => {
                if let Some(instance_id) = self.current_instance_id {
                    // 自分のdisplay_nameかチェック
                    let is_local_player = if let Some(local_player_id) = self.current_local_player_id {
                        conn.query_row(
                            "SELECT display_name FROM players WHERE id = ?1 AND is_local = 1",
                            [local_player_id],
                            |row| row.get::<_, String>(0),
                        ).ok() == Some(display_name.clone())
                    } else {
                        false
                    };

                    if is_local_player {
                        // 自分が退出 = インスタンス終了
                        // Destroying順序が不定なため、left_atがまだ設定されていない全プレイヤーを一括更新
                        conn.execute(
                            "UPDATE instance_players
                             SET left_at = ?1
                             WHERE instance_id = ?2
                             AND left_at IS NULL",
                            rusqlite::params![timestamp.to_rfc3339(), instance_id],
                        )?;

                        operations::end_instance(conn, instance_id, timestamp)?;
                        println!("Leaving instance: instance {} ended, all remaining players marked as left", instance_id);
                        self.current_instance_id = None;
                        self.player_ids.clear();
                        Some(ProcessedEvent::InstanceEnded {
                            instance_id,
                            ended_at: timestamp.to_rfc3339(),
                        })
                    } else {
                        // 他のプレイヤーが退出
                        // display_nameからplayer_idを取得してleft_atを更新
                        if let Some((_user_id, &player_id)) = self.player_ids.iter()
                            .find(|(user_id, _)| {
                                conn.query_row(
                                    "SELECT display_name FROM players WHERE user_id = ?1",
                                    [user_id.as_str()],
                                    |row| row.get::<_, String>(0),
                                ).ok() == Some(display_name.clone())
                            }) {
                            operations::remove_player_from_instance(
                                conn,
                                instance_id,
                                player_id,
                                timestamp,
                            )?;
                            println!("Player {} left (destroying)", display_name);
                            Some(ProcessedEvent::PlayerLeft { instance_id })
                        } else {
                            None
                        }
                    }
                } else {
                    None
                }
            }

            LogEvent::AvatarChanged { timestamp, display_name, avatar_name } => {
                if let Some(instance_id) = self.current_instance_id {
                    // アバターを作成または更新
                    let avatar_id = operations::upsert_avatar_by_name(
                        conn,
                        &avatar_name,
                        None,
                        timestamp,
                    )?;

                    // display_nameから誰のアバター変更かを判定
                    // 1. まず自分（local_player）のdisplay_nameと比較
                    let is_local_player = if let Some(local_player_id) = self.current_local_player_id {
                        conn.query_row(
                            "SELECT display_name FROM players WHERE id = ?1 AND is_local = 1",
                            [local_player_id],
                            |row| row.get::<_, String>(0),
                        ).ok().map(|name| name == display_name).unwrap_or(false)
                    } else {
                        false
                    };

                    if is_local_player {
                        // 自分のアバター変更（local_player_id使用）
                        operations::record_avatar_usage(
                            conn,
                            instance_id,
                            self.current_local_player_id.unwrap(),
                            avatar_id,
                            timestamp,
                        )?;
                        println!("Avatar changed (self): {} -> {}", display_name, avatar_name);
                    } else {
                        // 他プレイヤーのアバター変更
                        // display_nameでplayer_idを探す（逆引き）
                        if let Some((_user_id, &player_id)) = self.player_ids.iter()
                            .find(|(user_id, _)| {
                                // players テーブルから display_name を取得して比較
                                conn.query_row(
                                    "SELECT display_name FROM players WHERE user_id = ?1",
                                    [user_id.as_str()],
                                    |row| row.get::<_, String>(0),
                                ).ok().map(|name| name == display_name).unwrap_or(false)
                            }) {
                            operations::record_avatar_usage(
                                conn,
                                instance_id,
                                player_id,
                                avatar_id,
                                timestamp,
                            )?;
                            println!("Avatar changed (player): {} -> {}", display_name, avatar_name);
                        } else {
                            // プレイヤーがまだ登録されていない場合、保留中として保存
                            // （OnPlayerJoinedイベントが後で来る）
                            self.pending_avatars.insert(display_name.clone(), (avatar_id, timestamp));
                            println!("Avatar changed (pending): {} -> {} (player not yet joined)", display_name, avatar_name);
                        }
                    }
                } else {
                    eprintln!("Warning: AvatarChanged event without active instance");
                }
                None  // アバター変更は通知不要（将来的にアバター履歴機能で使うかも）
            }
        };

        Ok(processed_event)
    }
}

impl Default for EventProcessor {
    fn default() -> Self {
        Self::new()
    }
}
