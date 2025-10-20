use chrono::{DateTime, Utc};

/// Events extracted from VRChat logs
#[derive(Debug, Clone, PartialEq)]
pub enum LogEvent {
    UserAuthenticated {
        timestamp: DateTime<Utc>,
        display_name: String,
        user_id: String,
    },
    JoiningWorld {
        timestamp: DateTime<Utc>,
        world_name: String,
        world_id: String,
        instance_id: String,
    },
    EnteringRoom {
        timestamp: DateTime<Utc>,
        world_name: String,
    },
    PlayerJoined {
        timestamp: DateTime<Utc>,
        display_name: String,
        user_id: String,
    },
    AvatarChanged {
        timestamp: DateTime<Utc>,
        display_name: String,
        avatar_name: String,
    },
    ScreenshotTaken {
        timestamp: DateTime<Utc>,
        file_path: String,
    },
    DestroyingPlayer {
        timestamp: DateTime<Utc>,
        display_name: String,
    },
    EventSyncFailed {
        timestamp: DateTime<Utc>,
    },
}
