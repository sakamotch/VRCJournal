mod commands;
mod db;
mod event_processor;
mod log_watcher;
mod parser;
mod setup;

use commands::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_os::init())
        .setup(setup::setup_app)
        .invoke_handler(tauri::generate_handler![
            open_invite_url,
            open_user_page,
            open_screenshot_directory
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
