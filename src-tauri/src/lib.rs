mod db;
mod event_processor;
mod log_watcher;
mod parser;

use event_processor::EventProcessor;
use std::sync::{Arc, Mutex};
use tauri::{Emitter, Manager};
use tauri_plugin_opener::OpenerExt;

// グローバルステート
#[derive(Clone)]
pub struct AppState {
    db: Arc<Mutex<db::Database>>,
    event_processor: Arc<Mutex<EventProcessor>>,
    backend_ready: Arc<Mutex<bool>>,
}

// Tauri Commands

/// バックエンドの準備完了状態を確認
#[tauri::command]
async fn is_backend_ready(state: tauri::State<'_, AppState>) -> Result<bool, String> {
    Ok(*state.backend_ready.lock().unwrap())
}

/// VRChatログディレクトリのパスを取得
#[tauri::command]
async fn get_log_path() -> Result<String, String> {
    log_watcher::get_vrchat_log_path().map(|p| p.to_string_lossy().to_string())
}

/// インスタンス招待URLを生成してデフォルトブラウザで開く
#[tauri::command]
async fn open_invite_url(
    app: tauri::AppHandle,
    world_id: String,
    instance_id: String,
) -> Result<String, String> {
    // VRChatのWeb招待URL形式
    let url = format!(
        "https://vrchat.com/home/launch?worldId={}&instanceId={}",
        world_id, instance_id
    );

    // デフォルトブラウザで開く
    app.opener()
        .open_url(&url, None::<&str>)
        .map_err(|e| format!("Failed to open URL: {}", e))?;

    Ok(url)
}

/// ローカルプレイヤー（自分のアカウント）一覧を取得
#[tauri::command]
async fn get_local_users(state: tauri::State<'_, AppState>) -> Result<serde_json::Value, String> {
    let db = state.db.lock().unwrap();
    let conn = db.connection();

    let players = db::operations::get_all_local_players(conn)
        .map_err(|e| format!("Failed to get local players: {}", e))?;

    let json = serde_json::json!(players
        .into_iter()
        .map(|p| {
            serde_json::json!({
                "id": p.id,
                "displayName": p.display_name,
                "userId": p.user_id,
                "firstAuthenticatedAt": p.first_authenticated_at
                    .expect("Local player must have first_authenticated_at")
                    .to_rfc3339(),
                "lastAuthenticatedAt": p.last_authenticated_at
                    .expect("Local player must have last_authenticated_at")
                    .to_rfc3339(),
            })
        })
        .collect::<Vec<_>>());

    Ok(json)
}

/// インスタンス一覧を取得
/// local_user_id: 0 = 全アカウント, 1以上 = 特定のアカウント
#[tauri::command]
async fn get_instances(
    state: tauri::State<'_, AppState>,
    local_user_id: i64,
    limit: Option<i64>,
) -> Result<serde_json::Value, String> {
    let db = state.db.lock().unwrap();
    let conn = db.connection();

    let limit = limit.unwrap_or(50);

    let query = if local_user_id > 0 {
        format!(
            "SELECT i.id, i.player_id, p.display_name as user_name, i.started_at, i.ended_at,
                    i.world_id, i.world_name, i.instance_id, i.status,
                    (SELECT COUNT(DISTINCT player_id) FROM instance_players WHERE instance_id = i.id) as player_count,
                    (SELECT COUNT(*) FROM screenshots WHERE instance_id = i.id) as screenshot_count
             FROM instances i
             JOIN players p ON i.player_id = p.id
             WHERE i.player_id = {} AND p.is_local = 1
             ORDER BY i.started_at DESC
             LIMIT {}",
            local_user_id, limit
        )
    } else {
        format!(
            "SELECT i.id, i.player_id, p.display_name as user_name, i.started_at, i.ended_at,
                    i.world_id, i.world_name, i.instance_id, i.status,
                    (SELECT COUNT(DISTINCT player_id) FROM instance_players WHERE instance_id = i.id) as player_count,
                    (SELECT COUNT(*) FROM screenshots WHERE instance_id = i.id) as screenshot_count
             FROM instances i
             JOIN players p ON i.player_id = p.id
             WHERE p.is_local = 1
             ORDER BY i.started_at DESC
             LIMIT {}",
            limit
        )
    };

    let mut stmt = conn
        .prepare(&query)
        .map_err(|e| format!("Failed to prepare statement: {}", e))?;

    let instances = stmt
        .query_map([], |row| {
            Ok(serde_json::json!({
                "id": row.get::<_, i64>(0)?,
                "localUserId": row.get::<_, i64>(1)?,
                "userName": row.get::<_, String>(2)?,
                "startedAt": row.get::<_, String>(3)?,
                "endedAt": row.get::<_, Option<String>>(4)?,
                "worldId": row.get::<_, String>(5)?,
                "worldName": row.get::<_, Option<String>>(6)?,
                "instanceId": row.get::<_, String>(7)?,
                "status": row.get::<_, String>(8)?,
                "playerCount": row.get::<_, i64>(9)?,
                "screenshotCount": row.get::<_, i64>(10)?,
            }))
        })
        .map_err(|e| format!("Failed to query instances: {}", e))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Failed to collect instances: {}", e))?;

    Ok(serde_json::json!(instances))
}

