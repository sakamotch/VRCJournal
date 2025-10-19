-- VRCJournal Database Schema

-- ============================================================================
-- Core User Management
-- ============================================================================

-- All VRChat users (both local accounts and remote players)
CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id TEXT NOT NULL UNIQUE,  -- VRChat user ID (usr_xxx)
    display_name TEXT NOT NULL,
    first_seen_at TEXT NOT NULL,
    last_seen_at TEXT NOT NULL
);

CREATE INDEX idx_users_user_id ON users(user_id);
CREATE INDEX idx_users_last_seen_at ON users(last_seen_at);

-- Track display name changes over time
CREATE TABLE user_name_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    display_name TEXT NOT NULL,
    first_seen_at TEXT NOT NULL,
    last_seen_at TEXT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX idx_user_name_history_user_id ON user_name_history(user_id);

-- Local accounts only (subset of users)
CREATE TABLE my_accounts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL UNIQUE,  -- References users.id
    first_authenticated_at TEXT NOT NULL,
    last_authenticated_at TEXT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- ============================================================================
-- World Management
-- ============================================================================

-- World master data
CREATE TABLE worlds (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    world_id TEXT NOT NULL UNIQUE,  -- VRChat world ID (wrld_xxx)
    world_name TEXT NOT NULL,
    first_seen_at TEXT NOT NULL,
    last_seen_at TEXT NOT NULL
);

CREATE INDEX idx_worlds_world_id ON worlds(world_id);
CREATE INDEX idx_worlds_last_seen_at ON worlds(last_seen_at);

-- Track world name changes over time
CREATE TABLE world_name_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    world_id INTEGER NOT NULL,
    world_name TEXT NOT NULL,
    first_seen_at TEXT NOT NULL,
    last_seen_at TEXT NOT NULL,
    FOREIGN KEY (world_id) REFERENCES worlds(id) ON DELETE CASCADE
);

CREATE INDEX idx_world_name_history_world_id ON world_name_history(world_id);

-- ============================================================================
-- Instance (Session) Management
-- ============================================================================

-- VRChat instance visits
CREATE TABLE instances (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    my_account_id INTEGER NOT NULL,  -- Which local account
    world_id INTEGER NOT NULL,
    world_name_at_join_id INTEGER,  -- Snapshot of world name at join time
    instance_id TEXT NOT NULL,  -- VRChat instance ID
    started_at TEXT NOT NULL,
    ended_at TEXT,
    status TEXT NOT NULL DEFAULT 'active',  -- 'active', 'completed', 'interrupted', 'error'
    FOREIGN KEY (my_account_id) REFERENCES my_accounts(id) ON DELETE CASCADE,
    FOREIGN KEY (world_id) REFERENCES worlds(id) ON DELETE CASCADE,
    FOREIGN KEY (world_name_at_join_id) REFERENCES world_name_history(id)
);

CREATE INDEX idx_instances_my_account_id ON instances(my_account_id);
CREATE INDEX idx_instances_world_id ON instances(world_id);
CREATE INDEX idx_instances_started_at ON instances(started_at DESC);
CREATE INDEX idx_instances_status ON instances(status);

-- Users present in instances (including local user)
CREATE TABLE instance_users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    instance_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    display_name_at_join_id INTEGER NOT NULL,  -- Snapshot of display name at join time
    joined_at TEXT NOT NULL,
    left_at TEXT,
    FOREIGN KEY (instance_id) REFERENCES instances(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (display_name_at_join_id) REFERENCES user_name_history(id)
);

CREATE INDEX idx_instance_users_instance_id ON instance_users(instance_id);
CREATE INDEX idx_instance_users_user_id ON instance_users(user_id);
CREATE INDEX idx_instance_users_joined_at ON instance_users(joined_at);

-- ============================================================================
-- Avatar Management
-- ============================================================================

-- Avatar master data
CREATE TABLE avatars (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    avatar_id TEXT UNIQUE,  -- VRChat avatar ID (avtr_xxx), nullable as it may not be available
    avatar_name TEXT NOT NULL,
    first_seen_at TEXT NOT NULL,
    last_seen_at TEXT NOT NULL
);

CREATE INDEX idx_avatars_avatar_id ON avatars(avatar_id);
CREATE INDEX idx_avatars_last_seen_at ON avatars(last_seen_at);

-- Avatar usage history (unified for local and remote users)
CREATE TABLE avatar_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    instance_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,  -- Who changed avatar (local or remote)
    avatar_id INTEGER NOT NULL,
    changed_at TEXT NOT NULL,
    FOREIGN KEY (instance_id) REFERENCES instances(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (avatar_id) REFERENCES avatars(id) ON DELETE CASCADE
);

CREATE INDEX idx_avatar_history_instance_id ON avatar_history(instance_id);
CREATE INDEX idx_avatar_history_user_id ON avatar_history(user_id);
CREATE INDEX idx_avatar_history_changed_at ON avatar_history(changed_at);

-- ============================================================================
-- Screenshot Management
-- ============================================================================

CREATE TABLE screenshots (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    instance_id INTEGER NOT NULL,
    file_path TEXT NOT NULL,
    taken_at TEXT NOT NULL,
    FOREIGN KEY (instance_id) REFERENCES instances(id) ON DELETE CASCADE
);

CREATE INDEX idx_screenshots_instance_id ON screenshots(instance_id);
CREATE INDEX idx_screenshots_taken_at ON screenshots(taken_at DESC);

-- ============================================================================
-- Log File Parsing State
-- ============================================================================

-- Track parsing progress to resume from last position on restart
CREATE TABLE log_files (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    file_path TEXT NOT NULL UNIQUE,
    file_size INTEGER NOT NULL,
    last_read_position INTEGER NOT NULL DEFAULT 0,  -- Byte offset for resuming
    last_modified_at TEXT,
    last_processed_at TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'active'  -- 'active', 'completed', 'archived'
);

CREATE INDEX idx_log_files_status ON log_files(status);
CREATE INDEX idx_log_files_last_processed_at ON log_files(last_processed_at DESC);
