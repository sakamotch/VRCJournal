use super::path::{get_all_log_files, get_vrchat_log_path};
use crate::{
    db,
    parser::{log_parser::VRChatLogParser, types::LogEvent},
};
use chrono::Utc;
use rusqlite::Connection;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Read, Seek, SeekFrom};
use std::path::PathBuf;

pub struct LogWatcher {
    log_dir: PathBuf,
    file_states: HashMap<PathBuf, u64>,
    parser: VRChatLogParser,
}

impl LogWatcher {
    /// 新しいLogWatcherを作成
    pub fn new() -> Result<Self, String> {
        let log_dir = get_vrchat_log_path()?;

        Ok(Self {
            log_dir,
            file_states: HashMap::new(),
            parser: VRChatLogParser::new(),
        })
    }

    /// DBから前回の処理位置を復元
    ///
    /// ファイルシステムに存在する全ファイルについて、DBから読み込み位置を取得
    /// DBに記録されていないファイルは位置0から開始
    pub fn restore_file_positions(&mut self, conn: &Connection) -> Result<(), String> {
        let log_files = get_all_log_files(&self.log_dir)?;

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

    /// バックログイベント読み込み：前回位置からログを読み込んでイベント一覧を返す
    pub fn read_backlog_events(&mut self) -> Result<Vec<LogEvent>, String> {
        self.read_all_logs()
    }

    /// 新しいイベントをポーリング：ファイルサイズをチェックして変更があれば読み込む
    pub fn poll_new_events(&mut self) -> Result<Vec<LogEvent>, String> {
        let log_files = get_all_log_files(&self.log_dir)?;
        let mut all_events = Vec::new();

        for file_path in log_files {
            // ファイルサイズを取得
            let metadata = match fs::metadata(&file_path) {
                Ok(m) => m,
                Err(_) => continue,
            };
            let current_size = metadata.len();

            // 現在の読み込み位置を取得
            let position = self.file_states.get(&file_path).copied().unwrap_or(0);

            // 新しいデータがあるかチェック
            if current_size > position {
                if position == 0 {
                    println!("New log file detected: {:?}", file_path);
                }

                // ファイルを読み込む
                let (events, final_position) =
                    self.read_file_from_position(&file_path, position)?;
                self.file_states.insert(file_path.clone(), final_position);
                all_events.extend(events);
            }
        }

        Ok(all_events)
    }

    /// ファイル状態をDBに保存
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

    /// 全てのログファイルを初期読み込み（file_statesに記録された位置から）
    fn read_all_logs(&mut self) -> Result<Vec<LogEvent>, String> {
        let log_files = get_all_log_files(&self.log_dir)?;
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

    /// 指定したファイルを指定位置から読み込み
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

impl Default for LogWatcher {
    fn default() -> Self {
        Self::new().expect("Failed to create LogWatcher")
    }
}
