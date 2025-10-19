use rusqlite::{Connection, OptionalExtension, Result};

/// Create a new instance and return its ID
pub fn create_instance(
    conn: &Connection,
    my_account_id: i64,
    world_id: i64,
    world_name_at_join_id: Option<i64>,
    instance_id: &str,
    started_at: &str,
) -> Result<i64> {
    conn.execute(
        "INSERT INTO instances (my_account_id, world_id, world_name_at_join_id, instance_id, started_at)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        (
            my_account_id,
            world_id,
            world_name_at_join_id,
            instance_id,
            started_at,
        ),
    )?;
    Ok(conn.last_insert_rowid())
}

/// End an instance by setting ended_at and status
/// Only updates status to 'completed' if current status is 'active'
pub fn end_instance(
    conn: &Connection,
    instance_id: i64,
    ended_at: &str,
) -> Result<()> {
    conn.execute(
        "UPDATE instances
         SET ended_at = ?1,
             status = CASE WHEN status = 'active' THEN 'completed' ELSE status END
         WHERE id = ?2",
        (ended_at, instance_id),
    )?;
    Ok(())
}

/// Update instance status
pub fn update_instance_status(
    conn: &Connection,
    instance_id: i64,
    status: &str,
) -> Result<()> {
    conn.execute(
        "UPDATE instances SET status = ?1 WHERE id = ?2",
        (status, instance_id),
    )?;
    Ok(())
}

/// Get the latest active instance for a my_account
pub fn get_latest_active_instance(
    conn: &Connection,
    my_account_id: i64,
) -> Result<Option<i64>> {
    conn.query_row(
        "SELECT id FROM instances
         WHERE my_account_id = ?1 AND ended_at IS NULL
         ORDER BY started_at DESC
         LIMIT 1",
        [my_account_id],
        |row| row.get(0),
    )
    .optional()
}

/// Add a user to an instance and return the instance_user ID
pub fn add_user_to_instance(
    conn: &Connection,
    instance_id: i64,
    user_id: i64,
    display_name_at_join_id: i64,
    joined_at: &str,
) -> Result<i64> {
    conn.execute(
        "INSERT INTO instance_users (instance_id, user_id, display_name_at_join_id, joined_at)
         VALUES (?1, ?2, ?3, ?4)",
        (instance_id, user_id, display_name_at_join_id, joined_at),
    )?;
    Ok(conn.last_insert_rowid())
}

/// Mark a user as having left an instance
pub fn set_user_left_instance(
    conn: &Connection,
    instance_user_id: i64,
    left_at: &str,
) -> Result<()> {
    conn.execute(
        "UPDATE instance_users SET left_at = ?1 WHERE id = ?2",
        (left_at, instance_user_id),
    )?;
    Ok(())
}

/// Get the instance_user ID for a user in a specific instance who hasn't left yet
pub fn get_active_instance_user(
    conn: &Connection,
    instance_id: i64,
    user_id: i64,
) -> Result<Option<i64>> {
    conn.query_row(
        "SELECT id FROM instance_users
         WHERE instance_id = ?1 AND user_id = ?2 AND left_at IS NULL
         ORDER BY joined_at DESC
         LIMIT 1",
        (instance_id, user_id),
        |row| row.get(0),
    )
    .optional()
}

/// Mark all remaining users in an instance as left
pub fn set_all_users_left_instance(
    conn: &Connection,
    instance_id: i64,
    left_at: &str,
) -> Result<()> {
    conn.execute(
        "UPDATE instance_users
         SET left_at = ?1
         WHERE instance_id = ?2 AND left_at IS NULL",
        (left_at, instance_id),
    )?;
    Ok(())
}

/// Get world_id from an instance
pub fn get_instance_world_id(
    conn: &Connection,
    instance_id: i64,
) -> Result<i64> {
    conn.query_row(
        "SELECT world_id FROM instances WHERE id = ?1",
        [instance_id],
        |row| row.get(0),
    )
}
