use chrono::{DateTime, Utc};

/// ログから抽出されるイベントの種類
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
    /// プレイヤー退出
    PlayerLeft {
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
}

impl LogEvent {
    pub fn timestamp(&self) -> &DateTime<Utc> {
        match self {
            LogEvent::UserAuthenticated { timestamp, .. } => timestamp,
            LogEvent::JoiningWorld { timestamp, .. } => timestamp,
            LogEvent::EnteringRoom { timestamp, .. } => timestamp,
            LogEvent::PlayerJoined { timestamp, .. } => timestamp,
            LogEvent::PlayerLeft { timestamp, .. } => timestamp,
            LogEvent::AvatarChanged { timestamp, .. } => timestamp,
        }
    }
}
