use rusqlite::{Connection, OptionalExtension, Result};

/// Upsert user and return user ID
pub fn upsert_user(
    conn: &Connection,
    user_id: &str,
    display_name: &str,
    timestamp: &str,
) -> Result<i64> {
    conn.execute(
        "INSERT INTO users (user_id, display_name, first_seen_at, last_seen_at)
         VALUES (?1, ?2, ?3, ?3)
         ON CONFLICT(user_id) DO UPDATE SET
           display_name = excluded.display_name,
           last_seen_at = excluded.last_seen_at",
        [user_id, display_name, timestamp],
    )?;

    let id = conn.query_row(
        "SELECT id FROM users WHERE user_id = ?1",
        [user_id],
        |row| row.get(0),
    )?;

    Ok(id)
}

/// Get user display name
pub fn get_user_display_name(conn: &Connection, id: i64) -> Result<String> {
    conn.query_row(
        "SELECT display_name FROM users WHERE id = ?1",
        [id],
        |row| row.get(0),
    )
}

/// Upsert user name history
pub fn upsert_user_name_history(
    conn: &Connection,
    user_id: i64,
    display_name: &str,
    timestamp: &str,
) -> Result<i64> {
    // Check if there's an existing entry with the same display name
    let existing: Option<i64> = conn
        .query_row(
            "SELECT id FROM user_name_history
             WHERE user_id = ?1 AND display_name = ?2
             ORDER BY first_seen_at DESC
             LIMIT 1",
            [user_id.to_string(), display_name.to_string()],
            |row| row.get(0),
        )
        .optional()?;

    if let Some(id) = existing {
        // Update last_seen_at
        conn.execute(
            "UPDATE user_name_history SET last_seen_at = ?1 WHERE id = ?2",
            [timestamp, &id.to_string()],
        )?;
        Ok(id)
    } else {
        // Insert new entry
        conn.execute(
            "INSERT INTO user_name_history (user_id, display_name, first_seen_at, last_seen_at)
             VALUES (?1, ?2, ?3, ?3)",
            [&user_id.to_string(), display_name, timestamp],
        )?;
        Ok(conn.last_insert_rowid())
    }
}
