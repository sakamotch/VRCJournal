use rusqlite::{Connection, OptionalExtension, Result};

/// Upsert world and return world ID
pub fn upsert_world(
    conn: &Connection,
    world_id: &str,
    world_name: &str,
    timestamp: &str,
) -> Result<i64> {
    conn.execute(
        "INSERT INTO worlds (world_id, world_name, first_seen_at, last_seen_at)
         VALUES (?1, ?2, ?3, ?3)
         ON CONFLICT(world_id) DO UPDATE SET
           world_name = excluded.world_name,
           last_seen_at = excluded.last_seen_at",
        [world_id, world_name, timestamp],
    )?;

    let id = conn.query_row(
        "SELECT id FROM worlds WHERE world_id = ?1",
        [world_id],
        |row| row.get(0),
    )?;

    Ok(id)
}

/// Update world name
pub fn update_world_name(conn: &Connection, world_id: i64, world_name: &str) -> Result<()> {
    conn.execute(
        "UPDATE worlds SET world_name = ?1 WHERE id = ?2",
        (world_name, world_id),
    )?;
    Ok(())
}

/// Upsert world name history
pub fn upsert_world_name_history(
    conn: &Connection,
    world_id: i64,
    world_name: &str,
    timestamp: &str,
) -> Result<i64> {
    // Check if there's an existing entry with the same world name
    let existing: Option<i64> = conn
        .query_row(
            "SELECT id FROM world_name_history
             WHERE world_id = ?1 AND world_name = ?2
             ORDER BY first_seen_at DESC
             LIMIT 1",
            [world_id.to_string(), world_name.to_string()],
            |row| row.get(0),
        )
        .optional()?;

    if let Some(id) = existing {
        // Update last_seen_at
        conn.execute(
            "UPDATE world_name_history SET last_seen_at = ?1 WHERE id = ?2",
            [timestamp, &id.to_string()],
        )?;
        Ok(id)
    } else {
        // Insert new entry
        conn.execute(
            "INSERT INTO world_name_history (world_id, world_name, first_seen_at, last_seen_at)
             VALUES (?1, ?2, ?3, ?3)",
            [&world_id.to_string(), world_name, timestamp],
        )?;
        Ok(conn.last_insert_rowid())
    }
}
