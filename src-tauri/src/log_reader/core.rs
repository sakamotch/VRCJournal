use crate::{db, log_parser::LogParser, types::LogEvent};
use chrono::Utc;
use rusqlite::Connection;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Read, Seek, SeekFrom};
use std::path::PathBuf;

pub struct LogReader {
    log_dir: PathBuf,
    file_states: HashMap<PathBuf, u64>,
    parser: LogParser,
}

impl LogReader {
    pub fn new() -> Self {
        Self {
            log_dir: PathBuf::new(),
            file_states: HashMap::new(),
            parser: LogParser::new(),
        }
    }

    /// Validate and set log directory
    pub fn initialize(&mut self) -> Result<(), String> {
        self.log_dir = Self::get_vrchat_log_path()?;

        if !self.log_dir.exists() {
            return Err(format!("Log directory not found: {:?}", self.log_dir));
        }

        Ok(())
    }

    /// Restore file read positions from database
    pub fn restore_file_positions(&mut self, conn: &Connection) -> Result<(), String> {
        let log_files = self.get_all_log_files()?;

        for log_file in log_files {
            let path_str = log_file.to_string_lossy().to_string();

            let position = db::operations::get_log_file_position(conn, &path_str)
                .unwrap_or(Some(0))
                .unwrap_or(0);

            self.file_states.insert(log_file.clone(), position);

            if position == 0 {
                println!("New file detected: {:?}", log_file);
            } else {
                println!("Restored file: {:?} at position {}", log_file, position);
            }
        }

        Ok(())
    }

    /// Read backlog events accumulated since last shutdown
    pub fn read_backlog(&mut self) -> Result<Vec<LogEvent>, String> {
        let log_files = self.get_all_log_files()?;
        let mut all_events = Vec::new();

        for log_file in log_files {
            let start_position = self.file_states.get(&log_file).copied().unwrap_or(0);
            let (events, final_position) =
                self.read_file_from_position(&log_file, start_position)?;

            self.file_states.insert(log_file.clone(), final_position);
            all_events.extend(events);
        }

        Ok(all_events)
    }

    /// Read new events by checking file size changes
    pub fn read_new_events(&mut self) -> Result<Vec<LogEvent>, String> {
        let log_files = self.get_all_log_files()?;
        let mut all_events = Vec::new();

        for file_path in log_files {
            let metadata = match fs::metadata(&file_path) {
                Ok(m) => m,
                Err(_) => continue,
            };
            let current_size = metadata.len();

            let position = self.file_states.get(&file_path).copied().unwrap_or(0);

            if current_size > position {
                if position == 0 {
                    println!("New log file detected: {:?}", file_path);
                }

                let (events, final_position) =
                    self.read_file_from_position(&file_path, position)?;
                self.file_states.insert(file_path.clone(), final_position);
                all_events.extend(events);
            }
        }

        Ok(all_events)
    }

    /// Save file states to database
    pub fn save_file_states(&self, conn: &Connection) {
        for (path, position) in self.file_states.iter() {
            let path_str = path.to_string_lossy().to_string();
            if let Ok(metadata) = fs::metadata(path) {
                let file_size = metadata.len();
                if let Ok(modified) = metadata.modified() {
                    let modified_dt = chrono::DateTime::<Utc>::from(modified);
                    let _ =
                        db::operations::upsert_log_file(conn, &path_str, file_size, modified_dt);
                    let _ = db::operations::update_log_file_position(conn, &path_str, *position);
                }
            }
        }
    }

    fn read_file_from_position(
        &self,
        file_path: &PathBuf,
        start_position: u64,
    ) -> Result<(Vec<LogEvent>, u64), String> {
        let mut file = File::open(file_path)
            .map_err(|e| format!("Failed to open log file {:?}: {}", file_path, e))?;

        file.seek(SeekFrom::Start(start_position))
            .map_err(|e| format!("Failed to seek file: {}", e))?;

        let mut events = Vec::new();

        let mut buffer = Vec::new();
        let bytes_read = file
            .read_to_end(&mut buffer)
            .map_err(|e| format!("Failed to read file: {}", e))?;

        let content = String::from_utf8_lossy(&buffer);

        for line in content.lines() {
            if let Some(event) = self.parser.parse_line(line) {
                events.push(event);
            }
        }

        let final_position = start_position + bytes_read as u64;
        Ok((events, final_position))
    }

    /// Get VRChat log directory path (Windows only)
    fn get_vrchat_log_path() -> Result<PathBuf, String> {
        #[cfg(target_os = "windows")]
        {
            match std::env::var("USERPROFILE") {
                Ok(userprofile) => {
                    let log_path = PathBuf::from(userprofile)
                        .join("AppData")
                        .join("LocalLow")
                        .join("VRChat")
                        .join("VRChat");

                    if log_path.exists() {
                        Ok(log_path)
                    } else {
                        Err(format!("VRChat log directory not found at {:?}", log_path))
                    }
                }
                Err(_) => Err("USERPROFILE environment variable not found".to_string()),
            }
        }

        #[cfg(not(target_os = "windows"))]
        {
            Err("VRChat is only available on Windows".to_string())
        }
    }

    /// Get all log files sorted by modification time (oldest first)
    fn get_all_log_files(&self) -> Result<Vec<PathBuf>, String> {
        let mut log_files: Vec<PathBuf> = std::fs::read_dir(&self.log_dir)
            .map_err(|e| format!("Failed to read log directory: {}", e))?
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path())
            .filter(|path| {
                path.file_name()
                    .and_then(|name| name.to_str())
                    .map(|name| name.starts_with("output_log") && name.ends_with(".txt"))
                    .unwrap_or(false)
            })
            .collect();

        if log_files.is_empty() {
            return Err("No VRChat log files found".to_string());
        }

        log_files.sort_by_key(|path| std::fs::metadata(path).and_then(|m| m.modified()).ok());

        Ok(log_files)
    }
}
