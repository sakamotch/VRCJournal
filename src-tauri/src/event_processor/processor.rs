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
    SessionCreated { session_id: i64 },
    SessionEnded { session_id: i64, ended_at: String },
    PlayerJoined { session_id: i64 },
    PlayerLeft { session_id: i64 },
}

/// イベントプロセッサー：LogEventをデータベースに保存
pub struct EventProcessor {
    current_local_player_id: Option<i64>,  // ローカルプレイヤー (is_local=1)
    current_session_id: Option<i64>,
    player_ids: HashMap<String, i64>, // user_id -> player_id のマッピング
    pending_avatars: HashMap<String, (i64, DateTime<Utc>)>, // display_name -> (avatar_id, timestamp) のマッピング（PlayerJoined前のアバター情報）
}

impl EventProcessor {
    pub fn new() -> Self {
        Self {
            current_local_player_id: None,
            current_session_id: None,
            player_ids: HashMap::new(),
            pending_avatars: HashMap::new(),
        }
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
                    // 前のセッションが終了していない場合、interrupted状態にする
                    if let Some(previous_session_id) = self.current_session_id {
                        conn.execute(
                            "UPDATE sessions SET status = 'interrupted' WHERE id = ?1",
                            [previous_session_id],
                        )?;
                        println!("Previous session {} marked as interrupted", previous_session_id);
                    }

                    // 新しいセッションを作成
                    let world_name_opt = if world_name.is_empty() { None } else { Some(world_name.as_str()) };
                    let session_id = operations::create_session(
                        conn,
                        local_player_id,
                        timestamp,
                        &world_id,
                        world_name_opt,
                        &instance_id,
                    )?;
                    self.current_session_id = Some(session_id);
                    self.player_ids.clear(); // 新しいセッションなのでプレイヤーマップをクリア
                    self.pending_avatars.clear(); // 保留中のアバター情報もクリア
                    println!("Joined world: {} (session: {})", world_id, session_id);

                    Some(ProcessedEvent::SessionCreated { session_id })
                } else {
                    eprintln!("Warning: JoiningWorld event without authenticated user");
                    None
                }
            }

            LogEvent::EnteringRoom { world_name, .. } => {
                // 最後に作成したセッションのworld_nameを更新
                if let Some(session_id) = self.current_session_id {
                    conn.execute(
                        "UPDATE sessions SET world_name = ?1 WHERE id = ?2",
                        rusqlite::params![world_name, session_id],
                    )?;
                    println!("World name updated: {} (session: {})", world_name, session_id);
                }
                None  // ワールド名更新は通知不要（セッション作成時に既に通知済み）
            }

            LogEvent::PlayerJoined { timestamp, display_name, user_id } => {
                if let Some(session_id) = self.current_session_id {
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

                    // セッションにプレイヤーを追加（名前履歴IDと共に）
                    operations::add_player_to_session(
                        conn,
                        session_id,
                        player_id,
                        display_name_history_id,
                        timestamp,
                    )?;

                    self.player_ids.insert(user_id.clone(), player_id);

                    // 保留中のアバター情報があれば記録
                    if let Some((avatar_id, avatar_timestamp)) = self.pending_avatars.remove(&display_name) {
                        operations::record_avatar_usage(
                            conn,
                            session_id,
                            player_id,
                            avatar_id,
                            avatar_timestamp,
                        )?;
                        println!("Player joined: {} ({}) with pending avatar", display_name, user_id);
                    } else {
                        println!("Player joined: {} ({})", display_name, user_id);
                    }

                    Some(ProcessedEvent::PlayerJoined { session_id })
                } else {
                    eprintln!("Warning: PlayerJoined event without active session");
                    None
                }
            }

            LogEvent::ScreenshotTaken { timestamp, file_path } => {
                if let Some(session_id) = self.current_session_id {
                    operations::record_screenshot(
                        conn,
                        session_id,
                        &file_path,
                        &timestamp,
                    )?;
                    println!("Screenshot taken: {}", file_path);
                } else {
                    eprintln!("Warning: Screenshot taken without active session");
                }
                None  // スクリーンショットはリアルタイム通知不要（将来的にUI表示するかも）
            }

            LogEvent::DestroyingPlayer { timestamp, display_name } => {
                if let Some(session_id) = self.current_session_id {
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
                        // 自分が退出 = セッション終了
                        // Destroying順序が不定なため、left_atがまだ設定されていない全プレイヤーを一括更新
                        conn.execute(
                            "UPDATE session_players
                             SET left_at = ?1
                             WHERE session_id = ?2
                             AND left_at IS NULL",
                            rusqlite::params![timestamp.to_rfc3339(), session_id],
                        )?;

                        operations::end_session(conn, session_id, timestamp)?;
                        println!("Leaving instance: session {} ended, all remaining players marked as left", session_id);
                        self.current_session_id = None;
                        self.player_ids.clear();
                        Some(ProcessedEvent::SessionEnded {
                            session_id,
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
                            operations::remove_player_from_session(
                                conn,
                                session_id,
                                player_id,
                                timestamp,
                            )?;
                            println!("Player {} left (destroying)", display_name);
                            Some(ProcessedEvent::PlayerLeft { session_id })
                        } else {
                            None
                        }
                    }
                } else {
                    None
                }
            }

            LogEvent::AvatarChanged { timestamp, display_name, avatar_name } => {
                if let Some(session_id) = self.current_session_id {
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
                            session_id,
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
                                session_id,
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
                    eprintln!("Warning: AvatarChanged event without active session");
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
