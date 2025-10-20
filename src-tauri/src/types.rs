use chrono::{DateTime, Utc};

/// Instance status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstanceStatus {
    /// Instance is active (user is in the world)
    Active,
    /// Instance completed normally (user left intentionally)
    Completed,
    /// Instance was interrupted (VRChat crashed or unexpected termination)
    Interrupted,
    /// Instance encountered sync failure (network synchronization error)
    SyncFailed,
}

impl InstanceStatus {
    /// Convert to database string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Completed => "completed",
            Self::Interrupted => "interrupted",
            Self::SyncFailed => "sync_failed",
        }
    }

    /// Parse from database string representation
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "active" => Ok(Self::Active),
            "completed" => Ok(Self::Completed),
            "interrupted" => Ok(Self::Interrupted),
            "sync_failed" => Ok(Self::SyncFailed),
            _ => Err(format!("Unknown instance status: {}", s)),
        }
    }
}

impl Default for InstanceStatus {
    fn default() -> Self {
        Self::Active
    }
}

// Serialize for sending to frontend
impl serde::Serialize for InstanceStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

// Deserialize for receiving from frontend (if needed)
impl<'de> serde::Deserialize<'de> for InstanceStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::from_str(&s).map_err(serde::de::Error::custom)
    }
}

/// ログから抽出されるイベントの種類（内部利用）
#[derive(Debug, Clone, PartialEq)]
pub enum LogEvent {
    /// ユーザー認証
    UserAuthenticated {
        timestamp: DateTime<Utc>,
        display_name: String,
        user_id: String,
    },
    /// ワールド/インスタンス入室
    JoiningWorld {
        timestamp: DateTime<Utc>,
        world_name: String,
        world_id: String,
        instance_id: String,
    },
    /// ルーム名取得 (Joining or Creating Room)
    EnteringRoom {
        timestamp: DateTime<Utc>,
        world_name: String,
    },
    /// プレイヤー参加
    PlayerJoined {
        timestamp: DateTime<Utc>,
        display_name: String,
        user_id: String,
    },
    /// アバター変更
    AvatarChanged {
        timestamp: DateTime<Utc>,
        display_name: String,
        avatar_name: String,
    },
    /// スクリーンショット撮影
    ScreenshotTaken {
        timestamp: DateTime<Utc>,
        file_path: String,
    },
    /// プレイヤーのDisplayName破棄（退出の最終段階）
    DestroyingPlayer {
        timestamp: DateTime<Utc>,
        display_name: String,
    },
    /// イベント同期失敗（マスターからイベントが送信されない）
    /// 次のJoiningWorldイベントが代替インスタンスとなる
    EventSyncFailed { timestamp: DateTime<Utc> },
}

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
