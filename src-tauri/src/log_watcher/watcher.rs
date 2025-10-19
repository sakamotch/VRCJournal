use super::path::{get_all_log_files, get_vrchat_log_path};
use crate::{db, parser::log_parser::VRChatLogParser, parser::types::LogEvent};
use chrono::Utc;
use rusqlite::Connection;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Read, Seek, SeekFrom};
use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub struct LogWatcher {
    log_dir: PathBuf,
    file_states: Arc<Mutex<HashMap<PathBuf, u64>>>,
    parser: VRChatLogParser,
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
            parser: VRChatLogParser::new(),
            event_tx,
            event_rx: Arc::new(Mutex::new(event_rx)),
        })
    }

    /// DBから前回の処理位置を復元
    pub fn restore_file_positions(&mut self, conn: &Connection) -> Result<(), String> {
        let file_positions = load_file_positions(conn);

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

        self.read_all_logs(file_positions)
    }

    /// ディレクトリ監視を開始（独自ポーリングでファイルサイズをチェック）
    pub fn start_watching(&self) -> Result<(), String> {
        start_watching(
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
        save_file_states(&self.file_states, conn);
    }

    /// 全てのログファイルを初期読み込み（指定した位置から）
    fn read_all_logs(
        &self,
        file_positions: HashMap<PathBuf, u64>,
    ) -> Result<Vec<(PathBuf, LogEvent)>, String> {
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

            for event in events {
                all_events.push((log_file.clone(), event));
            }
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

// ===== Helper functions =====

/// DBから前回の処理位置を取得
fn load_file_positions(conn: &Connection) -> HashMap<PathBuf, u64> {
    db::operations::get_all_log_files(conn)
        .map(|files| {
            files
                .into_iter()
                .map(|(path, _, pos, _)| (PathBuf::from(path), pos))
                .collect()
        })
        .unwrap_or_else(|e| {
            eprintln!("Failed to get tracked files: {}", e);
            HashMap::new()
        })
}

/// ファイル状態をDBに保存
fn save_file_states(file_states: &Arc<Mutex<HashMap<PathBuf, u64>>>, conn: &Connection) {
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

/// リアルタイムファイル監視
fn start_watching(
    log_dir: PathBuf,
    file_states: Arc<Mutex<HashMap<PathBuf, u64>>>,
    event_tx: Sender<Vec<(PathBuf, LogEvent)>>,
) -> Result<(), String> {
    // ファイルサイズを記録するマップ（ポーリング用）
    let file_sizes: Arc<Mutex<HashMap<PathBuf, u64>>> = Arc::new(Mutex::new(HashMap::new()));

    // 既存のファイルサイズを初期化
    if let Ok(log_files) = get_all_log_files(&log_dir) {
        let mut sizes = file_sizes.lock().unwrap();
        for file in log_files {
            if let Ok(metadata) = fs::metadata(&file) {
                sizes.insert(file.clone(), metadata.len());
            }
        }
    }

    let parser = VRChatLogParser::new();

    // ポーリングスレッドを起動
    std::thread::spawn(move || {
        loop {
            // ディレクトリ内のログファイルをチェック
            if let Ok(log_files) = get_all_log_files(&log_dir) {
                let mut sizes = file_sizes.lock().unwrap();

                for file_path in log_files {
                    if let Ok(metadata) = fs::metadata(&file_path) {
                        let current_size = metadata.len();
                        let previous_size = sizes.get(&file_path).copied().unwrap_or(0);

                        // 新規ファイルの検知
                        if previous_size == 0 && current_size > 0 {
                            println!("New log file detected: {:?}", file_path);
                            file_states.lock().unwrap().insert(file_path.clone(), 0);
                            sizes.insert(file_path.clone(), current_size);

                            if let Err(e) =
                                handle_file_change(&file_path, &file_states, &parser, &event_tx)
                            {
                                eprintln!("Error handling new file: {}", e);
                            }
                        }
                        // ファイルサイズの変更を検知
                        else if current_size > previous_size {
                            sizes.insert(file_path.clone(), current_size);

                            if let Err(e) =
                                handle_file_change(&file_path, &file_states, &parser, &event_tx)
                            {
                                eprintln!("Error handling file change: {}", e);
                            }
                        }
                        // ファイルサイズが減った場合（ログローテーション等）
                        else if current_size < previous_size {
                            println!(
                                "File size decreased, resetting position: {:?}",
                                file_path
                            );
                            file_states.lock().unwrap().insert(file_path.clone(), 0);
                            sizes.insert(file_path.clone(), current_size);

                            if let Err(e) =
                                handle_file_change(&file_path, &file_states, &parser, &event_tx)
                            {
                                eprintln!("Error handling file reset: {}", e);
                            }
                        }
                    }
                }
            }

            // 500ミリ秒ごとにチェック
            std::thread::sleep(Duration::from_millis(500));
        }
    });

    Ok(())
}

/// ファイル変更を処理
fn handle_file_change(
    log_path: &PathBuf,
    file_states: &Arc<Mutex<HashMap<PathBuf, u64>>>,
    parser: &VRChatLogParser,
    event_tx: &Sender<Vec<(PathBuf, LogEvent)>>,
) -> Result<(), String> {
    let mut file =
        File::open(log_path).map_err(|e| format!("Failed to open log file: {}", e))?;

    let mut states = file_states.lock().unwrap();
    let position = states.get(log_path).copied().unwrap_or(0);

    file.seek(SeekFrom::Start(position))
        .map_err(|e| format!("Failed to seek file: {}", e))?;

    // バイト列を読み込んで、UTF-8エラーを無視しながら行ごとに処理
    let mut buffer = Vec::new();
    let bytes_read = file
        .read_to_end(&mut buffer)
        .map_err(|e| format!("Failed to read file: {}", e))?;

    // 新しく読み込んだデータがない場合は何もしない
    if bytes_read == 0 {
        return Ok(());
    }

    // 不正なUTF-8シーケンスを置換して文字列に変換
    let content = String::from_utf8_lossy(&buffer);

    // イベント一覧を収集
    let mut events = Vec::new();
    for line in content.lines() {
        if let Some(event) = parser.parse_line(line) {
            events.push((log_path.clone(), event));
        }
    }

    // 新しい位置は元の位置 + 実際に読み込んだバイト数
    let new_position = position + bytes_read as u64;
    states.insert(log_path.clone(), new_position);

    // イベント一覧を一度に送信
    if !events.is_empty() {
        let _ = event_tx.send(events);
    }

    Ok(())
}