/// 特定のインスタンスを取得
#[tauri::command]
async fn get_instance_by_id(
    state: tauri::State<'_, AppState>,
    instance_id: i64,
) -> Result<serde_json::Value, String> {
    let db = state.db.lock().unwrap();
    let conn = db.connection();

    let instance = conn.query_row(
        "SELECT i.id, i.player_id, p.display_name as user_name, i.started_at, i.ended_at,
                i.world_id, i.world_name, i.instance_id, i.status,
                (SELECT COUNT(DISTINCT player_id) FROM instance_players WHERE instance_id = i.id) as player_count
         FROM instances i
         JOIN players p ON i.player_id = p.id
         WHERE i.id = ?1 AND p.is_local = 1",
        [instance_id],
        |row| {
            Ok(serde_json::json!({
                "id": row.get::<_, i64>(0)?,
                "localUserId": row.get::<_, i64>(1)?,
                "userName": row.get::<_, String>(2)?,
                "startedAt": row.get::<_, String>(3)?,
                "endedAt": row.get::<_, Option<String>>(4)?,
                "worldId": row.get::<_, String>(5)?,
                "worldName": row.get::<_, Option<String>>(6)?,
                "instanceId": row.get::<_, String>(7)?,
                "status": row.get::<_, String>(8)?,
                "playerCount": row.get::<_, i64>(9)?,
            }))
        }
    )
    .map_err(|e| format!("Failed to get instance: {}", e))?;

    Ok(instance)
}

/// インスタンスのスクリーンショット一覧を取得
#[tauri::command]
async fn get_instance_screenshots(
    state: tauri::State<'_, AppState>,
    instance_id: i64,
) -> Result<serde_json::Value, String> {
    use std::path::Path;

    let db = state.db.lock().unwrap();
    let conn = db.connection();

    let screenshots = db::operations::get_instance_screenshots(conn, instance_id)
        .map_err(|e| format!("Failed to get screenshots: {}", e))?;

    let result: Vec<serde_json::Value> = screenshots
        .iter()
        .map(|(id, file_path, taken_at)| {
            let exists = Path::new(file_path).exists();
            serde_json::json!({
                "id": id,
                "filePath": file_path,
                "takenAt": taken_at,
                "exists": exists,
            })
        })
        .collect();

    Ok(serde_json::json!(result))
}

/// スクリーンショットのディレクトリをエクスプローラーで開く
#[tauri::command]
async fn open_screenshot_directory(file_path: String) -> Result<(), String> {
    use std::path::Path;

    let path = Path::new(&file_path);
    let dir = path.parent().ok_or("Failed to get parent directory")?;

    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer")
            .arg(dir)
            .spawn()
            .map_err(|e| format!("Failed to open directory: {}", e))?;
    }

    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(dir)
            .spawn()
            .map_err(|e| format!("Failed to open directory: {}", e))?;
    }

    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(dir)
            .spawn()
            .map_err(|e| format!("Failed to open directory: {}", e))?;
    }

    Ok(())
}

