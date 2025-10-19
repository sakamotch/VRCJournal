use crate::parser::log_parser::VRChatLogParser;
use crate::parser::types::LogEvent;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use super::get_all_log_files;

/// ファイル読み込み処理
pub struct FileReader {
    parser: VRChatLogParser,
}

impl FileReader {
    pub fn new() -> Self {
        Self {
            parser: VRChatLogParser::new(),
        }
    }

    /// 全てのログファイルを初期読み込み（指定した位置から）
    pub fn read_all_logs(
        &self,
        log_dir: &PathBuf,
        file_positions: HashMap<PathBuf, u64>,
        file_states: &Arc<Mutex<HashMap<PathBuf, u64>>>,
    ) -> Result<Vec<(PathBuf, LogEvent)>, String> {
        let log_files = get_all_log_files(log_dir)?;
        let mut all_events = Vec::new();

        for log_file in log_files {
            let start_position = file_positions.get(&log_file).copied().unwrap_or(0);
            let (events, final_position) =
                self.read_file_from_position(&log_file, start_position)?;

            // ファイルの状態を記録
            file_states
                .lock()
                .unwrap()
                .insert(log_file.clone(), final_position);

            for event in events {
                all_events.push((log_file.clone(), event));
            }
        }

        Ok(all_events)
    }

    /// 指定したファイルを指定位置から読み込み
    pub fn read_file_from_position(
        &self,
        file_path: &PathBuf,
        start_position: u64,
    ) -> Result<(Vec<LogEvent>, u64), String> {
        let mut file = File::open(file_path)
            .map_err(|e| format!("Failed to open log file {:?}: {}", file_path, e))?;

        file.seek(SeekFrom::Start(start_position))
            .map_err(|e| format!("Failed to seek file: {}", e))?;

        let mut events = Vec::new();

        // バイト列を読み込んで、UTF-8エラーを無視しながら行ごとに処理
        let mut buffer = Vec::new();
        let bytes_read = file
            .read_to_end(&mut buffer)
            .map_err(|e| format!("Failed to read file: {}", e))?;

        // 不正なUTF-8シーケンスを置換して文字列に変換
        let content = String::from_utf8_lossy(&buffer);

        for line in content.lines() {
            if let Some(event) = self.parser.parse_line(line) {
                events.push(event);
            }
        }

        // 新しい位置は元の位置 + 実際に読み込んだバイト数
        let final_position = start_position + bytes_read as u64;
        Ok((events, final_position))
    }
}
