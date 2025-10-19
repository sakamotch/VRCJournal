use crate::app_state::AppState;
use crate::{db, event_processor::EventProcessor, log_watcher};
use std::sync::{Arc, Mutex};
use tauri::{App, AppHandle, Emitter, Manager};

/// Tauriアプリケーションのセットアップ処理
pub fn setup_app(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    // データベース初期化
    let app_data_dir = app.path().app_data_dir()?;
    std::fs::create_dir_all(&app_data_dir)?;
    let db_path = app_data_dir.join("vrcjournal.db");

    let database = db::Database::open(db_path)?;
    database.migrate()?;

    // AppState 作成
    let app_state = AppState {
        db: Arc::new(Mutex::new(database)),
        event_processor: Arc::new(Mutex::new(EventProcessor::new())),
    };

    app.manage(app_state.clone());

    // バックグラウンドでログ監視を開始
    let app_handle = app.handle().clone();
    std::thread::spawn(move || {
        start_log_watcher(app_state, app_handle);
    });

    Ok(())
}

/// ログ監視スレッドの実行
fn start_log_watcher(app_state: AppState, app_handle: AppHandle) {
    use std::collections::HashMap;

    match log_watcher::LogWatcher::new() {
        Ok(mut watcher) => {
            // データベースから処理済みファイル位置を取得
            let file_positions = {
                let db = app_state.db.lock().unwrap();
                let conn = db.connection();

                match db::operations::get_all_log_files(conn) {
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
                }
            };

            // EventProcessorをデータベースから初期化
            {
                let db = app_state.db.lock().unwrap();
                let conn = db.connection();
                let mut processor = app_state.event_processor.lock().unwrap();
                if let Err(e) = processor.initialize_from_db(conn) {
                    eprintln!("Failed to initialize EventProcessor from database: {}", e);
                }
            }

            // 全てのログファイルを読み込み
            match watcher.read_all_logs(file_positions) {
                Ok(events) => {
                    let events_count = process_initial_events(&app_state, &mut watcher, events);

                    println!("Initial log processing completed: {} events", events_count);

                    // ファイル監視を開始
                    if let Err(e) = watcher.start_watching() {
                        eprintln!("Failed to start watching: {}", e);
                        return;
                    }

                    // バックグラウンドでイベントを処理
                    process_log_events(app_state, app_handle, watcher);
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
}

/// 初期イベント処理
fn process_initial_events(
    app_state: &AppState,
    watcher: &mut log_watcher::LogWatcher,
    events: Vec<(std::path::PathBuf, crate::parser::LogEvent)>,
) -> usize {
    use chrono::Utc;

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
                let modified_dt = chrono::DateTime::<Utc>::from(modified);
                let _ = db::operations::upsert_log_file(conn, &path_str, file_size, modified_dt);
                let _ = db::operations::update_log_file_position(conn, &path_str, *position);
            }
        }
    }

    events_count
}

/// リアルタイムイベント処理ループ
fn process_log_events(
    app_state: AppState,
    app_handle: AppHandle,
    watcher: log_watcher::LogWatcher,
) {
    use chrono::Utc;

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
                    if let Err(e) = app_handle.emit("log-event", &processed_event) {
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
                        let modified_dt = chrono::DateTime::<Utc>::from(modified);
                        let _ =
                            db::operations::upsert_log_file(conn, &path_str, file_size, modified_dt);
                    }
                }

                let _ = db::operations::update_log_file_position(conn, &path_str, *position);
            }
        }
    }
}
