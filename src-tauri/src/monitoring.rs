use crate::{db, event_processor::EventProcessor, log_watcher::LogWatcher, types::LogEvent};
use std::time::Duration;
use tauri::{AppHandle, Emitter};

/// ログ監視とイベント処理を統合して実行
pub fn start(database: db::Database, app_handle: AppHandle) {
    // LogWatcherとEventProcessorを作成
    let mut watcher = match LogWatcher::new() {
        Ok(w) => w,
        Err(e) => {
            eprintln!("Failed to create log watcher: {}", e);
            return;
        }
    };

    let mut processor = EventProcessor::new();

    // 前回終了時点の状態を復元
    {
        let conn = database.connection();

        if let Err(e) = processor.restore_previous_state(conn) {
            eprintln!("Failed to restore EventProcessor state: {}", e);
        }

        if let Err(e) = watcher.restore_file_positions(conn) {
            eprintln!("Failed to restore file positions: {}", e);
            return;
        }
    }

    // バックログ処理
    {
        let events = match watcher.read_backlog_events() {
            Ok(events) => events,
            Err(e) => {
                eprintln!("Failed to read backlog events: {}", e);
                Vec::new()
            }
        };

        let conn = database.connection();
        let count = process_event_batch(conn, &mut processor, events, None);
        watcher.save_file_states(conn);
        println!("Backlog processing completed: {} events", count);
    }

    // バックエンド準備完了を通知
    if let Err(e) = app_handle.emit("backend-ready", ()) {
        eprintln!("Failed to emit backend-ready event: {}", e);
        return;
    }

    // リアルタイム処理ループ
    loop {
        std::thread::sleep(Duration::from_millis(1000));

        // 新しいイベントをポーリング
        let events = match watcher.poll_new_events() {
            Ok(events) if !events.is_empty() => events,
            Ok(_) => continue,
            Err(e) => {
                eprintln!("Failed to poll events: {}", e);
                continue;
            }
        };

        // イベント処理とDB保存
        let conn = database.connection();
        process_event_batch(conn, &mut processor, events, Some(&app_handle));
        watcher.save_file_states(conn);
    }
}

/// イベント一覧を処理する統一ヘルパー
///
/// - バックログ処理: app_handle = None → イベント送信しない
/// - リアルタイム処理: app_handle = Some → イベント送信する
fn process_event_batch(
    conn: &rusqlite::Connection,
    processor: &mut EventProcessor,
    events: Vec<LogEvent>,
    app_handle: Option<&AppHandle>,
) -> usize {
    let mut count = 0;

    for event in events {
        match processor.process_event(conn, event) {
            Ok(Some(processed_event)) => {
                // フロントエンドに送信（リアルタイムのみ）
                if let Some(handle) = app_handle {
                    if let Err(e) = handle.emit("log-event", &processed_event) {
                        eprintln!("Failed to emit event: {}", e);
                    }
                }
                count += 1;
            }
            Ok(None) => {
                // 通知不要なイベント
                count += 1;
            }
            Err(e) => {
                eprintln!("Failed to process event: {}", e);
            }
        }
    }

    count
}
