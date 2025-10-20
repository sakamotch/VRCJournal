use crate::db::operations;
use crate::event_handler::HandlerContext;
use crate::types::{InstanceStatus, ProcessedEvent};
use rusqlite::Connection;

pub fn handle(
    conn: &Connection,
    ctx: &mut HandlerContext,
    timestamp: &str,
    world_id: &str,
    instance_id: &str,
) -> Result<Option<ProcessedEvent>, rusqlite::Error> {
    let my_account_id = match *ctx.current_my_account_id {
        Some(id) => id,
        None => {
            eprintln!("Cannot join world: no local account authenticated");
            return Ok(None);
        }
    };

    // End previous instance if exists (mark as interrupted)
    if let Some(prev_instance_id) = *ctx.current_instance_id {
        operations::update_instance_status(conn, prev_instance_id, InstanceStatus::Interrupted)?;
        println!(
            "Previous instance {} marked as interrupted",
            prev_instance_id
        );
    }

    // Clear state
    ctx.user_ids.clear();
    ctx.instance_user_ids.clear();
    ctx.display_name_to_user_id.clear();
    ctx.pending_avatars.clear();

    // Upsert world (without world name yet)
    let world_db_id = operations::upsert_world(conn, world_id, timestamp)?;

    // Create new instance (world_name_at_join_id will be set later in entering_room)
    let new_instance_id =
        operations::create_instance(conn, my_account_id, world_db_id, instance_id, timestamp)?;

    *ctx.current_instance_id = Some(new_instance_id);

    println!(
        "Created new instance: {} in world: {}",
        new_instance_id, world_id
    );

    Ok(Some(ProcessedEvent::InstanceCreated {
        instance_id: new_instance_id,
        my_account_id,
        world_id: world_id.to_string(),
        vrchat_instance_id: instance_id.to_string(),
        started_at: timestamp.to_string(),
        status: InstanceStatus::Active,
    }))
}
