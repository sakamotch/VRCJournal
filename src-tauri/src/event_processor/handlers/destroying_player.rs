use crate::db::{operations, InstanceStatus};
use crate::event_processor::{processor::ProcessorContext, ProcessedEvent};
use rusqlite::Connection;

pub fn handle(
    conn: &Connection,
    ctx: &mut ProcessorContext,
    timestamp: &str,
    display_name: &str,
) -> Result<Option<ProcessedEvent>, rusqlite::Error> {
    let instance_id = match *ctx.current_instance_id {
        Some(id) => id,
        None => return Ok(None),
    };

    let local_user_id = match ctx.current_user_id {
        Some(id) => id,
        None => return Ok(None),
    };

    // Check if this is the local player
    let local_display_name = operations::get_user_display_name(conn, local_user_id)?;
    let is_local_player = display_name == local_display_name;

    if is_local_player {
        // Local player is leaving - end the instance
        // Mark all remaining users as left
        operations::set_all_users_left_instance(conn, instance_id, timestamp)?;

        operations::end_instance(conn, instance_id, timestamp)?;

        println!("Local player left, instance {} ended", instance_id);

        let result = Some(ProcessedEvent::InstanceEnded {
            instance_id,
            ended_at: timestamp.to_string(),
            status: InstanceStatus::Completed,
        });

        *ctx.current_instance_id = None;
        ctx.instance_user_ids.clear();
        ctx.pending_avatars.clear();

        Ok(result)
    } else {
        // Remote player is leaving
        // Find user by display name
        let placeholder_user_id = format!("unknown_{}", display_name);

        if let Some(&user_id) = ctx.user_ids.get(&placeholder_user_id) {
            if let Some(instance_user_id) = ctx.instance_user_ids.remove(&user_id) {
                operations::set_user_left_instance(conn, instance_user_id, timestamp)?;
                println!("Player {} left (destroying)", display_name);

                return Ok(Some(ProcessedEvent::UserLeft {
                    instance_id,
                    instance_user_id,
                    left_at: timestamp.to_string(),
                }));
            }
        }

        Ok(None)
    }
}
