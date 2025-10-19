use crate::db;
use chrono::Utc;
use rusqlite::Connection;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

/// ファイル状態をDBに保存
pub fn save_file_states(file_states: &Arc<Mutex<HashMap<PathBuf, u64>>>, conn: &Connection) {
    let states = file_states.lock().unwrap();

    for (path, position) in states.iter() {
        let path_str = path.to_string_lossy().to_string();
        if let Ok(metadata) = std::fs::metadata(path) {
            let file_size = metadata.len();
            if let Ok(modified) = metadata.modified() {
                let modified_dt = chrono::DateTime::<Utc>::from(modified);
                let _ = db::operations::upsert_log_file(conn, &path_str, file_size, modified_dt);
                let _ = db::operations::update_log_file_position(conn, &path_str, *position);
            }
        }
    }
}
