use chrono::{DateTime, Utc};
use rusqlite::{Connection, Result};

/// ログファイルの情報を登録または更新
pub fn upsert_log_file(
    conn: &Connection,
    file_path: &str,
    file_size: u64,
    last_modified_at: DateTime<Utc>,
) -> Result<i64> {
    conn.execute(
        "INSERT INTO log_files (file_path, file_size, last_read_position, last_modified_at, last_processed_at)
         VALUES (?1, ?2, 0, ?3, CURRENT_TIMESTAMP)
         ON CONFLICT(file_path) DO UPDATE SET
             file_size = ?2,
             last_modified_at = ?3,
             last_processed_at = CURRENT_TIMESTAMP",
        rusqlite::params![file_path, file_size as i64, last_modified_at.to_rfc3339()],
    )?;

    let id = conn.query_row(
        "SELECT id FROM log_files WHERE file_path = ?1",
        [file_path],
        |row| row.get(0),
    )?;

    Ok(id)
}

/// ログファイルの処理位置を更新
pub fn update_log_file_position(conn: &Connection, file_path: &str, position: u64) -> Result<()> {
    conn.execute(
        "UPDATE log_files
         SET last_read_position = ?1, last_processed_at = CURRENT_TIMESTAMP
         WHERE file_path = ?2",
        rusqlite::params![position as i64, file_path],
    )?;
    Ok(())
}

/// 特定のログファイルの読み込み位置を取得
/// 戻り値: Some(position) または None（DBに未登録）
pub fn get_log_file_position(conn: &Connection, file_path: &str) -> Result<Option<u64>> {
    let result = conn.query_row(
        "SELECT last_read_position FROM log_files WHERE file_path = ?1",
        [file_path],
        |row| {
            let position: i64 = row.get(0)?;
            Ok(position as u64)
        },
    );

    match result {
        Ok(position) => Ok(Some(position)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e),
    }
}
