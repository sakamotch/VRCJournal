use crate::parser::log_parser::VRChatLogParser;
use crate::parser::types::LogEvent;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Read, Seek, SeekFrom};
use std::path::PathBuf;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use super::get_all_log_files;

/// リアルタイムファイル監視
pub fn start_watching(
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

                            if let Err(e) = handle_file_change(
                                &file_path,
                                &file_states,
                                &parser,
                                &event_tx,
                            ) {
                                eprintln!("Error handling new file: {}", e);
                            }
                        }
                        // ファイルサイズの変更を検知
                        else if current_size > previous_size {
                            sizes.insert(file_path.clone(), current_size);

                            if let Err(e) = handle_file_change(
                                &file_path,
                                &file_states,
                                &parser,
                                &event_tx,
                            ) {
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

                            if let Err(e) = handle_file_change(
                                &file_path,
                                &file_states,
                                &parser,
                                &event_tx,
                            ) {
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
