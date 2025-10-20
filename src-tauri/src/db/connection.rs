use rusqlite::{Connection, Result, Transaction};
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

    /// Begin a new transaction
    pub fn transaction(&mut self) -> Result<Transaction<'_>> {
        self.conn.transaction()
    }

    /// Run database migrations
    pub fn migrate(&self) -> Result<()> {
        super::migrations::run_migrations(&self.conn)
    }
}
