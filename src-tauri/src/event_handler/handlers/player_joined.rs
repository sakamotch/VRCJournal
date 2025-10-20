use crate::db::operations;
use crate::event_handler::core::HandlerContext;
use crate::types::ProcessedEvent;
use rusqlite::Connection;

pub fn handle(
    conn: &Connection,
    ctx: &mut HandlerContext,
    timestamp: &str,
    display_name: &str,
    vrchat_user_id: &str,
) -> Result<Option<ProcessedEvent>, rusqlite::Error> {
    let instance_id = match ctx.current_instance_id.as_ref().copied() {
        Some(id) => id,
        None => {
            eprintln!("PlayerJoined but no active instance");
            return Ok(None);
        }
    };

    let user_id = if let Some(&existing_user_id) = ctx.user_ids.get(vrchat_user_id) {
        operations::upsert_user(conn, vrchat_user_id, display_name, timestamp)?;
        existing_user_id
    } else {
        let new_user_id = operations::upsert_user(conn, vrchat_user_id, display_name, timestamp)?;
        ctx.user_ids.insert(vrchat_user_id.to_string(), new_user_id);
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
