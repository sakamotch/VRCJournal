use rusqlite::{Connection, Result};

/// Record a screenshot
pub fn record_screenshot(
    conn: &Connection,
    instance_id: i64,
    file_path: &str,
    taken_at: i64,
) -> Result<i64> {
    conn.execute(
        "INSERT INTO screenshots (instance_id, file_path, taken_at)
         VALUES (?1, ?2, ?3)",
        (instance_id, file_path, taken_at),
    )?;
    Ok(conn.last_insert_rowid())
}
