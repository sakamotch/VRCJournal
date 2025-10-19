use rusqlite::{Connection, Result};

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
