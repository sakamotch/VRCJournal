use super::status::InstanceStatus;

/// Events emitted to frontend for UI updates
#[derive(Debug, Clone, serde::Serialize)]
#[serde(tag = "type")]
pub enum ProcessedEvent {
    UserAuthenticated {
        my_account_id: i64,
        user_id: i64,
        display_name: String,
        vrchat_user_id: String,
    },

    InstanceCreated {
        instance_id: i64,
        my_account_id: i64,
        world_id: String,
        world_name: String,
        vrchat_instance_id: String,
        started_at: String,
        status: InstanceStatus,
    },

    InstanceEnded {
        instance_id: i64,
        ended_at: String,
        status: InstanceStatus,
    },

    UserJoined {
        instance_id: i64,
        instance_user_id: i64,
        user_id: i64,
        display_name: String,
        joined_at: String,
    },

    UserLeft {
        instance_id: i64,
        instance_user_id: i64,
        left_at: String,
    },

    AvatarChanged {
        instance_id: i64,
        user_id: i64,
        display_name: String,
        avatar_id: i64,
        avatar_name: String,
        changed_at: String,
    },

    ScreenshotTaken {
        instance_id: i64,
        screenshot_id: i64,
        file_path: String,
        taken_at: String,
    },

    WorldNameUpdated {
        instance_id: i64,
        world_name: String,
        updated_at: String,
    },

    InstanceSyncFailed {
        instance_id: i64,
        failed_at: String,
        status: InstanceStatus,
    },
}
