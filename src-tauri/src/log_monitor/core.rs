use crate::{
    db,
    event_handler::EventHandler,
    log_reader::LogReader,
    types::{LogEvent, ProcessedEvent},
};

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

    /// Process backlog events
    fn process_backlog(&mut self) -> Result<(), String> {
        let events = self
            .reader
            .read_backlog()
            .map_err(|e| format!("Failed to read backlog: {}", e))?;

        if events.is_empty() {
            return Ok(());
        }

        let processed = self.process_events(events)?;
        println!("Processed {} backlog events", processed.len());

        Ok(())
    }

    /// Fetch new events
    pub fn fetch_new_events(&mut self) -> Result<Vec<ProcessedEvent>, String> {
        let events = self
            .reader
            .read_new_events()
            .map_err(|e| format!("Failed to read new events: {}", e))?;

        if events.is_empty() {
            return Ok(Vec::new());
        }

        self.process_events(events)
    }

    /// Process events within a single transaction
    fn process_events(&mut self, events: Vec<LogEvent>) -> Result<Vec<ProcessedEvent>, String> {
        let tx = self
            .database
            .transaction()
            .map_err(|e| format!("Failed to begin transaction: {}", e))?;

        let mut processed = Vec::new();
        for event in events {
            match self.handler.process_event(&tx, event) {
                Ok(Some(processed_event)) => {
                    processed.push(processed_event);
                }
                Ok(None) => {}
                Err(e) => {
                    return Err(format!("Failed to process event: {}", e));
                }
            }
        }

        self.reader.save_file_states(&tx);

        tx.commit()
            .map_err(|e| format!("Failed to commit transaction: {}", e))?;

        Ok(processed)
    }
}
