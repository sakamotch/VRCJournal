use crate::{db, monitor::Monitor};
use tauri::{App, Emitter, Manager};

/// Tauriアプリケーションのセットアップ処理
///
/// アプリケーション全体の初期化を担当：
/// 1. データベースの初期化とマイグレーション
/// 2. ログ監視の起動（VRChat ログ監視）
pub fn setup(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let database = setup_database(app)?;
    start_log_monitor(database, app.handle().clone());
    Ok(())
}

/// データベースのセットアップ
///
/// アプリケーションデータディレクトリを作成し、
/// データベースを開いてマイグレーションを実行する
fn setup_database(app: &App) -> Result<db::Database, Box<dyn std::error::Error>> {
    let app_data_dir = app.path().app_data_dir()?;
    std::fs::create_dir_all(&app_data_dir)?;
    let db_path = app_data_dir.join("vrcjournal.db");

    let database = db::Database::open(db_path)?;
    database.migrate()?;

    Ok(database)
}

/// ログ監視の起動
///
/// バックグラウンドスレッドで Monitor を実行し、
/// VRChat ログの監視とイベント処理を開始する
fn start_log_monitor(database: db::Database, app_handle: tauri::AppHandle) {
    std::thread::spawn(move || {
        // Monitor を作成
        let mut monitor = match Monitor::new(database) {
            Ok(m) => m,
            Err(e) => {
                eprintln!("Failed to create monitor: {}", e);
                return;
            }
        };

        // 初期化（状態復元 + バックログ処理）
        match monitor.initialize() {
            Ok(count) => println!("Monitor initialized: {} backlog events processed", count),
            Err(e) => {
                eprintln!("Failed to initialize monitor: {}", e);
                return;
            }
        }

        // バックエンド準備完了を通知
        if let Err(e) = app_handle.emit("backend-ready", ()) {
            eprintln!("Failed to emit backend-ready event: {}", e);
            return;
        }

        // リアルタイム処理ループ（ブロッキング）
        monitor.run(app_handle);
    });
}
