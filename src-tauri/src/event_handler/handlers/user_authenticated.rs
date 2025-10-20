use crate::db::operations;
use crate::event_handler::core::HandlerContext;
use crate::types::ProcessedEvent;
use rusqlite::Connection;

pub fn handle(
    conn: &Connection,
    ctx: &mut HandlerContext,
    timestamp: &str,
    vrchat_user_id: &str,
    display_name: &str,
) -> Result<Option<ProcessedEvent>, rusqlite::Error> {
    // Upsert user
    let user_id = operations::upsert_user(conn, vrchat_user_id, display_name, timestamp)?;

    // Upsert user name history
    operations::upsert_user_name_history(conn, user_id, display_name, timestamp)?;

    // Upsert my_account
    let my_account_id = operations::upsert_my_account(conn, user_id, timestamp)?;

    // Update current state
    ctx.current_my_account_id = Some(my_account_id);
    ctx.current_user_id = Some(user_id);
    ctx.user_ids.insert(vrchat_user_id.to_string(), user_id);

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
