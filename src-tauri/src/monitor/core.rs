use crate::{db, handler::EventHandler, reader::LogReader, types::LogEvent};
use rusqlite::Connection;
use tauri::{AppHandle, Emitter};

/// VRChat ログ監視サービス
///
/// ログファイルを監視し、イベントを抽出・処理して
/// フロントエンドに送信可能な形式に変換する
pub struct Monitor {
    reader: LogReader,
    handler: EventHandler,
    database: db::Database,
}

impl Monitor {
    /// 新しい Monitor を作成
    pub fn new(database: db::Database) -> Result<Self, String> {
        let reader = LogReader::new().map_err(|e| format!("Failed to create reader: {}", e))?;
        let handler = EventHandler::new();

        Ok(Self {
            reader,
            handler,
            database,
        })
    }

    /// 監視サービスの初期化
    ///
    /// 前回終了時の状態を復元し、バックログイベントを処理する
    /// アプリケーション起動時に一度だけ呼ぶ
    pub fn initialize(&mut self) -> Result<usize, String> {
        // 状態復元
        self.restore_state()?;

        // バックログ処理
        let count = super::backlog::process(&mut self.reader, &mut self.handler, &self.database)?;

        Ok(count)
    }

    /// リアルタイム処理ループを開始（ブロッキング）
    ///
    /// ログファイルを定期的にポーリングし、新しいイベントを処理する
    /// このメソッドは無限ループのため、別スレッドで実行すること
    pub fn run(self, app_handle: AppHandle) {
        super::realtime::run_loop(self.reader, self.handler, self.database, app_handle)
    }

    /// 前回終了時の状態を復元
    fn restore_state(&mut self) -> Result<(), String> {
        let conn = self.database.connection();

        // EventHandler の状態復元
        self.handler
            .restore_previous_state(conn)
            .map_err(|e| format!("Failed to restore handler state: {}", e))?;

        // LogReader のファイル位置復元
        self.reader
            .restore_file_positions(conn)
            .map_err(|e| format!("Failed to restore reader state: {}", e))?;

        Ok(())
    }
}

/// イベント一覧を処理して ProcessedEvent に変換
///
/// - app_handle が None の場合: バックログ処理（フロントエンドに送信しない）
/// - app_handle が Some の場合: リアルタイム処理（フロントエンドに送信する）
pub(super) fn process_events(
    conn: &Connection,
    handler: &mut EventHandler,
    events: Vec<LogEvent>,
    app_handle: Option<&AppHandle>,
) -> usize {
    let mut count = 0;

    for event in events {
        match handler.process_event(conn, event) {
            Ok(Some(processed_event)) => {
                // フロントエンドに送信（リアルタイムのみ）
                if let Some(handle) = app_handle {
                    if let Err(e) = handle.emit("log-event", &processed_event) {
                        eprintln!("Failed to emit event: {}", e);
                    }
                }
                count += 1;
            }
            Ok(None) => {
                // 通知不要なイベント
                count += 1;
            }
            Err(e) => {
                eprintln!("Failed to process event: {}", e);
            }
        }
    }

    count
}
