use rusqlite::{Connection, Result};

/// Upsert a local account and return the my_account ID
pub fn upsert_my_account(conn: &Connection, user_id: i64, timestamp: &str) -> Result<i64> {
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

/// Get the most recently authenticated local account
///
/// Returns (my_account_id, user_id)
pub fn get_latest_authenticated_account(conn: &Connection) -> Result<Option<(i64, i64)>> {
    let result = conn.query_row(
        "SELECT ma.id, ma.user_id
         FROM my_accounts ma
         ORDER BY ma.last_authenticated_at DESC
         LIMIT 1",
        [],
        |row| Ok((row.get::<_, i64>(0)?, row.get::<_, i64>(1)?)),
    );

    match result {
        Ok(data) => Ok(Some(data)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e),
    }
}
