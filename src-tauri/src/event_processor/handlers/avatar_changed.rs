use crate::db::operations;
use crate::event_processor::{processor::ProcessorContext, ProcessedEvent};
use rusqlite::Connection;

pub fn handle(
    conn: &Connection,
    ctx: &mut ProcessorContext,
    timestamp: &str,
    display_name: &str,
    avatar_name: &str,
) -> Result<Option<ProcessedEvent>, rusqlite::Error> {
    let instance_id = match ctx.current_instance_id.as_ref().copied() {
        Some(id) => id,
        None => {
            eprintln!("AvatarChanged but no active instance");
            return Ok(None);
        }
    };

    // Upsert avatar (avatar_id is currently unavailable from logs)
    let avatar_id = operations::upsert_avatar(conn, avatar_name, None, timestamp)?;

    // Check if this is local player or remote player
    let local_display_name = if let Some(uid) = ctx.current_user_id {
        Some(operations::get_user_display_name(conn, uid)?)
    } else {
        None
    };

    let user_id = if Some(display_name) == local_display_name.as_deref() {
        // Local player
        ctx.current_user_id.expect("Local user should be set")
    } else {
        // Remote player
        let placeholder_user_id = format!("unknown_{}", display_name);
        match ctx.user_ids.get(&placeholder_user_id) {
            Some(&uid) => uid,
            None => {
                // Player not yet joined - store as pending
                ctx.pending_avatars
                    .insert(display_name.to_string(), (avatar_id, timestamp.to_string()));
                println!(
                    "Avatar changed before join, storing as pending: {} -> {}",
                    display_name, avatar_name
                );
                return Ok(None);
            }
        }
    };

    // Record avatar history
    operations::record_avatar_history(conn, instance_id, user_id, avatar_id, timestamp)?;

    println!("Avatar changed: {} -> {}", display_name, avatar_name);

    Ok(Some(ProcessedEvent::AvatarChanged {
        instance_id,
        user_id,
        display_name: display_name.to_string(),
        avatar_id,
        avatar_name: avatar_name.to_string(),
        changed_at: timestamp.to_string(),
    }))
}
