use super::status::InstanceStatus;

/// プロセッサから発行されるイベント（フロントエンド利用）
/// UI更新のために設計されており、再クエリ不要
#[derive(Debug, Clone, serde::Serialize)]
#[serde(tag = "type")]
pub enum ProcessedEvent {
    /// Local user authenticated
    UserAuthenticated {
        my_account_id: i64,
        user_id: i64,
        display_name: String,
        vrchat_user_id: String,
    },

    /// Instance created (add new instance to list)
    InstanceCreated {
        instance_id: i64,
        my_account_id: i64,
        world_id: String,
        world_name: String,
        vrchat_instance_id: String,
        started_at: String,
        status: InstanceStatus,
    },

    /// Instance ended (update instance status)
    InstanceEnded {
        instance_id: i64,
        ended_at: String,
        status: InstanceStatus,
    },

    /// User joined instance (increment player count)
    UserJoined {
        instance_id: i64,
        instance_user_id: i64,
        user_id: i64,
        display_name: String,
        joined_at: String,
    },

    /// User left instance (decrement player count)
    UserLeft {
        instance_id: i64,
        instance_user_id: i64,
        left_at: String,
    },

    /// Avatar changed (update avatar in player list if visible)
    AvatarChanged {
        instance_id: i64,
        user_id: i64,
        display_name: String,
        avatar_id: i64,
        avatar_name: String,
        changed_at: String,
    },

    /// Screenshot taken (increment screenshot count)
    ScreenshotTaken {
        instance_id: i64,
        screenshot_id: i64,
        file_path: String,
        taken_at: String,
    },

    /// World name updated
    WorldNameUpdated {
        instance_id: i64,
        world_name: String,
        updated_at: String,
    },

    /// Instance sync failed
    InstanceSyncFailed {
        instance_id: i64,
        failed_at: String,
        status: InstanceStatus,
    },
}
