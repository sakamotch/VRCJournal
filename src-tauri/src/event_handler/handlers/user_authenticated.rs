use crate::db::operations;
use crate::event_handler::HandlerContext;
use crate::types::VRChatEvent;
use chrono::{DateTime, Utc};
use rusqlite::Connection;

pub fn handle(
    conn: &Connection,
    ctx: &mut HandlerContext,
    timestamp: DateTime<Utc>,
    vrchat_user_id: &str,
    display_name: &str,
) -> Result<Option<VRChatEvent>, rusqlite::Error> {
    let timestamp_ms = timestamp.timestamp_millis();

    // Upsert user
    let user_id = operations::upsert_user(conn, vrchat_user_id, display_name, timestamp_ms)?;

    // Upsert user name history
    operations::upsert_user_name_history(conn, user_id, display_name, timestamp_ms)?;

    // Upsert my_account
    let my_account_id = operations::upsert_my_account(conn, user_id, timestamp_ms)?;

    // Update current state
    *ctx.current_my_account_id = Some(my_account_id);
    *ctx.current_user_id = Some(user_id);
    ctx.user_ids.insert(vrchat_user_id.to_string(), user_id);

    println!(
        "Local player authenticated: {} (my_account_id: {}, user_id: {})",
        display_name, my_account_id, user_id
    );

    Ok(Some(VRChatEvent::UserAuthenticated {
        my_account_id,
        user_id,
        display_name: display_name.to_string(),
        vrchat_user_id: vrchat_user_id.to_string(),
    }))
}
