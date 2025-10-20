use rusqlite::{Connection, Result};

const INITIAL_SCHEMA: &str = include_str!("../../migrations/001_initial_schema.sql");

/// Run database migrations
pub fn run_migrations(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS schema_migrations (
            version INTEGER PRIMARY KEY,
            applied_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
        );",
    )?;

    let current_version: i32 = conn
        .query_row(
            "SELECT COALESCE(MAX(version), 0) FROM schema_migrations",
            [],
            |row| row.get(0),
        )
        .unwrap_or(0);

    if current_version < 1 {
        println!("Running migration 001: Initial schema");
        conn.execute_batch(INITIAL_SCHEMA)?;
        conn.execute("INSERT INTO schema_migrations (version) VALUES (1)", [])?;
    }

    println!(
        "Database migrations complete. Current version: {}",
        current_version.max(1)
    );
    Ok(())
}
