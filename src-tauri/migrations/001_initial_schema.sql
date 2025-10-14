-- VRCJournal Initial Schema
-- Created: 2025-10-13

-- 1. Local users (複数アカウント対応)
CREATE TABLE local_users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    display_name TEXT NOT NULL,
    user_id TEXT NOT NULL UNIQUE,
    first_authenticated_at TEXT NOT NULL,  -- ISO 8601 / RFC3339形式
    last_authenticated_at TEXT NOT NULL    -- ISO 8601 / RFC3339形式
);
CREATE INDEX idx_local_users_user_id ON local_users(user_id);

-- 1-1. Local User Name History (自分のアカウントの名前変更履歴)
CREATE TABLE local_user_name_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    local_user_id INTEGER NOT NULL,
    display_name TEXT NOT NULL,
    first_seen_at TEXT NOT NULL,           -- ISO 8601 / RFC3339形式
    last_seen_at TEXT NOT NULL,            -- ISO 8601 / RFC3339形式
    FOREIGN KEY (local_user_id) REFERENCES local_users(id) ON DELETE CASCADE
);
CREATE INDEX idx_local_user_name_history_user ON local_user_name_history(local_user_id);
CREATE INDEX idx_local_user_name_history_seen_at ON local_user_name_history(first_seen_at);

-- 2. Sessions (セッション情報)
CREATE TABLE sessions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    local_user_id INTEGER NOT NULL,
    started_at TEXT NOT NULL,              -- ISO 8601 / RFC3339形式（日付検索用）
    ended_at TEXT,                         -- ISO 8601 / RFC3339形式
    world_id TEXT NOT NULL,
    world_name TEXT,
    instance_id TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (local_user_id) REFERENCES local_users(id) ON DELETE CASCADE
);
CREATE INDEX idx_sessions_local_user ON sessions(local_user_id);
CREATE INDEX idx_sessions_started_at ON sessions(started_at);  -- 日付範囲検索用
CREATE INDEX idx_sessions_world_id ON sessions(world_id);
CREATE INDEX idx_sessions_user_started ON sessions(local_user_id, started_at DESC);  -- 複合インデックス（ユーザー別時系列）

-- 3. Avatars (アバター情報)
CREATE TABLE avatars (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    avatar_id TEXT UNIQUE,
    avatar_name TEXT NOT NULL,
    first_seen_at TEXT NOT NULL,
    last_seen_at TEXT NOT NULL
);
CREATE INDEX idx_avatars_avatar_id ON avatars(avatar_id);
CREATE INDEX idx_avatars_name ON avatars(avatar_name);

-- 4. Players (プレイヤー情報)
CREATE TABLE players (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    display_name TEXT NOT NULL,
    user_id TEXT NOT NULL UNIQUE,
    first_seen_at TEXT NOT NULL,
    last_seen_at TEXT NOT NULL
);
CREATE INDEX idx_players_user_id ON players(user_id);
CREATE INDEX idx_players_display_name ON players(display_name);

-- 5. Player Name History (プレイヤー名変更履歴)
CREATE TABLE player_name_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    player_id INTEGER NOT NULL,
    display_name TEXT NOT NULL,
    first_seen_at TEXT NOT NULL,
    last_seen_at TEXT NOT NULL,
    FOREIGN KEY (player_id) REFERENCES players(id) ON DELETE CASCADE
);
CREATE INDEX idx_player_name_history_player ON player_name_history(player_id);
CREATE INDEX idx_player_name_history_seen_at ON player_name_history(first_seen_at);

-- 6. Session Players (セッションとプレイヤーの関連)
CREATE TABLE session_players (
    session_id INTEGER NOT NULL,
    player_id INTEGER NOT NULL,
    joined_at TEXT NOT NULL,
    left_at TEXT,
    display_name_history_id INTEGER NOT NULL,
    PRIMARY KEY (session_id, player_id),
    FOREIGN KEY (session_id) REFERENCES sessions(id) ON DELETE CASCADE,
    FOREIGN KEY (player_id) REFERENCES players(id) ON DELETE CASCADE,
    FOREIGN KEY (display_name_history_id) REFERENCES player_name_history(id) ON DELETE RESTRICT
);
CREATE INDEX idx_session_players_session ON session_players(session_id);
CREATE INDEX idx_session_players_player ON session_players(player_id);

-- 7. Avatar Usages (アバター使用履歴)
-- セッション内での各プレイヤーのアバター変更を記録
-- 自分（local_user）のアバター変更も含む
CREATE TABLE avatar_usages (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    session_id INTEGER NOT NULL,
    player_id INTEGER,                     -- NULLの場合は自分（local_user）のアバター
    avatar_id INTEGER,                     -- avatarsテーブルへの参照
    changed_at TEXT NOT NULL,              -- ISO 8601 / RFC3339形式
    FOREIGN KEY (session_id) REFERENCES sessions(id) ON DELETE CASCADE,
    FOREIGN KEY (player_id) REFERENCES players(id) ON DELETE CASCADE,
    FOREIGN KEY (avatar_id) REFERENCES avatars(id) ON DELETE SET NULL
);
CREATE INDEX idx_avatar_usages_session ON avatar_usages(session_id);
CREATE INDEX idx_avatar_usages_player ON avatar_usages(player_id);
CREATE INDEX idx_avatar_usages_avatar ON avatar_usages(avatar_id);
CREATE INDEX idx_avatar_usages_session_changed ON avatar_usages(session_id, changed_at);  -- セッション内時系列

-- 8. Tags (タグマスター)
CREATE TABLE tags (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    color TEXT,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 9. Session Tags (セッションとタグの関連)
CREATE TABLE session_tags (
    session_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    PRIMARY KEY (session_id, tag_id),
    FOREIGN KEY (session_id) REFERENCES sessions(id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
);

-- 10. Memos (メモ)
CREATE TABLE memos (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    session_id INTEGER NOT NULL,
    content TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (session_id) REFERENCES sessions(id) ON DELETE CASCADE
);
CREATE INDEX idx_memos_session ON memos(session_id);

-- 11. Log Files (ログファイルの解析状態を追跡)
CREATE TABLE IF NOT EXISTS log_files (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    file_path TEXT NOT NULL UNIQUE,
    file_size INTEGER NOT NULL DEFAULT 0,
    last_processed_position INTEGER NOT NULL DEFAULT 0,
    last_modified_at TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_log_files_file_path ON log_files(file_path);
CREATE INDEX IF NOT EXISTS idx_log_files_last_modified ON log_files(last_modified_at);
