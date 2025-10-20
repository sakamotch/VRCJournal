use crate::db::operations;
use crate::event_handler::HandlerContext;
use crate::types::ProcessedEvent;
use rusqlite::Connection;

pub fn handle(
    conn: &Connection,
    ctx: &mut HandlerContext,
    timestamp: &str,
    display_name: &str,
    avatar_name: &str,
) -> Result<Option<ProcessedEvent>, rusqlite::Error> {
    let instance_id = match *ctx.current_instance_id {
        Some(id) => id,
        None => {
            eprintln!("AvatarChanged but no active instance");
            return Ok(None);
        }
    };

    // Upsert avatar (avatar_id is currently unavailable from logs)
    let avatar_id = operations::upsert_avatar(conn, avatar_name, None, timestamp)?;

    // Find user by display name
    let user_id = match ctx.display_name_to_user_id.get(display_name) {
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