/// データベースの統計情報を取得
#[tauri::command]
async fn get_database_stats(state: tauri::State<'_, AppState>) -> Result<String, String> {
    let db = state.db.lock().unwrap();
    let conn = db.connection();

    let local_players: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM players WHERE is_local = 1",
            [],
            |row| row.get(0),
        )
        .unwrap_or(0);

    let instances: i64 = conn
        .query_row("SELECT COUNT(*) FROM instances", [], |row| row.get(0))
        .unwrap_or(0);

    let players: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM players WHERE is_local = 0",
            [],
            |row| row.get(0),
        )
        .unwrap_or(0);

    Ok(format!(
        "Local Players: {}, Instances: {}, Remote Players: {}",
        local_players, instances, players
    ))
}

/// インスタンスのプレイヤー一覧を取得
#[tauri::command]
async fn get_instance_players(
    state: tauri::State<'_, AppState>,
    instance_id: i64,
) -> Result<serde_json::Value, String> {
    let db = state.db.lock().unwrap();
    let conn = db.connection();

    let players = db::operations::get_players_in_instance(conn, instance_id)
        .map_err(|e| format!("Failed to get players: {}", e))?;

    let json = serde_json::json!(players
        .into_iter()
        .map(|p| {
            serde_json::json!({
                "id": p.id,
                "displayName": p.display_name,
                "displayNameAtJoin": p.display_name_at_join,
                "userId": p.user_id,
                "firstSeenAt": p.first_seen_at.to_rfc3339(),
                "lastSeenAt": p.last_seen_at.to_rfc3339(),
                "joinedAt": p.joined_at.to_rfc3339(),
                "leftAt": p.left_at.map(|dt| dt.to_rfc3339()),
            })
        })
        .collect::<Vec<_>>());

    Ok(json)
}

