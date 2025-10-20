use chrono::{DateTime, Utc};

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
