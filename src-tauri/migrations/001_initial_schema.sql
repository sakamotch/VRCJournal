-- VRCJournal Initial Schema
-- Created: 2025-10-13

-- 1. Local users (複数アカウント対応)
CREATE TABLE local_users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    display_name TEXT NOT NULL,
    user_id TEXT NOT NULL UNIQUE,
    first_authenticated_at TEXT NOT NULL,
    last_authenticated_at TEXT NOT NULL
);
CREATE INDEX idx_local_users_user_id ON local_users(user_id);

-- 2. Sessions (セッション情報)
CREATE TABLE sessions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    local_user_id INTEGER NOT NULL,
    started_at TEXT NOT NULL,
    ended_at TEXT,
    world_id TEXT NOT NULL,
    world_name TEXT,
    instance_id TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (local_user_id) REFERENCES local_users(id) ON DELETE CASCADE
);
CREATE INDEX idx_sessions_local_user ON sessions(local_user_id);
CREATE INDEX idx_sessions_started_at ON sessions(started_at);
CREATE INDEX idx_sessions_world_id ON sessions(world_id);

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

-- 5. Session Players (セッションとプレイヤーの関連)
CREATE TABLE session_players (
    session_id INTEGER NOT NULL,
    player_id INTEGER NOT NULL,
    joined_at TEXT NOT NULL,
    leaved_at TEXT,
    PRIMARY KEY (session_id, player_id),
    FOREIGN KEY (session_id) REFERENCES sessions(id) ON DELETE CASCADE,
    FOREIGN KEY (player_id) REFERENCES players(id) ON DELETE CASCADE
);
CREATE INDEX idx_session_players_session ON session_players(session_id);
CREATE INDEX idx_session_players_player ON session_players(player_id);

-- 6. Avatar Usages (アバター使用履歴)
CREATE TABLE avatar_usages (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    session_id INTEGER NOT NULL,
    user_id TEXT NOT NULL,
    avatar_id INTEGER,
    avatar_name TEXT NOT NULL,
    changed_at TEXT NOT NULL,
    FOREIGN KEY (session_id) REFERENCES sessions(id) ON DELETE CASCADE,
    FOREIGN KEY (avatar_id) REFERENCES avatars(id) ON DELETE SET NULL
);
CREATE INDEX idx_avatar_usages_session ON avatar_usages(session_id);
CREATE INDEX idx_avatar_usages_user ON avatar_usages(user_id);
CREATE INDEX idx_avatar_usages_avatar ON avatar_usages(avatar_id);

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

-- 9. Memos (メモ)
CREATE TABLE memos (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    session_id INTEGER NOT NULL,
    content TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (session_id) REFERENCES sessions(id) ON DELETE CASCADE
);
CREATE INDEX idx_memos_session ON memos(session_id);
