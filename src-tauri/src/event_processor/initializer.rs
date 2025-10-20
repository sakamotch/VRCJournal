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
    if let Some((my_account_id, user_id)) = operations::get_latest_authenticated_account(conn)? {
        *current_my_account_id = Some(my_account_id);
        *current_user_id = Some(user_id);
        println!(
            "EventProcessor initialized with my_account_id: {}, user_id: {}",
            my_account_id, user_id
        );

        // Find active instance for this account
        if let Some(instance_id) = operations::get_latest_active_instance(conn, my_account_id)? {
            *current_instance_id = Some(instance_id);
            println!("Found active instance: {}", instance_id);

            // Restore users currently in the instance
            let users = operations::get_instance_active_users(conn, instance_id)?;
            for (vrchat_user_id, user_id, instance_user_id) in users {
                user_ids.insert(vrchat_user_id, user_id);
                instance_user_ids.insert(user_id, instance_user_id);
            }

            println!("Restored {} users in current instance", user_ids.len());
        }
    } else {
        println!("No local account found. Waiting for authentication event.");
    }

    Ok(())
}
