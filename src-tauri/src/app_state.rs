use crate::db;
use crate::event_processor::EventProcessor;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Mutex<db::Database>>,
    pub event_processor: Arc<Mutex<EventProcessor>>,
}
