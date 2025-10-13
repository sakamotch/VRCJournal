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

                    // セッションにプレイヤーを追加
                    operations::add_player_to_session(
                        conn,
                        session_id,
                        player_id,
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
                    // アバターを作成または更新（名前ベース）
                    let avatar_id = operations::upsert_avatar_by_name(
                        conn,
                        &avatar_name,
                        None, // avatar_idは現状取得できない
                        timestamp,
                    )?;

                    // user_idを特定（ローカルユーザーまたはプレイヤー）
                    // display_nameからuser_idを逆引きする必要があるが、ここでは簡易実装
                    // 実際にはローカルユーザーのdisplay_nameと一致するか確認
                    let user_id = if let Some(local_user_id) = self.current_local_user_id {
                        // ローカルユーザーの情報を取得してdisplay_nameを比較
                        // 簡易実装: ローカルユーザーと仮定
                        if let Ok(Some(local_user)) = operations::get_local_user_by_user_id(conn, &self.get_current_user_id(conn)?) {
                            if local_user.display_name == display_name {
                                local_user.user_id
                            } else {
                                // プレイヤーの中から探す
                                self.find_user_id_by_display_name(&display_name).unwrap_or_default()
                            }
                        } else {
                            String::new()
                        }
                    } else {
                        String::new()
                    };

                    if !user_id.is_empty() {
                        // アバター使用履歴を記録
                        operations::record_avatar_usage(
                            conn,
                            session_id,
                            &user_id,
                            Some(avatar_id),
                            &avatar_name,
                            timestamp,
                        )?;
                        println!("Avatar changed: {} -> {}", display_name, avatar_name);
                    }
                } else {
                    eprintln!("Warning: AvatarChanged event without active session");
                }
            }
        }

        Ok(())
    }

    fn get_current_user_id(&self, conn: &Connection) -> Result<String, rusqlite::Error> {
        if let Some(local_user_id) = self.current_local_user_id {
            let user_id = conn.query_row(
                "SELECT user_id FROM local_users WHERE id = ?1",
                [local_user_id],
                |row| row.get(0),
            )?;
            Ok(user_id)
        } else {
            Ok(String::new())
        }
    }

    fn find_user_id_by_display_name(&self, _display_name: &str) -> Option<String> {
        // player_idsから逆引き（display_nameベースでは難しいので簡易実装）
        // 本来はplayers テーブルを検索すべき
        None
    }
}

impl Default for EventProcessor {
    fn default() -> Self {
        Self::new()
    }
}
