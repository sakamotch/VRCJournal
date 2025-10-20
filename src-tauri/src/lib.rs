mod commands;
mod db;
mod event_processor;
mod log_watcher;
mod monitoring;
mod parser;
mod types;

use commands::*;
use tauri::{App, Manager};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_os::init())
        .setup(setup_app)
        .invoke_handler(tauri::generate_handler![
            open_invite_url,
            open_user_page,
            open_screenshot_directory
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/// Tauriアプリケーションのセットアップ処理
fn setup_app(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    // データベース初期化
    let app_data_dir = app.path().app_data_dir()?;
    std::fs::create_dir_all(&app_data_dir)?;
    let db_path = app_data_dir.join("vrcjournal.db");

    let database = db::Database::open(db_path)?;
    database.migrate()?;

    // バックグラウンドでログ監視を開始
    let app_handle = app.handle().clone();
    std::thread::spawn(move || {
        monitoring::start(database, app_handle);
    });

    Ok(())
}
