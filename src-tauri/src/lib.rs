mod db;
mod parser;
mod event_processor;
mod log_watcher;

use std::sync::{Arc, Mutex};
use tauri::Manager;
use tauri_plugin_opener::OpenerExt;
use event_processor::EventProcessor;

// グローバルステート
pub struct AppState {
    db: Arc<Mutex<db::Database>>,
    event_processor: Arc<Mutex<EventProcessor>>,
}

// Tauri Commands

/// ログファイルの監視を開始
#[tauri::command]
async fn start_log_watching(state: tauri::State<'_, AppState>) -> Result<String, String> {
    use std::collections::HashMap;
    use chrono::Utc;

    let mut watcher = log_watcher::LogWatcher::new()?;

    // データベースから既に処理済みのファイル位置を取得
    let db = state.db.lock().unwrap();
    let conn = db.connection();

    let tracked_files = db::operations::get_all_log_files(conn)
        .map_err(|e| format!("Failed to get tracked files: {}", e))?;

    let mut file_positions: HashMap<std::path::PathBuf, u64> = HashMap::new();
    for (path, _size, position, _modified) in tracked_files {
        file_positions.insert(std::path::PathBuf::from(path), position);
    }

    drop(db); // DB lockを解放

    // 全てのログファイルを読み込み（続きから）
    let events = watcher.read_all_logs(file_positions)?;

    let mut files_count = 0;
    let mut events_count = 0;

    // イベントをデータベースに保存
    let db = state.db.lock().unwrap();
    let conn = db.connection();
    let mut processor = state.event_processor.lock().unwrap();

    let mut current_file: Option<String> = None;

    for (file_path, event) in events {
        let file_path_str = file_path.to_string_lossy().to_string();

        // 新しいファイルに移った時
        if current_file.as_ref() != Some(&file_path_str) {
            current_file = Some(file_path_str.clone());
            files_count += 1;
        }

        processor.process_event(conn, event)
            .map_err(|e| format!("Failed to process event: {}", e))?;
        events_count += 1;
    }

    // ファイル位置をデータベースに保存
    let file_states = watcher.get_file_states();
    for (path, position) in file_states.iter() {
        let path_str = path.to_string_lossy().to_string();
        let metadata = std::fs::metadata(path).map_err(|e| format!("Failed to get metadata: {}", e))?;
        let file_size = metadata.len();
        let modified = metadata.modified()
            .map_err(|e| format!("Failed to get modified time: {}", e))?;
        let modified_dt = chrono::DateTime::<Utc>::from(modified);

        db::operations::upsert_log_file(conn, &path_str, file_size, modified_dt)
            .map_err(|e| format!("Failed to upsert log file: {}", e))?;
        db::operations::update_log_file_position(conn, &path_str, *position)
            .map_err(|e| format!("Failed to update position: {}", e))?;
    }

    drop(processor);
    drop(db);

    // ファイル監視を開始
    watcher.start_watching()?;

    // バックグラウンドでイベントを処理
    let db_clone = Arc::clone(&state.db);
    let processor_clone = Arc::clone(&state.event_processor);

    std::thread::spawn(move || {
        loop {
            if let Ok((file_path, event)) = watcher.recv_event() {
                let db = db_clone.lock().unwrap();
                let conn = db.connection();
                let mut processor = processor_clone.lock().unwrap();

                if let Err(e) = processor.process_event(conn, event) {
                    eprintln!("Failed to process event: {}", e);
                    continue;
                }

                // ファイル位置を更新
                let file_states = watcher.get_file_states();
                if let Some(position) = file_states.get(&file_path) {
                    let path_str = file_path.to_string_lossy().to_string();
                    if let Err(e) = db::operations::update_log_file_position(conn, &path_str, *position) {
                        eprintln!("Failed to update file position: {}", e);
                    }
                }
            }
        }
    });

    Ok(format!(
        "Log watching started\nProcessed {} files with {} events",
        files_count, events_count
    ))
}

/// VRChatログディレクトリのパスを取得
#[tauri::command]
async fn get_log_path() -> Result<String, String> {
    log_watcher::get_vrchat_log_path()
        .map(|p| p.to_string_lossy().to_string())
}

/// インスタンス招待URLを生成
#[tauri::command]
async fn generate_invite_url(world_id: String, instance_id: String) -> Result<String, String> {
    // VRChatのWeb招待URL形式
    // https://vrchat.com/home/launch?worldId=wrld_xxx&instanceId=instance_id
    let url = format!(
        "https://vrchat.com/home/launch?worldId={}&instanceId={}",
        world_id, instance_id
    );
    Ok(url)
}

