use super::{database, monitor, path::{get_all_log_files, get_vrchat_log_path}};
use crate::parser::{log_parser::VRChatLogParser, types::LogEvent};
use rusqlite::Connection;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};

pub struct LogWatcher {
    log_dir: PathBuf,
    file_states: Arc<Mutex<HashMap<PathBuf, u64>>>,
    parser: VRChatLogParser,
    event_tx: Sender<Vec<LogEvent>>,
    event_rx: Arc<Mutex<Receiver<Vec<LogEvent>>>>,
}

impl LogWatcher {
    /// 新しいLogWatcherを作成
    pub fn new() -> Result<Self, String> {
        // Verify VRChat log directory exists and store the path
        let log_dir = match get_vrchat_log_path() {
            Ok(path) => path,
            Err(e) => return Err(e),
        };

        let (event_tx, event_rx) = channel();

        Ok(Self {
            log_dir,
            file_states: Arc::new(Mutex::new(HashMap::new())),
            parser: VRChatLogParser::new(),
            event_tx,
            event_rx: Arc::new(Mutex::new(event_rx)),
        })
    }

    /// DBから前回の処理位置を復元
    pub fn restore_file_positions(&mut self, conn: &Connection) -> Result<(), String> {
        let file_positions = database::load_file_positions(conn);

        // ファイル位置を復元
        let mut states = self.file_states.lock().unwrap();
        for (path, position) in file_positions {
            states.insert(path, position);
        }

        Ok(())
    }

    /// バックログイベント読み込み：前回位置からログを読み込んでイベント一覧を返す
    pub fn read_backlog_events(&mut self) -> Result<Vec<LogEvent>, String> {
        let file_positions = self.file_states.lock().unwrap().clone();

        self.read_all_logs(file_positions)
    }

    /// ディレクトリ監視を開始（独自ポーリングでファイルサイズをチェック）
    pub fn start_watching(&self) -> Result<(), String> {
        monitor::start_watching(
            self.log_dir.clone(),
            Arc::clone(&self.file_states),
            self.event_tx.clone(),
        )
    }

    /// リアルタイムイベント受信：ファイル変更で検知したイベント一覧を受信
    pub fn recv_realtime_events(&self) -> Result<Vec<LogEvent>, String> {
        self.event_rx
            .lock()
            .unwrap()
            .recv()
            .map_err(|e| format!("Failed to receive events: {}", e))
    }

    /// ファイル状態をDBに保存
    pub fn save_file_states(&self, conn: &Connection) {
        database::save_file_states(&self.file_states, conn);
    }

    /// 全てのログファイルを初期読み込み（指定した位置から）
    fn read_all_logs(
        &self,
        file_positions: HashMap<PathBuf, u64>,
    ) -> Result<Vec<LogEvent>, String> {
        let log_files = get_all_log_files(&self.log_dir)?;
        let mut all_events = Vec::new();

        for log_file in log_files {
            let start_position = file_positions.get(&log_file).copied().unwrap_or(0);
            let (events, final_position) = self.read_file_from_position(&log_file, start_position)?;

            // ファイルの状態を記録
            self.file_states
                .lock()
                .unwrap()
                .insert(log_file.clone(), final_position);

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
