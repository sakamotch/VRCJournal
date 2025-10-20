use crate::{db, event_handler::EventHandler, log_reader::LogReader, types::{LogEvent, ProcessedEvent}};

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

    /// Initialize monitor
    pub fn initialize(&mut self) -> Result<(), String> {
        self.reader.initialize()?;

        self.restore_state()?;
        self.process_backlog()?;

        Ok(())
    }

    /// Restore previous state from database
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

    /// Process backlog events (save to DB only, don't emit to frontend)
    fn process_backlog(&mut self) -> Result<(), String> {
        let events = self
            .reader
            .read_backlog_events()
            .map_err(|e| format!("Failed to read backlog: {}", e))?;

        if events.is_empty() {
            return Ok(());
        }

        self.process_events(events);

        let conn = self.database.connection();
        self.reader.save_file_states(conn);

        Ok(())
    }

    /// Fetch new events
    pub fn fetch_new_events(&mut self) -> Result<Vec<ProcessedEvent>, String> {
        let events = self
            .reader
            .poll_new_events()
            .map_err(|e| format!("Failed to poll events: {}", e))?;

        if events.is_empty() {
            return Ok(Vec::new());
        }

        let processed = self.process_events(events);

        let conn = self.database.connection();
        self.reader.save_file_states(conn);

        Ok(processed)
    }

    /// Process events and return processed events
    fn process_events(&mut self, events: Vec<LogEvent>) -> Vec<ProcessedEvent> {
        let mut processed = Vec::new();
        let conn = self.database.connection();

        for event in events {
            match self.handler.process_event(conn, event) {
                Ok(Some(processed_event)) => {
                    processed.push(processed_event);
                }
                Ok(None) => {}
                Err(e) => {
                    eprintln!("Failed to process event: {}", e);
                }
            }
        }

        processed
    }
}
