-- VRCJournal Initial Schema
-- Created: 2025-10-13
-- Updated: 2025-10-14 - Unified players and local_users

-- 1. Players (全VRChatユーザー: 自分のアカウント + 他のプレイヤー)
CREATE TABLE players (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id TEXT NOT NULL UNIQUE,
    display_name TEXT NOT NULL,
    is_local BOOLEAN NOT NULL DEFAULT 0,   -- 自分のアカウントか (1=自分, 0=他人)
    first_seen_at TEXT NOT NULL,           -- ISO 8601 / RFC3339形式
    last_seen_at TEXT NOT NULL,            -- ISO 8601 / RFC3339形式
    first_authenticated_at TEXT,           -- is_local=1の場合のみ使用
    last_authenticated_at TEXT             -- is_local=1の場合のみ使用
);
CREATE INDEX idx_players_user_id ON players(user_id);
CREATE INDEX idx_players_display_name ON players(display_name);
CREATE INDEX idx_players_is_local ON players(is_local);

-- 2. Player Name History (全ユーザーの名前変更履歴)
CREATE TABLE player_name_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    player_id INTEGER NOT NULL,
    display_name TEXT NOT NULL,
    first_seen_at TEXT NOT NULL,           -- ISO 8601 / RFC3339形式
    last_seen_at TEXT NOT NULL,            -- ISO 8601 / RFC3339形式
    FOREIGN KEY (player_id) REFERENCES players(id) ON DELETE CASCADE
);
CREATE INDEX idx_player_name_history_player ON player_name_history(player_id);
CREATE INDEX idx_player_name_history_seen_at ON player_name_history(first_seen_at);

-- 3. Sessions (セッション情報)
CREATE TABLE sessions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    player_id INTEGER NOT NULL,            -- セッションを開始したプレイヤー (is_local=1のplayer)
    started_at TEXT NOT NULL,              -- ISO 8601 / RFC3339形式（日付検索用）
    ended_at TEXT,                         -- ISO 8601 / RFC3339形式
    world_id TEXT NOT NULL,
    world_name TEXT,
    instance_id TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'active', -- active: 進行中, completed: 正常終了, interrupted: 異常終了
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (player_id) REFERENCES players(id) ON DELETE CASCADE
);
CREATE INDEX idx_sessions_player ON sessions(player_id);
CREATE INDEX idx_sessions_started_at ON sessions(started_at);  -- 日付範囲検索用
CREATE INDEX idx_sessions_world_id ON sessions(world_id);
CREATE INDEX idx_sessions_player_started ON sessions(player_id, started_at DESC);  -- 複合インデックス（ユーザー別時系列）

-- 4. Avatars (アバター情報)
CREATE TABLE avatars (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    avatar_id TEXT UNIQUE,
    avatar_name TEXT NOT NULL,
    first_seen_at TEXT NOT NULL,           -- ISO 8601 / RFC3339形式
    last_seen_at TEXT NOT NULL             -- ISO 8601 / RFC3339形式
);
CREATE INDEX idx_avatars_avatar_id ON avatars(avatar_id);
CREATE INDEX idx_avatars_name ON avatars(avatar_name);

-- 5. Session Players (セッションとプレイヤーの関連)
CREATE TABLE session_players (
    session_id INTEGER NOT NULL,
    player_id INTEGER NOT NULL,
    joined_at TEXT NOT NULL,               -- ISO 8601 / RFC3339形式
    left_at TEXT,                          -- ISO 8601 / RFC3339形式
    display_name_history_id INTEGER NOT NULL,
    PRIMARY KEY (session_id, player_id),
    FOREIGN KEY (session_id) REFERENCES sessions(id) ON DELETE CASCADE,
    FOREIGN KEY (player_id) REFERENCES players(id) ON DELETE CASCADE,
    FOREIGN KEY (display_name_history_id) REFERENCES player_name_history(id) ON DELETE RESTRICT
);
CREATE INDEX idx_session_players_session ON session_players(session_id);
CREATE INDEX idx_session_players_player ON session_players(player_id);

-- 6. Avatar Usages (アバター使用履歴)
-- セッション内での各プレイヤー（自分含む）のアバター変更を記録
CREATE TABLE avatar_usages (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    session_id INTEGER NOT NULL,
    player_id INTEGER NOT NULL,            -- アバターを変更したプレイヤー（自分も含む）
    avatar_id INTEGER NOT NULL,            -- avatarsテーブルへの参照
    changed_at TEXT NOT NULL,              -- ISO 8601 / RFC3339形式
    FOREIGN KEY (session_id) REFERENCES sessions(id) ON DELETE CASCADE,
    FOREIGN KEY (player_id) REFERENCES players(id) ON DELETE CASCADE,
    FOREIGN KEY (avatar_id) REFERENCES avatars(id) ON DELETE CASCADE
);
CREATE INDEX idx_avatar_usages_session ON avatar_usages(session_id);
CREATE INDEX idx_avatar_usages_player ON avatar_usages(player_id);
CREATE INDEX idx_avatar_usages_avatar ON avatar_usages(avatar_id);
CREATE INDEX idx_avatar_usages_session_changed ON avatar_usages(session_id, changed_at);  -- セッション内時系列

-- 7. Tags (タグマスター)
CREATE TABLE tags (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    color TEXT,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 8. Session Tags (セッションとタグの関連)
CREATE TABLE session_tags (
    session_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    PRIMARY KEY (session_id, tag_id),
    FOREIGN KEY (session_id) REFERENCES sessions(id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
);

-- 9. Memos (メモ: 1セッション1メモ)
CREATE TABLE memos (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    session_id INTEGER NOT NULL UNIQUE,   -- UNIQUE制約: 1セッション1メモ
    content TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (session_id) REFERENCES sessions(id) ON DELETE CASCADE
);

-- 10. Log Files (ログファイルの解析状態を追跡)
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

-- 11. Screenshots (スクリーンショット記録)
CREATE TABLE IF NOT EXISTS screenshots (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    session_id INTEGER NOT NULL,
    file_path TEXT NOT NULL,
    taken_at TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (session_id) REFERENCES sessions(id) ON DELETE CASCADE
);
CREATE INDEX IF NOT EXISTS idx_screenshots_session ON screenshots(session_id);
CREATE INDEX IF NOT EXISTS idx_screenshots_taken_at ON screenshots(taken_at);
