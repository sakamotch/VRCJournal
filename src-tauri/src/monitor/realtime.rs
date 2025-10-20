use crate::{db, handler::EventHandler, reader::LogReader};
use std::time::Duration;
use tauri::AppHandle;

use super::process_events;

/// リアルタイム処理ループ（ブロッキング）
///
/// 定期的にログファイルをポーリングし、
/// 新しいイベントを検出して処理する
pub(super) fn run_loop(
    mut reader: LogReader,
    mut handler: EventHandler,
    database: db::Database,
    app_handle: AppHandle,
) {
    loop {
        std::thread::sleep(Duration::from_millis(1000));

        // 新しいイベントをポーリング
        let events = match reader.poll_new_events() {
            Ok(events) if !events.is_empty() => events,
            Ok(_) => continue, // イベントなし
            Err(e) => {
                eprintln!("Failed to poll events: {}", e);
                continue;
            }
        };

        // イベント処理とフロントエンドへの送信
        let conn = database.connection();
        process_events(conn, &mut handler, events, Some(&app_handle));

        // ファイル位置を保存
        reader.save_file_states(conn);
    }
}
