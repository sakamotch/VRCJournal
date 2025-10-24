use crate::db::operations;
use crate::event_handler::HandlerContext;
use crate::types::VRChatEvent;
use chrono::{DateTime, Utc};
use rusqlite::Connection;

pub fn handle(
    conn: &Connection,
    ctx: &HandlerContext,
    timestamp: DateTime<Utc>,
    file_path: &str,
) -> Result<Option<VRChatEvent>, rusqlite::Error> {
    let timestamp_ms = timestamp.timestamp_millis();

    let instance_id = match *ctx.current_instance_id {
        Some(id) => id,
        None => {
            eprintln!("Screenshot taken but no active instance");
            return Ok(None);
        }
    };

    let screenshot_id = operations::record_screenshot(conn, instance_id, file_path, timestamp_ms)?;

    println!("Screenshot recorded: {}", file_path);

    Ok(Some(VRChatEvent::ScreenshotTaken {
        instance_id,
        screenshot_id,
        file_path: file_path.to_string(),
        taken_at: timestamp_ms,
    }))
}
