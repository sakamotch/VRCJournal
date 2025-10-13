use super::path::{get_all_log_files, get_vrchat_log_path};
use crate::parser::log_parser::VRChatLogParser;
use crate::parser::types::LogEvent;
use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};

/// ファイルごとの処理状態
struct FileState {
    path: PathBuf,
    position: u64,
}

pub struct LogWatcher {
    log_dir: PathBuf,
    file_states: Arc<Mutex<HashMap<PathBuf, u64>>>,
    parser: VRChatLogParser,
    event_tx: Sender<(PathBuf, LogEvent)>,
    event_rx: Arc<Mutex<Receiver<(PathBuf, LogEvent)>>>,
}

impl LogWatcher {
    /// 新しいLogWatcherを作成
    pub fn new() -> Result<Self, String> {
        let log_dir = get_vrchat_log_path()?;
        let (event_tx, event_rx) = channel();

        Ok(Self {
            log_dir,
            file_states: Arc::new(Mutex::new(HashMap::new())),
            parser: VRChatLogParser::new(),
            event_tx,
            event_rx: Arc::new(Mutex::new(event_rx)),
        })
    }

    /// 全てのログファイルを初期読み込み（指定した位置から）
    pub fn read_all_logs(
        &mut self,
        file_positions: HashMap<PathBuf, u64>,
    ) -> Result<Vec<(PathBuf, LogEvent)>, String> {
        let log_files = get_all_log_files()?;
        let mut all_events = Vec::new();

        for log_file in log_files {
            let start_position = file_positions.get(&log_file).copied().unwrap_or(0);
            let (events, final_position) = self.read_file_from_position(&log_file, start_position)?;

            // ファイルの状態を記録
            self.file_states.lock().unwrap().insert(log_file.clone(), final_position);

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
        let mut current_position = start_position;

        // バイト列を読み込んで、UTF-8エラーを無視しながら行ごとに処理
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)
            .map_err(|e| format!("Failed to read file: {}", e))?;

        // 不正なUTF-8シーケンスを置換して文字列に変換
        let content = String::from_utf8_lossy(&buffer);

        for line in content.lines() {
            current_position += line.len() as u64 + 1; // +1 for newline

            if let Some(event) = self.parser.parse_line(line) {
                events.push(event);
            }
        }

        Ok((events, current_position))
    }

    /// ディレクトリ監視を開始（ファイルの変更と新規作成を検知）
    pub fn start_watching(&self) -> Result<(), String> {
        let (tx, rx) = channel();

        let log_dir = self.log_dir.clone();
        let file_states = Arc::clone(&self.file_states);
        let parser = VRChatLogParser::new();
        let event_tx = self.event_tx.clone();

        // ディレクトリ監視用のスレッドを起動
        std::thread::spawn(move || {
            let mut watcher = RecommendedWatcher::new(
                move |res: Result<Event, notify::Error>| {
                    if let Ok(event) = res {
                        let _ = tx.send(event);
                    }
                },
                Config::default(),
            )
            .expect("Failed to create watcher");

            // ディレクトリ全体を監視
            watcher
                .watch(&log_dir, RecursiveMode::NonRecursive)
                .expect("Failed to watch log directory");

            // イベントを処理
            for event in rx {
                match event.kind {
                    EventKind::Modify(_) => {
                        // ファイルの変更を検知
                        for path in &event.paths {
                            if Self::is_log_file(path) {
                                if let Err(e) = Self::handle_file_change(
                                    path,
                                    &file_states,
                                    &parser,
                                    &event_tx,
                                ) {
                                    eprintln!("Error handling file change: {}", e);
                                }
                            }
                        }
                    }
                    EventKind::Create(_) => {
                        // 新規ファイルの作成を検知
                        for path in &event.paths {
                            if Self::is_log_file(path) {
                                println!("New log file detected: {:?}", path);
                                // 新規ファイルは最初から読む
                                file_states.lock().unwrap().insert(path.clone(), 0);
                                if let Err(e) = Self::handle_file_change(
                                    path,
                                    &file_states,
                                    &parser,
                                    &event_tx,
                                ) {
                                    eprintln!("Error handling new file: {}", e);
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        });

        Ok(())
    }

    /// output_log_*.txt ファイルかどうか判定
    fn is_log_file(path: &PathBuf) -> bool {
        path.file_name()
            .and_then(|name| name.to_str())
            .map(|name| name.starts_with("output_log") && name.ends_with(".txt"))
            .unwrap_or(false)
    }

    /// ファイル変更を処理
    fn handle_file_change(
        log_path: &PathBuf,
        file_states: &Arc<Mutex<HashMap<PathBuf, u64>>>,
        parser: &VRChatLogParser,
        event_tx: &Sender<(PathBuf, LogEvent)>,
    ) -> Result<(), String> {
        let mut file = File::open(log_path)
            .map_err(|e| format!("Failed to open log file: {}", e))?;

        let mut states = file_states.lock().unwrap();
        let position = states.get(log_path).copied().unwrap_or(0);

        file.seek(SeekFrom::Start(position))
            .map_err(|e| format!("Failed to seek file: {}", e))?;

        let mut new_position = position;

        // バイト列を読み込んで、UTF-8エラーを無視しながら行ごとに処理
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)
            .map_err(|e| format!("Failed to read file: {}", e))?;

        // 不正なUTF-8シーケンスを置換して文字列に変換
        let content = String::from_utf8_lossy(&buffer);

        for line in content.lines() {
            new_position += line.len() as u64 + 1; // +1 for newline

            if let Some(event) = parser.parse_line(line) {
                let _ = event_tx.send((log_path.clone(), event));
            }
        }

        states.insert(log_path.clone(), new_position);
        Ok(())
    }

    /// 受信したイベントを取得（ファイルパスとイベントのペア）
    pub fn recv_event(&self) -> Result<(PathBuf, LogEvent), String> {
        self.event_rx
            .lock()
            .unwrap()
            .recv()
            .map_err(|e| format!("Failed to receive event: {}", e))
    }

    /// 受信したイベントを非ブロッキングで取得
    pub fn try_recv_event(&self) -> Option<(PathBuf, LogEvent)> {
        self.event_rx.lock().unwrap().try_recv().ok()
    }

    /// 現在のファイル状態を取得（外部でDB保存するため）
    pub fn get_file_states(&self) -> HashMap<PathBuf, u64> {
        self.file_states.lock().unwrap().clone()
    }
}

impl Default for LogWatcher {
    fn default() -> Self {
        Self::new().expect("Failed to create LogWatcher")
    }
}
