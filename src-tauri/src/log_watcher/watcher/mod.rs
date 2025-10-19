mod initializer;
mod monitor;
mod reader;

use super::path::{get_all_log_files, get_vrchat_log_path};
use crate::parser::types::LogEvent;
use rusqlite::Connection;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};

pub struct LogWatcher {
    log_dir: PathBuf,
    file_states: Arc<Mutex<HashMap<PathBuf, u64>>>,
    reader: reader::FileReader,
    event_tx: Sender<Vec<(PathBuf, LogEvent)>>,
    event_rx: Arc<Mutex<Receiver<Vec<(PathBuf, LogEvent)>>>>,
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
            reader: reader::FileReader::new(),
            event_tx,
            event_rx: Arc::new(Mutex::new(event_rx)),
        })
    }

    /// DBから前回の処理位置を復元
    pub fn restore_file_positions(&mut self, conn: &Connection) -> Result<(), String> {
        let file_positions = initializer::load_file_positions(conn);

        // ファイル位置を復元
        let mut states = self.file_states.lock().unwrap();
        for (path, position) in file_positions {
            states.insert(path, position);
        }

        Ok(())
    }

    /// バックログイベント読み込み：前回位置からログを読み込んでイベント一覧を返す
    pub fn read_backlog_events(&mut self) -> Result<Vec<(PathBuf, LogEvent)>, String> {
        let file_positions = self.file_states.lock().unwrap().clone();

        self.reader.read_all_logs(
            &self.log_dir,
            file_positions,
            &self.file_states,
        )
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
    pub fn recv_realtime_events(&self) -> Result<Vec<(PathBuf, LogEvent)>, String> {
        self.event_rx
            .lock()
            .unwrap()
            .recv()
            .map_err(|e| format!("Failed to receive events: {}", e))
    }

    /// ファイル状態をDBに保存
    pub fn save_file_states(&self, conn: &Connection) {
        initializer::save_file_states(&self.file_states, conn);
    }
}

impl Default for LogWatcher {
    fn default() -> Self {
        Self::new().expect("Failed to create LogWatcher")
    }
}
