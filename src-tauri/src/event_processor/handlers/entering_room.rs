use crate::db::operations;
use crate::event_processor::{processor::ProcessorContext, ProcessedEvent};
use rusqlite::Connection;

pub fn handle(
    conn: &Connection,
    ctx: &ProcessorContext,
    timestamp: &str,
    world_name: &str,
) -> Result<Option<ProcessedEvent>, rusqlite::Error> {
    // Update world name for current instance
    let instance_id = match ctx.current_instance_id.as_ref().copied() {
        Some(id) => id,
        None => {
            eprintln!("EnteringRoom but no active instance");
            return Ok(None);
        }
    };

    // Get world_id from instance
    let world_id = operations::get_instance_world_id(conn, instance_id)?;

    // Update world name in worlds table
    operations::update_world_name(conn, world_id, world_name)?;

    println!(
        "Updated world name for instance {}: {}",
        instance_id, world_name
    );

    Ok(Some(ProcessedEvent::WorldNameUpdated {
        instance_id,
        world_name: world_name.to_string(),
        updated_at: timestamp.to_string(),
    }))
}
