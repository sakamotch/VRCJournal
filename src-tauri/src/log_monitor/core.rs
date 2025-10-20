use crate::{db, event_handler::EventHandler, log_reader::LogReader, types::LogEvent};
use std::time::Duration;
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
    pub fn new(database: db::Database) -> Self {
        Self {
            reader: LogReader::new(),
            handler: EventHandler::new(),
            database,
        }
    }

    /// 監視サービスの初期化
    ///
    /// 前回終了時の状態を復元し、バックログイベントを処理する
    /// アプリケーション起動時に一度だけ呼ぶ
    pub fn initialize(&mut self) -> Result<usize, String> {
        // LogReader の初期化
        self.reader.initialize()?;

        // 状態復元
        self.restore_state()?;

        // バックログ処理
        let count = self.process_backlog()?;

        Ok(count)
    }

    /// リアルタイム処理ループを開始（ブロッキング）
    ///
    /// ログファイルを定期的にポーリングし、新しいイベントを処理する
    /// このメソッドは無限ループのため、別スレッドで実行すること
    pub fn run(self, app_handle: AppHandle) {
        self.run_realtime_loop(app_handle)
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

    /// バックログイベントを読み込んで処理
    ///
    /// アプリケーション起動時に、前回終了後から現在までに
    /// 蓄積されたログイベントを一括処理する
    fn process_backlog(&mut self) -> Result<usize, String> {
        // バックログイベントを読み込み
        let events = self
            .reader
            .read_backlog_events()
            .map_err(|e| format!("Failed to read backlog: {}", e))?;

        if events.is_empty() {
            return Ok(0);
        }

        // バッチ処理（フロントエンドには送信しない）
        let count = self.process_events(events, None);

        // ファイル位置を保存
        let conn = self.database.connection();
        self.reader.save_file_states(conn);

        Ok(count)
    }

    /// リアルタイム処理ループ（ブロッキング）
    ///
    /// 定期的にログファイルをポーリングし、
    /// 新しいイベントを検出して処理する
    fn run_realtime_loop(mut self, app_handle: AppHandle) {
        loop {
            std::thread::sleep(Duration::from_millis(1000));

            // 新しいイベントをポーリング
            let events = match self.reader.poll_new_events() {
                Ok(events) if !events.is_empty() => events,
                Ok(_) => continue, // イベントなし
                Err(e) => {
                    eprintln!("Failed to poll events: {}", e);
                    continue;
                }
            };

            // イベント処理とフロントエンドへの送信
            self.process_events(events, Some(&app_handle));

            // ファイル位置を保存
            let conn = self.database.connection();
            self.reader.save_file_states(conn);
        }
    }

    /// イベント一覧を処理して ProcessedEvent に変換
    ///
    /// - app_handle が None の場合: バックログ処理（フロントエンドに送信しない）
    /// - app_handle が Some の場合: リアルタイム処理（フロントエンドに送信する）
    fn process_events(&mut self, events: Vec<LogEvent>, app_handle: Option<&AppHandle>) -> usize {
        let mut count = 0;
        let conn = self.database.connection();

        for event in events {
            match self.handler.process_event(conn, event) {
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
}
