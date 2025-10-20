use crate::{db, event_handler::EventHandler, log_reader::LogReader, types::LogEvent};
use std::time::Duration;
use tauri::{AppHandle, Emitter};

/// VRChat log monitoring service
pub struct Monitor {
    reader: LogReader,
    handler: EventHandler,
    database: db::Database,
}

impl Monitor {
    pub fn new(database: db::Database) -> Self {
        Self {
            reader: LogReader::new(),
            handler: EventHandler::new(),
            database,
        }
    }

    /// Initialize monitor: restore state and process backlog
    pub fn initialize(&mut self) -> Result<usize, String> {
        self.reader.initialize()?;
        self.restore_state()?;
        let count = self.process_backlog()?;

        Ok(count)
    }

    /// Start real-time event processing loop (blocking)
    pub fn run(self, app_handle: AppHandle) {
        self.run_realtime_loop(app_handle)
    }

    fn restore_state(&mut self) -> Result<(), String> {
        let conn = self.database.connection();

        self.handler
            .restore_previous_state(conn)
            .map_err(|e| format!("Failed to restore handler state: {}", e))?;

        self.reader
            .restore_file_positions(conn)
            .map_err(|e| format!("Failed to restore reader state: {}", e))?;

        Ok(())
    }

    /// Process accumulated events since last shutdown
    fn process_backlog(&mut self) -> Result<usize, String> {
        let events = self
            .reader
            .read_backlog_events()
            .map_err(|e| format!("Failed to read backlog: {}", e))?;

        if events.is_empty() {
            return Ok(0);
        }

        let count = self.process_events(events, None);

        let conn = self.database.connection();
        self.reader.save_file_states(conn);

        Ok(count)
    }

    fn run_realtime_loop(mut self, app_handle: AppHandle) {
        loop {
            std::thread::sleep(Duration::from_millis(1000));

            let events = match self.reader.poll_new_events() {
                Ok(events) if !events.is_empty() => events,
                Ok(_) => continue,
                Err(e) => {
                    eprintln!("Failed to poll events: {}", e);
                    continue;
                }
            };

            self.process_events(events, Some(&app_handle));

            let conn = self.database.connection();
            self.reader.save_file_states(conn);
        }
    }

    /// Process events and emit to frontend if app_handle is provided
    fn process_events(&mut self, events: Vec<LogEvent>, app_handle: Option<&AppHandle>) -> usize {
        let mut count = 0;
        let conn = self.database.connection();

        for event in events {
            match self.handler.process_event(conn, event) {
                Ok(Some(processed_event)) => {
                    if let Some(handle) = app_handle {
                        if let Err(e) = handle.emit("log-event", &processed_event) {
                            eprintln!("Failed to emit event: {}", e);
                        }
                    }
                    count += 1;
                }
                Ok(None) => {
                    count += 1;
                }
                Err(e) => {
                    eprintln!("Failed to process event: {}", e);
                }
            }
        }

        count
    }
}
