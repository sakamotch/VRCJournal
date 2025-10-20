use crate::{db, log_monitor::Monitor};
use std::time::Duration;
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

        // Initialize monitor
        if let Err(e) = monitor.initialize() {
            eprintln!("Failed to initialize monitor: {}", e);
            return;
        }

        // Signal that backend is ready
        if let Err(e) = app_handle.emit("backend-ready", ()) {
            eprintln!("Failed to emit backend-ready event: {}", e);
            return;
        }

        // Real-time monitoring loop
        loop {
            std::thread::sleep(Duration::from_millis(1000));

            match monitor.fetch_new_events() {
                Ok(events) => {
                    for event in events {
                        if let Err(e) = app_handle.emit("log-event", &event) {
                            eprintln!("Failed to emit event: {}", e);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to fetch new events: {}", e);
                }
            }
        }
    });
}
