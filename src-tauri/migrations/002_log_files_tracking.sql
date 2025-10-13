-- ログファイルの解析状態を追跡するテーブル
CREATE TABLE IF NOT EXISTS log_files (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    file_path TEXT NOT NULL UNIQUE,
    file_size INTEGER NOT NULL DEFAULT 0,
    last_processed_position INTEGER NOT NULL DEFAULT 0,
    last_modified_at TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- インデックス
CREATE INDEX IF NOT EXISTS idx_log_files_file_path ON log_files(file_path);
CREATE INDEX IF NOT EXISTS idx_log_files_last_modified ON log_files(last_modified_at);
