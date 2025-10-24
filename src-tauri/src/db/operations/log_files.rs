use chrono::{DateTime, Utc};
use rusqlite::{Connection, Result};

/// Register or update log file information
pub fn upsert_log_file(
    conn: &Connection,
    file_path: &str,
    file_size: u64,
    last_modified_at: DateTime<Utc>,
) -> Result<i64> {
    let timestamp = last_modified_at.timestamp_millis();
    let now = Utc::now().timestamp_millis();

    conn.execute(
        "INSERT INTO log_files (file_path, file_size, last_read_position, last_modified_at, last_processed_at)
         VALUES (?1, ?2, 0, ?3, ?4)
         ON CONFLICT(file_path) DO UPDATE SET
             file_size = ?2,
             last_modified_at = ?3,
             last_processed_at = ?4",
        rusqlite::params![file_path, file_size as i64, timestamp, now],
    )?;

    let id = conn.query_row(
        "SELECT id FROM log_files WHERE file_path = ?1",
        (file_path,),
        |row| row.get(0),
    )?;

    Ok(id)
}

/// Update log file read position
pub fn update_log_file_position(conn: &Connection, file_path: &str, position: u64) -> Result<()> {
    let now = Utc::now().timestamp_millis();

    conn.execute(
        "UPDATE log_files
         SET last_read_position = ?1, last_processed_at = ?2
         WHERE file_path = ?3",
        rusqlite::params![position as i64, now, file_path],
    )?;
    Ok(())
}

/// Get log file read position (returns None if not registered)
pub fn get_log_file_position(conn: &Connection, file_path: &str) -> Result<Option<u64>> {
    let result = conn.query_row(
        "SELECT last_read_position FROM log_files WHERE file_path = ?1",
        (file_path,),
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
