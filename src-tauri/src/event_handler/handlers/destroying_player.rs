use crate::db::operations;
use crate::event_handler::HandlerContext;
use crate::types::{InstanceStatus, ProcessedEvent};
use rusqlite::Connection;

pub fn handle(
    conn: &Connection,
    ctx: &mut HandlerContext,
    timestamp: &str,
    display_name: &str,
) -> Result<Option<ProcessedEvent>, rusqlite::Error> {
    let instance_id = match *ctx.current_instance_id {
        Some(id) => id,
        None => return Ok(None),
    };

    let local_user_id = match *ctx.current_user_id {
        Some(id) => id,
        None => return Ok(None),
    };

    // Returns None if already cleared by local player leaving
    let user_id = match ctx.display_name_to_user_id.remove(display_name) {
        Some(uid) => uid,
        None => return Ok(None),
    };

    let instance_user_id = match ctx.instance_user_ids.remove(&user_id) {
        Some(iuid) => iuid,
        None => {
            eprintln!(
                "Warning: Player {} (user_id {}) not found in instance_user_ids",
                display_name, user_id
            );
            return Ok(None);
        }
    };

    let is_local_player = user_id == local_user_id;

    if is_local_player {
        // Local player is leaving - end the instance
        operations::set_all_users_left_instance(conn, instance_id, timestamp)?;
        operations::end_instance(conn, instance_id, timestamp)?;

        println!("Local player left, instance {} ended", instance_id);

        // Clear all state
        *ctx.current_instance_id = None;
        ctx.user_ids.clear();
        ctx.instance_user_ids.clear();
        ctx.display_name_to_user_id.clear();
        ctx.pending_avatars.clear();

        Ok(Some(ProcessedEvent::InstanceEnded {
            instance_id,
            ended_at: timestamp.to_string(),
            status: InstanceStatus::Completed,
        }))
    } else {
        // Remote player is leaving
        operations::set_user_left_instance(conn, instance_user_id, timestamp)?;
        println!("Player {} left (destroying)", display_name);

        Ok(Some(ProcessedEvent::UserLeft {
            instance_id,
            instance_user_id,
            left_at: timestamp.to_string(),
        }))
    }
}
