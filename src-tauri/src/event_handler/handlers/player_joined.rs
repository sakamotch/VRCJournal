use crate::db::operations;
use crate::event_handler::HandlerContext;
use crate::types::VRChatEvent;
use chrono::{DateTime, Utc};
use rusqlite::Connection;

pub fn handle(
    conn: &Connection,
    ctx: &mut HandlerContext,
    timestamp: DateTime<Utc>,
    display_name: &str,
    vrchat_user_id: &str,
) -> Result<Option<VRChatEvent>, rusqlite::Error> {
    let timestamp_ms = timestamp.timestamp_millis();

    let instance_id = match *ctx.current_instance_id {
        Some(id) => id,
        None => {
            eprintln!("PlayerJoined but no active instance");
            return Ok(None);
        }
    };

    // Check for duplicate join
    if ctx.user_ids.contains_key(vrchat_user_id) {
        eprintln!(
            "Warning: User {} ({}) already joined this instance, ignoring duplicate join event",
            display_name, vrchat_user_id
        );
        return Ok(None);
    }

    // Upsert user in database
    let user_id = operations::upsert_user(conn, vrchat_user_id, display_name, timestamp_ms)?;

    // Upsert user name history
    let display_name_history_id =
        operations::upsert_user_name_history(conn, user_id, display_name, timestamp_ms)?;

    // Add user to instance
    let instance_user_id = operations::add_user_to_instance(
        conn,
        instance_id,
        user_id,
        display_name_history_id,
        timestamp_ms,
    )?;

    // Update all context mappings together
    ctx.user_ids.insert(vrchat_user_id.to_string(), user_id);
    ctx.instance_user_ids.insert(user_id, instance_user_id);
    ctx.display_name_to_user_id
        .insert(display_name.to_string(), user_id);

    // Check if there's a pending avatar for this player
    let (initial_avatar_id, initial_avatar_name) =
        if let Some((avatar_id, avatar_timestamp)) = ctx.pending_avatars.remove(display_name) {
            operations::record_avatar_history(
                conn,
                instance_id,
                user_id,
                avatar_id,
                avatar_timestamp.timestamp_millis(),
            )?;

            let avatar_name = operations::get_avatar_name(conn, avatar_id)?;
            println!(
                "Player joined: {} with pending avatar: {}",
                display_name, avatar_name
            );
            (Some(avatar_id), Some(avatar_name))
        } else {
            println!("Player joined: {}", display_name);
            (None, None)
        };

    Ok(Some(VRChatEvent::UserJoined {
        instance_id,
        instance_user_id,
        user_id,
        display_name: display_name.to_string(),
        joined_at: timestamp_ms,
        initial_avatar_id,
        initial_avatar_name,
    }))
}
