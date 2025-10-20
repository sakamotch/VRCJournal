use crate::db::operations;
use crate::event_handler::core::HandlerContext;
use crate::types::ProcessedEvent;
use rusqlite::Connection;

pub fn handle(
    conn: &Connection,
    ctx: &HandlerContext,
    timestamp: &str,
    file_path: &str,
) -> Result<Option<ProcessedEvent>, rusqlite::Error> {
    let instance_id = match *ctx.current_instance_id {
        Some(id) => id,
        None => {
            eprintln!("Screenshot taken but no active instance");
            return Ok(None);
        }
    };

    let screenshot_id = operations::record_screenshot(conn, instance_id, file_path, timestamp)?;

    println!("Screenshot recorded: {}", file_path);

    Ok(Some(ProcessedEvent::ScreenshotTaken {
        instance_id,
        screenshot_id,
        file_path: file_path.to_string(),
        taken_at: timestamp.to_string(),
    }))
}
