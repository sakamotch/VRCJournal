use crate::db::operations;
use crate::event_handler::HandlerContext;
use crate::types::{InstanceStatus, VRChatEvent};
use chrono::{DateTime, Utc};
use rusqlite::Connection;

pub fn handle(
    conn: &Connection,
    ctx: &HandlerContext,
    timestamp: DateTime<Utc>,
) -> Result<Option<VRChatEvent>, rusqlite::Error> {
    let timestamp_ms = timestamp.timestamp_millis();

    let instance_id = match *ctx.current_instance_id {
        Some(id) => id,
        None => {
            eprintln!("EventSyncFailed without active instance");
            return Ok(None);
        }
    };

    operations::update_instance_status(conn, instance_id, InstanceStatus::SyncFailed)?;
    println!("Instance {} marked as sync_failed", instance_id);

    Ok(Some(VRChatEvent::InstanceSyncFailed {
        instance_id,
        failed_at: timestamp_ms,
        status: InstanceStatus::SyncFailed,
    }))
}
