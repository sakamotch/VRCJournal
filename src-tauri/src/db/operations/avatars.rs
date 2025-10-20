use rusqlite::{Connection, OptionalExtension, Result};

/// Upsert avatar by name
pub fn upsert_avatar(
    conn: &Connection,
    avatar_name: &str,
    avatar_id: Option<&str>,
    timestamp: &str,
) -> Result<i64> {
    // Check if avatar exists by name
    let existing: Option<i64> = conn
        .query_row(
            "SELECT id FROM avatars WHERE avatar_name = ?1",
            [avatar_name],
            |row| row.get(0),
        )
        .optional()?;

    if let Some(id) = existing {
        // Update last_seen_at and avatar_id if provided
        if let Some(aid) = avatar_id {
            conn.execute(
                "UPDATE avatars SET last_seen_at = ?1, avatar_id = ?2 WHERE id = ?3",
                (timestamp, aid, id),
            )?;
        } else {
            conn.execute(
                "UPDATE avatars SET last_seen_at = ?1 WHERE id = ?2",
                (timestamp, id),
            )?;
        }
        Ok(id)
    } else {
        // Insert new avatar
        conn.execute(
            "INSERT INTO avatars (avatar_id, avatar_name, first_seen_at, last_seen_at)
             VALUES (?1, ?2, ?3, ?3)",
            (avatar_id, avatar_name, timestamp),
        )?;
        Ok(conn.last_insert_rowid())
    }
}

/// Record avatar usage
pub fn record_avatar_history(
    conn: &Connection,
    instance_id: i64,
    user_id: i64,
    avatar_id: i64,
    changed_at: &str,
) -> Result<()> {
    conn.execute(
        "INSERT INTO avatar_history (instance_id, user_id, avatar_id, changed_at)
         VALUES (?1, ?2, ?3, ?4)",
        (instance_id, user_id, avatar_id, changed_at),
    )?;
    Ok(())
}
