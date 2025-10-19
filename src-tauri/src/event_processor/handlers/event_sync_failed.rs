use crate::db::{operations, InstanceStatus};
use crate::event_processor::ProcessedEvent;
use rusqlite::Connection;

pub fn handle(
    conn: &Connection,
    _timestamp: &str,
    current_instance_id: Option<i64>,
) -> Result<Option<ProcessedEvent>, rusqlite::Error> {
    if let Some(instance_id) = current_instance_id {
        operations::update_instance_status(conn, instance_id, InstanceStatus::SyncFailed)?;
        println!("Instance {} marked as sync_failed", instance_id);
    } else {
        eprintln!("EventSyncFailed without active instance");
    }

    Ok(None)
}
