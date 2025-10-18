use rusqlite::{Connection, Result};

const INITIAL_SCHEMA: &str = include_str!("../../migrations/001_initial_schema.sql");

/// マイグレーションを実行
pub fn run_migrations(conn: &Connection) -> Result<()> {
    // マイグレーション管理テーブルを作成
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS schema_migrations (
            version INTEGER PRIMARY KEY,
            applied_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
        );",
    )?;

    // 現在のバージョンを確認
    let current_version: i32 = conn
        .query_row(
            "SELECT COALESCE(MAX(version), 0) FROM schema_migrations",
            [],
            |row| row.get(0),
        )
        .unwrap_or(0);

    // マイグレーション001: 初期スキーマ
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