/// インスタンス招待URLを生成してデフォルトブラウザで開く
#[tauri::command]
async fn open_invite_url(app: tauri::AppHandle, world_id: String, instance_id: String) -> Result<String, String> {
    // VRChatのWeb招待URL形式
    let url = format!(
        "https://vrchat.com/home/launch?worldId={}&instanceId={}",
        world_id, instance_id
    );

    // デフォルトブラウザで開く
    app.opener().open_url(&url, None::<&str>)
        .map_err(|e| format!("Failed to open URL: {}", e))?;

    Ok(url)
}

/// ローカルユーザー一覧を取得
#[tauri::command]
async fn get_local_users(state: tauri::State<'_, AppState>) -> Result<serde_json::Value, String> {
    let db = state.db.lock().unwrap();
    let conn = db.connection();

    let users = db::operations::get_all_local_users(conn)
        .map_err(|e| format!("Failed to get local users: {}", e))?;

    let json = serde_json::json!(
        users.into_iter().map(|u| {
            serde_json::json!({
                "id": u.id,
                "displayName": u.display_name,
                "userId": u.user_id,
                "firstAuthenticatedAt": u.first_authenticated_at.to_rfc3339(),
                "lastAuthenticatedAt": u.last_authenticated_at.to_rfc3339(),
            })
        }).collect::<Vec<_>>()
    );

    Ok(json)
}

/// セッション一覧を取得
#[tauri::command]
async fn get_sessions(
    state: tauri::State<'_, AppState>,
    local_user_id: Option<i64>,
    limit: Option<i64>,
) -> Result<serde_json::Value, String> {
    let db = state.db.lock().unwrap();
    let conn = db.connection();

    let limit = limit.unwrap_or(50);

    let query = if let Some(user_id) = local_user_id {
        format!(
            "SELECT s.id, s.local_user_id, lu.display_name as user_name, s.started_at, s.ended_at,
                    s.world_id, s.world_name, s.instance_id,
                    (SELECT COUNT(*) FROM session_players WHERE session_id = s.id) as player_count
             FROM sessions s
             JOIN local_users lu ON s.local_user_id = lu.id
             WHERE s.local_user_id = {}
             ORDER BY s.started_at DESC
             LIMIT {}",
            user_id, limit
        )
    } else {
        format!(
            "SELECT s.id, s.local_user_id, lu.display_name as user_name, s.started_at, s.ended_at,
                    s.world_id, s.world_name, s.instance_id,
                    (SELECT COUNT(*) FROM session_players WHERE session_id = s.id) as player_count
             FROM sessions s
             JOIN local_users lu ON s.local_user_id = lu.id
             ORDER BY s.started_at DESC
             LIMIT {}",
            limit
        )
    };

    let mut stmt = conn.prepare(&query)
        .map_err(|e| format!("Failed to prepare statement: {}", e))?;

    let sessions = stmt.query_map([], |row| {
        Ok(serde_json::json!({
            "id": row.get::<_, i64>(0)?,
            "localUserId": row.get::<_, i64>(1)?,
            "userName": row.get::<_, String>(2)?,
            "startedAt": row.get::<_, String>(3)?,
            "endedAt": row.get::<_, Option<String>>(4)?,
            "worldId": row.get::<_, String>(5)?,
            "worldName": row.get::<_, Option<String>>(6)?,
            "instanceId": row.get::<_, String>(7)?,
            "playerCount": row.get::<_, i64>(8)?,
        }))
    })
    .map_err(|e| format!("Failed to query sessions: {}", e))?
    .collect::<Result<Vec<_>, _>>()
    .map_err(|e| format!("Failed to collect sessions: {}", e))?;

    Ok(serde_json::json!(sessions))
}

/// データベースの統計情報を取得
#[tauri::command]
async fn get_database_stats(state: tauri::State<'_, AppState>) -> Result<String, String> {
    let db = state.db.lock().unwrap();
    let conn = db.connection();

    let local_users: i64 = conn.query_row(
        "SELECT COUNT(*) FROM local_users",
        [],
        |row| row.get(0)
    ).unwrap_or(0);

    let sessions: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sessions",
        [],
        |row| row.get(0)
    ).unwrap_or(0);

    let players: i64 = conn.query_row(
        "SELECT COUNT(*) FROM players",
        [],
        |row| row.get(0)
    ).unwrap_or(0);

    Ok(format!(
        "Local Users: {}, Sessions: {}, Players: {}",
        local_users, sessions, players
    ))
}

