use chrono::{DateTime, Utc};
use rusqlite::{Connection, Result};

/// スクリーンショットを記録
pub fn record_screenshot(
    conn: &Connection,
    instance_id: i64,
    file_path: &str,
    taken_at: &DateTime<Utc>,
) -> Result<i64> {
    conn.execute(
        "INSERT INTO screenshots (instance_id, file_path, taken_at)
         VALUES (?1, ?2, ?3)",
        (instance_id, file_path, taken_at.to_rfc3339()),
    )?;
    Ok(conn.last_insert_rowid())
}

/// インスタンスのスクリーンショット一覧を取得
pub fn get_instance_screenshots(
    conn: &Connection,
    instance_id: i64,
) -> Result<Vec<(i64, String, String)>> {
    let mut stmt = conn.prepare(
        "SELECT id, file_path, taken_at
         FROM screenshots
         WHERE instance_id = ?1
         ORDER BY taken_at ASC",
    )?;

    let screenshots = stmt
        .query_map([instance_id], |row| {
            Ok((
                row.get(0)?, // id
                row.get(1)?, // file_path
                row.get(2)?, // taken_at
            ))
        })?
        .collect::<Result<Vec<_>>>()?;

    Ok(screenshots)
}
