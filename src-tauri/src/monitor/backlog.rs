use crate::{db, handler::EventHandler, reader::LogReader};

use super::process_events;

/// バックログイベントを読み込んで処理
///
/// アプリケーション起動時に、前回終了後から現在までに
/// 蓄積されたログイベントを一括処理する
pub(super) fn process(
    reader: &mut LogReader,
    handler: &mut EventHandler,
    database: &db::Database,
) -> Result<usize, String> {
    // バックログイベントを読み込み
    let events = reader
        .read_backlog_events()
        .map_err(|e| format!("Failed to read backlog: {}", e))?;

    if events.is_empty() {
        return Ok(0);
    }

    // バッチ処理（フロントエンドには送信しない）
    let conn = database.connection();
    let count = process_events(conn, handler, events, None);

    // ファイル位置を保存
    reader.save_file_states(conn);

    Ok(count)
}
