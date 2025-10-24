use crate::db::operations;
use crate::event_handler::HandlerContext;
use crate::types::VRChatEvent;
use chrono::{DateTime, Utc};
use rusqlite::Connection;

pub fn handle(
    conn: &Connection,
    ctx: &HandlerContext,
    timestamp: DateTime<Utc>,
    world_name: &str,
) -> Result<Option<VRChatEvent>, rusqlite::Error> {
    let timestamp_ms = timestamp.timestamp_millis();

    // Update world name for current instance
    let instance_id = match *ctx.current_instance_id {
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

    // Upsert world name history
    let world_name_history_id =
        operations::upsert_world_name_history(conn, world_id, world_name, timestamp_ms)?;

    // Link world_name_history to the instance
    operations::update_instance_world_name_history(conn, instance_id, world_name_history_id)?;

    println!(
        "Updated world name for instance {}: {}",
        instance_id, world_name
    );

    Ok(Some(VRChatEvent::WorldNameUpdated {
        instance_id,
        world_name: world_name.to_string(),
        updated_at: timestamp_ms,
    }))
}
