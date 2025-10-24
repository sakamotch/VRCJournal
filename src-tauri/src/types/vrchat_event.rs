use super::status::InstanceStatus;

/// VRChat events emitted to frontend for UI updates
#[derive(Debug, Clone, serde::Serialize)]
#[serde(tag = "type")]
pub enum VRChatEvent {
    UserAuthenticated {
        my_account_id: i64,
        user_id: i64,
        display_name: String,
        vrchat_user_id: String,
    },

    InstanceStarted {
        instance_id: i64,
        my_account_id: i64,
        world_id: String,
        vrchat_instance_id: String,
        started_at: i64,  // Unix timestamp in milliseconds
        status: InstanceStatus,
    },

    InstanceEnded {
        instance_id: i64,
        ended_at: i64,  // Unix timestamp in milliseconds
        status: InstanceStatus,
    },

    UserJoined {
        instance_id: i64,
        instance_user_id: i64,
        user_id: i64,
        display_name: String,
        joined_at: i64,  // Unix timestamp in milliseconds
        initial_avatar_id: Option<i64>,
        initial_avatar_name: Option<String>,
    },

    UserLeft {
        instance_id: i64,
        instance_user_id: i64,
        left_at: i64,  // Unix timestamp in milliseconds
    },

    AvatarChanged {
        instance_id: i64,
        user_id: i64,
        display_name: String,
        avatar_id: i64,
        avatar_name: String,
        changed_at: i64,  // Unix timestamp in milliseconds
    },

    ScreenshotTaken {
        instance_id: i64,
        screenshot_id: i64,
        file_path: String,
        taken_at: i64,  // Unix timestamp in milliseconds
    },

    WorldNameUpdated {
        instance_id: i64,
        world_name: String,
        updated_at: i64,  // Unix timestamp in milliseconds
    },

    InstanceSyncFailed {
        instance_id: i64,
        failed_at: i64,  // Unix timestamp in milliseconds
        status: InstanceStatus,
    },
}
