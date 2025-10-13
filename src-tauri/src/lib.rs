mod db;
mod parser;

use std::sync::Mutex;
use tauri::Manager;

// グローバルステート
pub struct AppState {
    db: Mutex<db::Database>,
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // データベースパスを決定
            let app_data_dir = app.path().app_data_dir()
                .expect("Failed to get app data dir");

            // ディレクトリが存在しない場合は作成
            std::fs::create_dir_all(&app_data_dir)
                .expect("Failed to create app data directory");

            let db_path = app_data_dir.join("vrcjournal.db");

            // データベース初期化
            let database = db::Database::open(db_path)
                .expect("Failed to open database");

            database.migrate()
                .expect("Failed to run migrations");

            let app_state = AppState {
                db: Mutex::new(database),
            };

            app.manage(app_state);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
