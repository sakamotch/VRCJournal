use crate::{db, log_monitor::Monitor};
use tauri::{App, Emitter, Manager};

/// Initialize database and start log monitoring
pub fn setup(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let database = setup_database(app)?;
    start_log_monitor(database, app.handle().clone());
    Ok(())
}

/// Setup database: create app data directory and run migrations
fn setup_database(app: &App) -> Result<db::Database, Box<dyn std::error::Error>> {
    let app_data_dir = app.path().app_data_dir()?;
    std::fs::create_dir_all(&app_data_dir)?;
    let db_path = app_data_dir.join("vrcjournal.db");

    let database = db::Database::open(db_path)?;
    database.migrate()?;

    Ok(database)
}

/// Start log monitor in a background thread
fn start_log_monitor(database: db::Database, app_handle: tauri::AppHandle) {
    std::thread::spawn(move || {
        let mut monitor = Monitor::new(database);

        match monitor.initialize() {
            Ok(count) => println!("Monitor initialized: {} backlog events processed", count),
            Err(e) => {
                eprintln!("Failed to initialize monitor: {}", e);
                return;
            }
        }

        if let Err(e) = app_handle.emit("backend-ready", ()) {
            eprintln!("Failed to emit backend-ready event: {}", e);
            return;
        }

        monitor.run(app_handle);
    });
}
