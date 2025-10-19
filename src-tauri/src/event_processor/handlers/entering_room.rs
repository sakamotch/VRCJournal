use crate::db::operations;
use crate::event_processor::ProcessedEvent;
use rusqlite::Connection;

pub fn handle(
    conn: &Connection,
    _timestamp: &str,
    world_name: &str,
    current_instance_id: Option<i64>,
) -> Result<Option<ProcessedEvent>, rusqlite::Error> {
    // Update world name for current instance
    if let Some(instance_id) = current_instance_id {
        // Get world_id from instance
        let world_id = operations::get_instance_world_id(conn, instance_id)?;

        // Update world name in worlds table
        operations::update_world_name(conn, world_id, world_name)?;

        println!("Updated world name for instance {}: {}", instance_id, world_name);
    }

    Ok(None)
}
