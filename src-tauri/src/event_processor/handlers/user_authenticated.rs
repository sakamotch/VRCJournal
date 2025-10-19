use crate::db::operations;
use crate::event_processor::ProcessedEvent;
use rusqlite::Connection;
use std::collections::HashMap;

pub fn handle(
    conn: &Connection,
    timestamp: &str,
    vrchat_user_id: &str,
    display_name: &str,
    current_my_account_id: &mut Option<i64>,
    current_user_id: &mut Option<i64>,
    user_ids: &mut HashMap<String, i64>,
) -> Result<Option<ProcessedEvent>, rusqlite::Error> {
    // Upsert user
    let user_id = operations::upsert_user(conn, vrchat_user_id, display_name, timestamp)?;

    // Upsert user name history
    operations::upsert_user_name_history(conn, user_id, display_name, timestamp)?;

    // Upsert my_account
    let my_account_id = operations::upsert_my_account(conn, user_id, timestamp)?;

    // Update current state
    *current_my_account_id = Some(my_account_id);
    *current_user_id = Some(user_id);
    user_ids.insert(vrchat_user_id.to_string(), user_id);

    println!(
        "Local player authenticated: {} (my_account_id: {}, user_id: {})",
        display_name, my_account_id, user_id
    );

    Ok(Some(ProcessedEvent::UserAuthenticated {
        my_account_id,
        user_id,
        display_name: display_name.to_string(),
        vrchat_user_id: vrchat_user_id.to_string(),
    }))
}