/// プレイヤーのVRChatユーザーページをデフォルトブラウザで開く
#[tauri::command]
async fn open_user_page(app: tauri::AppHandle, user_id: String) -> Result<String, String> {
    let url = format!("https://vrchat.com/home/user/{}", user_id);

    // デフォルトブラウザで開く
    app.opener()
        .open_url(&url, None::<&str>)
        .map_err(|e| format!("Failed to open URL: {}", e))?;

    Ok(url)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_os::init())
        .setup(|app| {
            use chrono::Utc;
            use std::collections::HashMap;

            // データベースパスを決定
            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("Failed to get app data dir");

            // ディレクトリが存在しない場合は作成
            std::fs::create_dir_all(&app_data_dir).expect("Failed to create app data directory");

            let db_path = app_data_dir.join("vrcjournal.db");

            // データベース初期化
            let database = db::Database::open(db_path).expect("Failed to open database");

            database.migrate().expect("Failed to run migrations");

            let app_state = AppState {
                db: Arc::new(Mutex::new(database)),
                event_processor: Arc::new(Mutex::new(EventProcessor::new())),
                backend_ready: Arc::new(Mutex::new(false)),
            };

            app.manage(app_state.clone());

            // バックグラウンドスレッドでログ監視を自動開始
            let app_handle = app.handle().clone();
            std::thread::spawn(move || {
                match log_watcher::LogWatcher::new() {
                    Ok(mut watcher) => {
                        // データベースから処理済みファイル位置を取得
                        let db = app_state.db.lock().unwrap();
                        let conn = db.connection();

                        let file_positions = match db::operations::get_all_log_files(conn) {
                            Ok(tracked_files) => {
                                let mut positions = HashMap::new();
                                for (path, _size, position, _modified) in tracked_files {
                                    positions.insert(std::path::PathBuf::from(path), position);
                                }
                                positions
                            }
                            Err(e) => {
                                eprintln!("Failed to get tracked files: {}", e);
                                HashMap::new()
                            }
                        };
                        drop(db);

                        // EventProcessorをデータベースから初期化
                        {
                            let db = app_state.db.lock().unwrap();
                            let conn = db.connection();
                            let mut processor = app_state.event_processor.lock().unwrap();
                            if let Err(e) = processor.initialize_from_db(conn) {
                                eprintln!(
                                    "Failed to initialize EventProcessor from database: {}",
                                    e
                                );
                            }
                        }

                        // 全てのログファイルを読み込み
                        match watcher.read_all_logs(file_positions) {
                            Ok(events) => {
                                let db = app_state.db.lock().unwrap();
                                let conn = db.connection();
                                let mut processor = app_state.event_processor.lock().unwrap();

                                let mut events_count = 0;
                                for (_file_path, event) in events {
                                    if let Err(e) = processor.process_event(conn, event) {
                                        eprintln!("Failed to process event: {}", e);
                                    } else {
                                        events_count += 1;
                                    }
                                }

                                // ファイル位置をデータベースに保存
                                let file_states = watcher.get_file_states();
                                for (path, position) in file_states.iter() {
                                    let path_str = path.to_string_lossy().to_string();
                                    if let Ok(metadata) = std::fs::metadata(path) {
                                        let file_size = metadata.len();
                                        if let Ok(modified) = metadata.modified() {
                                            let modified_dt =
                                                chrono::DateTime::<Utc>::from(modified);
                                            let _ = db::operations::upsert_log_file(
                                                conn,
                                                &path_str,
                                                file_size,
                                                modified_dt,
                                            );
                                            let _ = db::operations::update_log_file_position(
                                                conn, &path_str, *position,
                                            );
                                        }
                                    }
                                }

                                drop(processor);
                                drop(db);

                                println!(
                                    "Initial log processing completed: {} events",
                                    events_count
                                );

                                // ファイル監視を開始
                                if let Err(e) = watcher.start_watching() {
                                    eprintln!("Failed to start watching: {}", e);
                                    return;
                                }

                                // バックエンド準備完了フラグを設定
                                *app_state.backend_ready.lock().unwrap() = true;

                                // フロントエンドに初期化完了を通知
                                if let Err(e) = app_handle.emit("backend-ready", ()) {
                                    eprintln!("Failed to emit backend-ready: {}", e);
                                }

                                // バックグラウンドでイベントを処理
                                let db_clone = Arc::clone(&app_state.db);
                                let processor_clone = Arc::clone(&app_state.event_processor);

                                loop {
                                    if let Ok((file_path, event)) = watcher.recv_event() {
                                        let db = db_clone.lock().unwrap();
                                        let conn = db.connection();
                                        let mut processor = processor_clone.lock().unwrap();

                                        match processor.process_event(conn, event) {
                                            Ok(Some(processed_event)) => {
                                                // フロントエンドにイベントを通知（詳細情報付き）
                                                if let Err(e) =
                                                    app_handle.emit("log-event", &processed_event)
                                                {
                                                    eprintln!("Failed to emit event: {}", e);
                                                }
                                            }
                                            Ok(None) => {
                                                // 通知不要なイベント
                                            }
                                            Err(e) => {
                                                eprintln!("Failed to process event: {}", e);
                                                continue;
                                            }
                                        }

                                        // ファイル位置とメタデータを更新
                                        let file_states = watcher.get_file_states();
                                        if let Some(position) = file_states.get(&file_path) {
                                            let path_str = file_path.to_string_lossy().to_string();

                                            // ファイルサイズと更新日時も更新
                                            if let Ok(metadata) = std::fs::metadata(&file_path) {
                                                let file_size = metadata.len();
                                                if let Ok(modified) = metadata.modified() {
                                                    let modified_dt =
                                                        chrono::DateTime::<Utc>::from(modified);
                                                    let _ = db::operations::upsert_log_file(
                                                        conn,
                                                        &path_str,
                                                        file_size,
                                                        modified_dt,
                                                    );
                                                }
                                            }

                                            let _ = db::operations::update_log_file_position(
                                                conn, &path_str, *position,
                                            );
                                        }
                                    }
                                }
                            }
                            Err(e) => {
                                eprintln!("Failed to read logs: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to create log watcher: {}", e);
                    }
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            is_backend_ready,
            get_log_path,
            open_invite_url,
            get_local_users,
            get_instances,
            get_instance_by_id,
            get_instance_screenshots,
            open_screenshot_directory,
            get_database_stats,
            get_instance_players,
            open_user_page
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
