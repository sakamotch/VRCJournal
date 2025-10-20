use rusqlite::{Connection, Result};
use std::path::PathBuf;

pub struct Database {
    conn: Connection,
}

impl Database {
    /// Open database and enable foreign key constraints
    pub fn open(db_path: PathBuf) -> Result<Self> {
        let conn = Connection::open(db_path)?;

        conn.execute_batch("PRAGMA foreign_keys = ON;")?;

        Ok(Database { conn })
    }

    /// Get database connection reference
    pub fn connection(&self) -> &Connection {
        &self.conn
    }

    /// Run database migrations
    pub fn migrate(&self) -> Result<()> {
        super::migrations::run_migrations(&self.conn)
    }
}
