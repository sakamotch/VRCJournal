use crate::parser::LogEvent;
use crate::db::operations;
use rusqlite::Connection;
use std::collections::HashMap;

/// イベントプロセッサー：LogEventをデータベースに保存
pub struct EventProcessor {
    current_local_user_id: Option<i64>,
    current_session_id: Option<i64>,
    player_ids: HashMap<String, i64>, // user_id -> player_id のマッピング
}

impl EventProcessor {
    pub fn new() -> Self {
        Self {
            current_local_user_id: None,
            current_session_id: None,
            player_ids: HashMap::new(),
        }
    }

    /// LogEventを処理してデータベースに保存
    pub fn process_event(&mut self, conn: &Connection, event: LogEvent) -> Result<(), rusqlite::Error> {
        match event {
            LogEvent::UserAuthenticated { timestamp, display_name, user_id } => {
                // ローカルユーザーを作成または更新
                let local_user_id = operations::upsert_local_user(
                    conn,
                    &display_name,
                    &user_id,
                    timestamp,
                )?;
                self.current_local_user_id = Some(local_user_id);
                println!("User authenticated: {} ({})", display_name, user_id);
            }

            LogEvent::JoiningWorld { timestamp, world_id, instance_id, world_name } => {
                if let Some(local_user_id) = self.current_local_user_id {
                    // 前のセッションがあれば終了
                    if let Some(prev_session_id) = self.current_session_id {
                        operations::end_session(conn, prev_session_id, timestamp)?;
                        println!("Previous session ended: {}", prev_session_id);
                    }

                    // 新しいセッションを作成
                    let world_name_opt = if world_name.is_empty() { None } else { Some(world_name.as_str()) };
                    let session_id = operations::create_session(
                        conn,
                        local_user_id,
                        timestamp,
                        &world_id,
                        world_name_opt,
                        &instance_id,
                    )?;
                    self.current_session_id = Some(session_id);
                    self.player_ids.clear(); // 新しいセッションなのでプレイヤーマップをクリア
                    println!("Joined world: {} (session: {})", world_id, session_id);
                } else {
                    eprintln!("Warning: JoiningWorld event without authenticated user");
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
                    println!("Player joined: {} ({})", display_name, user_id);
                } else {
                    eprintln!("Warning: PlayerJoined event without active session");
                }
            }

            LogEvent::PlayerLeft { timestamp, display_name, user_id } => {
                if let Some(session_id) = self.current_session_id {
                    if let Some(&player_id) = self.player_ids.get(&user_id) {
                        // セッションからプレイヤーを退出
                        operations::remove_player_from_session(
                            conn,
                            session_id,
                            player_id,
                            timestamp,
                        )?;
                        println!("Player left: {} ({})", display_name, user_id);
                    }
                } else {
                    eprintln!("Warning: PlayerLeft event without active session");
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
                    // 1. まず自分（local_user）のdisplay_nameと比較
                    let is_local_user = if let Some(local_user_id) = self.current_local_user_id {
                        conn.query_row(
                            "SELECT display_name FROM local_users WHERE id = ?1",
                            [local_user_id],
                            |row| row.get::<_, String>(0),
                        ).ok().map(|name| name == display_name).unwrap_or(false)
                    } else {
                        false
                    };

                    if is_local_user {
                        // 自分のアバター変更（player_id = None）
                        operations::record_avatar_usage(
                            conn,
                            session_id,
                            None,
                            Some(avatar_id),
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
                                Some(player_id),
                                Some(avatar_id),
                                timestamp,
                            )?;
                            println!("Avatar changed (player): {} -> {}", display_name, avatar_name);
                        }
                    }
                } else {
                    eprintln!("Warning: AvatarChanged event without active session");
                }
            }
        }

        Ok(())
    }
}

impl Default for EventProcessor {
    fn default() -> Self {
        Self::new()
    }
}
