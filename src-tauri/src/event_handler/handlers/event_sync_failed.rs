use crate::db::operations;
use crate::event_handler::core::HandlerContext;
use crate::types::{InstanceStatus, ProcessedEvent};
use rusqlite::Connection;

pub fn handle(
    conn: &Connection,
    ctx: &HandlerContext,
    timestamp: &str,
) -> Result<Option<ProcessedEvent>, rusqlite::Error> {
    let instance_id = match ctx.current_instance_id.as_ref().copied() {
        Some(id) => id,
        None => {
            eprintln!("EventSyncFailed without active instance");
            return Ok(None);
        }
    };

    operations::update_instance_status(conn, instance_id, InstanceStatus::SyncFailed)?;
    println!("Instance {} marked as sync_failed", instance_id);

    Ok(Some(ProcessedEvent::InstanceSyncFailed {
        instance_id,
        failed_at: timestamp.to_string(),
        status: InstanceStatus::SyncFailed,
    }))
}
