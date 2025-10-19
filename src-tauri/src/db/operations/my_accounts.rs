use rusqlite::{Connection, OptionalExtension, Result};

/// Upsert a local account and return the my_account ID
pub fn upsert_my_account(
    conn: &Connection,
    user_id: i64,
    timestamp: &str,
) -> Result<i64> {
    conn.execute(
        "INSERT INTO my_accounts (user_id, first_authenticated_at, last_authenticated_at)
         VALUES (?1, ?2, ?2)
         ON CONFLICT(user_id) DO UPDATE SET
           last_authenticated_at = excluded.last_authenticated_at",
        [&user_id.to_string(), timestamp],
    )?;

    let id = conn.query_row(
        "SELECT id FROM my_accounts WHERE user_id = ?1",
        [user_id],
        |row| row.get(0),
    )?;

    Ok(id)
}

/// Get my_account ID by user ID
pub fn get_my_account_id(conn: &Connection, user_id: i64) -> Result<Option<i64>> {
    conn.query_row(
        "SELECT id FROM my_accounts WHERE user_id = ?1",
        [user_id],
        |row| row.get(0),
    )
    .optional()
}

/// Get all local account IDs
pub fn get_all_my_account_ids(conn: &Connection) -> Result<Vec<i64>> {
    let mut stmt = conn.prepare("SELECT id FROM my_accounts")?;
    let rows = stmt.query_map([], |row| row.get(0))?;
    rows.collect()
}