/// セッションのプレイヤー一覧を取得
#[tauri::command]
async fn get_session_players(
    state: tauri::State<'_, AppState>,
    session_id: i64,
) -> Result<serde_json::Value, String> {
    let db = state.db.lock().unwrap();
    let conn = db.connection();

    let players = db::operations::get_players_in_session(conn, session_id)
        .map_err(|e| format!("Failed to get players: {}", e))?;

    let json = serde_json::json!(
        players.into_iter().map(|p| {
            serde_json::json!({
                "id": p.id,
                "displayName": p.display_name,
                "userId": p.user_id,
                "firstSeenAt": p.first_seen_at.to_rfc3339(),
                "lastSeenAt": p.last_seen_at.to_rfc3339(),
            })
        }).collect::<Vec<_>>()
    );

    Ok(json)
}

/// プレイヤーのVRChatユーザーページをデフォルトブラウザで開く
#[tauri::command]
async fn open_user_page(app: tauri::AppHandle, user_id: String) -> Result<String, String> {
    let url = format!("https://vrchat.com/home/user/{}", user_id);

    // デフォルトブラウザで開く
    app.opener().open_url(&url, None::<&str>)
        .map_err(|e| format!("Failed to open URL: {}", e))?;

    Ok(url)
}

/// テスト用: 指定したログファイルを読み込んでパース＆DB登録
#[tauri::command]
async fn test_parse_log_file(
    state: tauri::State<'_, AppState>,
    file_path: String,
) -> Result<String, String> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use parser::log_parser::VRChatLogParser;

    let file = File::open(&file_path)
        .map_err(|e| format!("Failed to open file: {}", e))?;

    let reader = BufReader::new(file);
    let parser = VRChatLogParser::new();

    let mut total_lines = 0;
    let mut parsed_events = 0;
    let mut event_types = std::collections::HashMap::new();

    let db = state.db.lock().unwrap();
    let conn = db.connection();
    let mut processor = state.event_processor.lock().unwrap();

    for line in reader.lines() {
        total_lines += 1;
        let line = line.map_err(|e| format!("Failed to read line: {}", e))?;

        if let Some(event) = parser.parse_line(&line) {
            parsed_events += 1;

            // イベントタイプをカウント
            let event_type = match &event {
                parser::types::LogEvent::UserAuthenticated { .. } => "UserAuthenticated",
                parser::types::LogEvent::JoiningWorld { .. } => "JoiningWorld",
                parser::types::LogEvent::EnteringRoom { .. } => "EnteringRoom",
                parser::types::LogEvent::PlayerJoined { .. } => "PlayerJoined",
                parser::types::LogEvent::PlayerLeft { .. } => "PlayerLeft",
                parser::types::LogEvent::AvatarChanged { .. } => "AvatarChanged",
            };
            *event_types.entry(event_type).or_insert(0) += 1;

            // データベースに保存
            processor.process_event(conn, event)
                .map_err(|e| format!("Failed to process event: {}", e))?;
        }
    }

    // 統計情報を取得
    let local_users: i64 = conn.query_row(
        "SELECT COUNT(*) FROM local_users",
        [],
        |row| row.get(0)
    ).unwrap_or(0);

    let sessions: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sessions",
        [],
        |row| row.get(0)
    ).unwrap_or(0);

    let players: i64 = conn.query_row(
        "SELECT COUNT(*) FROM players",
        [],
        |row| row.get(0)
    ).unwrap_or(0);

    let avatars: i64 = conn.query_row(
        "SELECT COUNT(*) FROM avatars",
        [],
        |row| row.get(0)
    ).unwrap_or(0);

    let mut result = format!(
        "=== Parse Results ===\n\
         Total lines: {}\n\
         Parsed events: {}\n\
         \n\
         === Event Types ===\n",
        total_lines, parsed_events
    );

    for (event_type, count) in event_types.iter() {
        result.push_str(&format!("{}: {}\n", event_type, count));
    }

    result.push_str(&format!(
        "\n\
         === Database Stats ===\n\
         Local Users: {}\n\
         Sessions: {}\n\
         Players: {}\n\
         Avatars: {}\n",
        local_users, sessions, players, avatars
    ));

    Ok(result)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // データベースパスを決定
            let app_data_dir = app.path().app_data_dir()
                .expect("Failed to get app data dir");

            // ディレクトリが存在しない場合は作成
            std::fs::create_dir_all(&app_data_dir)
                .expect("Failed to create app data directory");

            let db_path = app_data_dir.join("vrcjournal.db");

            // データベース初期化
            let database = db::Database::open(db_path)
                .expect("Failed to open database");

            database.migrate()
                .expect("Failed to run migrations");

            let app_state = AppState {
                db: Arc::new(Mutex::new(database)),
                event_processor: Arc::new(Mutex::new(EventProcessor::new())),
            };

            app.manage(app_state);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            start_log_watching,
            get_log_path,
            generate_invite_url,
            open_invite_url,
            get_local_users,
            get_sessions,
            get_database_stats,
            get_session_players,
            open_user_page,
            test_parse_log_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
