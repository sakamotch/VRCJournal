use crate::db::operations;
use rusqlite::Connection;
use std::collections::HashMap;

/// Restore EventProcessor state from the database
///
/// This is necessary because log files are read incrementally from the last processed position.
/// When the application restarts, we need to restore:
/// - The currently active local account (my_account)
/// - The currently active instance
/// - Users currently in the instance
///
/// Without this context, we wouldn't know which account/instance new events belong to
/// when processing log entries from the middle of a file.
///
/// # Example scenario:
/// 1. Previous session: Processed log file up to byte 5000, user was in instance 123
/// 2. App restarts: Resume reading from byte 5000
/// 3. First event: "PlayerJoined: Alice" - which instance did Alice join?
/// 4. Answer: Instance 123 (restored from DB)
pub fn restore_previous_state(
    conn: &Connection,
    current_my_account_id: &mut Option<i64>,
    current_user_id: &mut Option<i64>,
    current_instance_id: &mut Option<i64>,
    user_ids: &mut HashMap<String, i64>,
    instance_user_ids: &mut HashMap<i64, i64>,
) -> Result<(), rusqlite::Error> {
    // Restore the most recently authenticated local account
    let my_account_result = conn.query_row(
        "SELECT ma.id, ma.user_id
         FROM my_accounts ma
         ORDER BY ma.last_authenticated_at DESC
         LIMIT 1",
        [],
        |row| Ok((row.get::<_, i64>(0)?, row.get::<_, i64>(1)?)),
    );

    match my_account_result {
        Ok((my_account_id, user_id)) => {
            *current_my_account_id = Some(my_account_id);
            *current_user_id = Some(user_id);
            println!(
                "EventProcessor initialized with my_account_id: {}, user_id: {}",
                my_account_id, user_id
            );

            // Find active instance for this account
            if let Some(instance_id) =
                operations::get_latest_active_instance(conn, my_account_id)?
            {
                *current_instance_id = Some(instance_id);
                println!("Found active instance: {}", instance_id);

                // Restore users currently in the instance
                let mut stmt = conn.prepare(
                    "SELECT u.user_id, iu.user_id, iu.id
                     FROM instance_users iu
                     JOIN users u ON iu.user_id = u.id
                     WHERE iu.instance_id = ?1 AND iu.left_at IS NULL",
                )?;

                let rows = stmt.query_map([instance_id], |row| {
                    Ok((
                        row.get::<_, String>(0)?, // vrchat user_id
                        row.get::<_, i64>(1)?,    // users.id
                        row.get::<_, i64>(2)?,    // instance_users.id
                    ))
                })?;

                for row in rows {
                    let (vrchat_user_id, user_id, instance_user_id) = row?;
                    user_ids.insert(vrchat_user_id, user_id);
                    instance_user_ids.insert(user_id, instance_user_id);
                }

                println!("Restored {} users in current instance", user_ids.len());
            }
        }
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            println!("No local account found. Waiting for authentication event.");
        }
        Err(e) => return Err(e),
    }

    Ok(())
}
