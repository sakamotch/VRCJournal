use crate::db::operations;
use crate::event_handler::core::HandlerContext;
use crate::types::ProcessedEvent;
use rusqlite::Connection;

pub fn handle(
    conn: &Connection,
    ctx: &mut HandlerContext,
    timestamp: &str,
    display_name: &str,
    _vrchat_user_id: &str,
) -> Result<Option<ProcessedEvent>, rusqlite::Error> {
    let instance_id = match ctx.current_instance_id.as_ref().copied() {
        Some(id) => id,
        None => {
            eprintln!("PlayerJoined but no active instance");
            return Ok(None);
        }
    };

    // For remote players, we don't have their VRChat user_id, so we use display_name as identifier
    // Create a placeholder user_id (we'll use "unknown_<display_name>")
    let placeholder_user_id = format!("unknown_{}", display_name);

    // Check if we already have this user
    let user_id = if let Some(&existing_user_id) = ctx.user_ids.get(&placeholder_user_id) {
        // Update display name if changed
        operations::upsert_user(conn, &placeholder_user_id, display_name, timestamp)?;
        existing_user_id
    } else {
        // Create new user with placeholder ID
        let new_user_id =
            operations::upsert_user(conn, &placeholder_user_id, display_name, timestamp)?;
        ctx.user_ids
            .insert(placeholder_user_id.clone(), new_user_id);
        new_user_id
    };

    // Upsert user name history
    let display_name_history_id =
        operations::upsert_user_name_history(conn, user_id, display_name, timestamp)?;

    // Add user to instance
    let instance_user_id = operations::add_user_to_instance(
        conn,
        instance_id,
        user_id,
        display_name_history_id,
        timestamp,
    )?;

    ctx.instance_user_ids.insert(user_id, instance_user_id);

    // Check if there's a pending avatar for this player
    if let Some((avatar_id, avatar_timestamp)) = ctx.pending_avatars.remove(display_name) {
        operations::record_avatar_history(
            conn,
            instance_id,
            user_id,
            avatar_id,
            &avatar_timestamp,
        )?;
        println!("Player joined: {} with pending avatar", display_name);
    } else {
        println!("Player joined: {}", display_name);
    }

    Ok(Some(ProcessedEvent::UserJoined {
        instance_id,
        instance_user_id,
        user_id,
        display_name: display_name.to_string(),
        joined_at: timestamp.to_string(),
    }))
}
