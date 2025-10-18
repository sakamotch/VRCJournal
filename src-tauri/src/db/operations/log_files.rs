use chrono::{DateTime, Utc};
use rusqlite::{Connection, OptionalExtension, Result};

/// ログファイルの情報を登録または更新
pub fn upsert_log_file(
    conn: &Connection,
    file_path: &str,
    file_size: u64,
    last_modified_at: DateTime<Utc>,
) -> Result<i64> {
    conn.execute(
        "INSERT INTO log_files (file_path, file_size, last_processed_position, last_modified_at, updated_at)
         VALUES (?1, ?2, 0, ?3, CURRENT_TIMESTAMP)
         ON CONFLICT(file_path) DO UPDATE SET
             file_size = ?2,
             last_modified_at = ?3,
             updated_at = CURRENT_TIMESTAMP",
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
         SET last_processed_position = ?1, updated_at = CURRENT_TIMESTAMP
         WHERE file_path = ?2",
        rusqlite::params![position as i64, file_path],
    )?;
    Ok(())
}

/// ログファイルの処理位置を取得
pub fn get_log_file_position(conn: &Connection, file_path: &str) -> Result<Option<u64>> {
    let position: Option<i64> = conn
        .query_row(
            "SELECT last_processed_position FROM log_files WHERE file_path = ?1",
            [file_path],
            |row| row.get(0),
        )
        .optional()?;

    Ok(position.map(|p| p as u64))
}

/// 全てのログファイル情報を取得
pub fn get_all_log_files(conn: &Connection) -> Result<Vec<(String, u64, u64, DateTime<Utc>)>> {
    let mut stmt = conn.prepare(
        "SELECT file_path, file_size, last_processed_position, last_modified_at
         FROM log_files
         ORDER BY last_modified_at DESC",
    )?;

    let rows = stmt.query_map([], |row| {
        let file_path: String = row.get(0)?;
        let file_size: i64 = row.get(1)?;
        let position: i64 = row.get(2)?;
        let modified_str: String = row.get(3)?;
        let modified_at = DateTime::parse_from_rfc3339(&modified_str)
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now());

        Ok((file_path, file_size as u64, position as u64, modified_at))
    })?;

    rows.collect()
}

/// ログファイルが完全に処理済みかチェック
pub fn is_log_file_fully_processed(
    conn: &Connection,
    file_path: &str,
    current_file_size: u64,
) -> Result<bool> {
    let result: Option<(i64, i64)> = conn
        .query_row(
            "SELECT file_size, last_processed_position FROM log_files WHERE file_path = ?1",
            [file_path],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .optional()?;

    match result {
        Some((stored_size, position)) => {
            Ok(stored_size as u64 == current_file_size && position as u64 >= current_file_size)
        }
        None => Ok(false),
    }
}
