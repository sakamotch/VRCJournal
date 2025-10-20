use crate::db::operations;
use crate::event_processor::processor::ProcessorContext;
use crate::types::{InstanceStatus, ProcessedEvent};
use rusqlite::Connection;

pub fn handle(
    conn: &Connection,
    ctx: &mut ProcessorContext,
    timestamp: &str,
    world_id: &str,
    world_name: &str,
    instance_id: &str,
) -> Result<Option<ProcessedEvent>, rusqlite::Error> {
    let my_account_id = match ctx.current_my_account_id {
        Some(id) => id,
        None => {
            eprintln!("Cannot join world: no local account authenticated");
            return Ok(None);
        }
    };

    let local_user_id = match ctx.current_user_id {
        Some(id) => id,
        None => {
            eprintln!("Cannot join world: no local user ID");
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
    ctx.instance_user_ids.clear();
    ctx.pending_avatars.clear();

    // Upsert world
    let world_db_id = operations::upsert_world(conn, world_id, world_name, timestamp)?;

    // Upsert world name history
    let world_name_history_id =
        operations::upsert_world_name_history(conn, world_db_id, world_name, timestamp)?;

    // Create new instance
    let new_instance_id = operations::create_instance(
        conn,
        my_account_id,
        world_db_id,
        Some(world_name_history_id),
        instance_id,
        timestamp,
    )?;

    *ctx.current_instance_id = Some(new_instance_id);

    // Add local user to instance
    let local_display_name = operations::get_user_display_name(conn, local_user_id)?;
    let display_name_history_id =
        operations::upsert_user_name_history(conn, local_user_id, &local_display_name, timestamp)?;

    let instance_user_id = operations::add_user_to_instance(
        conn,
        new_instance_id,
        local_user_id,
        display_name_history_id,
        timestamp,
    )?;

    ctx.instance_user_ids
        .insert(local_user_id, instance_user_id);

    println!(
        "Created new instance: {} in world: {}",
        new_instance_id, world_name
    );

    Ok(Some(ProcessedEvent::InstanceCreated {
        instance_id: new_instance_id,
        my_account_id,
        world_id: world_id.to_string(),
        world_name: world_name.to_string(),
        vrchat_instance_id: instance_id.to_string(),
        started_at: timestamp.to_string(),
        status: InstanceStatus::Active,
    }))
}
