use rusqlite::{Connection, Result};
use std::path::PathBuf;

pub struct Database {
    conn: Connection,
}

impl Database {
    /// データベースを開く（存在しない場合は作成）
    pub fn open(db_path: PathBuf) -> Result<Self> {
        let conn = Connection::open(db_path)?;

        // Foreign key制約を有効化
        conn.execute_batch("PRAGMA foreign_keys = ON;")?;

        Ok(Database { conn })
    }

    /// データベース接続の参照を取得
    pub fn connection(&self) -> &Connection {
        &self.conn
    }

    /// マイグレーションを実行
    pub fn migrate(&self) -> Result<()> {
        super::migrations::run_migrations(&self.conn)
    }
}
