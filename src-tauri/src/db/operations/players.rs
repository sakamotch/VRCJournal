use rusqlite::{Connection, Result, OptionalExtension};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Player {
    pub id: i64,
    pub display_name: String,
    pub user_id: String,
    pub first_seen_at: DateTime<Utc>,
    pub last_seen_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct SessionPlayer {
    pub id: i64,
    pub display_name: String,             // 現在の表示名
    pub display_name_at_join: String,     // その時の表示名
    pub user_id: String,
    pub first_seen_at: DateTime<Utc>,
    pub last_seen_at: DateTime<Utc>,
}

/// プレイヤーを作成または更新
pub fn upsert_player(
    conn: &Connection,
    display_name: &str,
    user_id: &str,
    seen_at: DateTime<Utc>,
) -> Result<i64> {
    // 既存のプレイヤーを確認
    let existing: Option<i64> = conn
        .query_row(
            "SELECT id FROM players WHERE user_id = ?1",
            [user_id],
            |row| row.get(0),
        )
        .optional()?;

    if let Some(id) = existing {
        // 既存プレイヤーの最終確認時刻と表示名を更新
        conn.execute(
            "UPDATE players SET last_seen_at = ?1, display_name = ?2 WHERE id = ?3",
            (seen_at.to_rfc3339(), display_name, id),
        )?;
        Ok(id)
    } else {
        // 新規プレイヤーを作成
        conn.execute(
            "INSERT INTO players (display_name, user_id, first_seen_at, last_seen_at)
             VALUES (?1, ?2, ?3, ?4)",
            (
                display_name,
                user_id,
                seen_at.to_rfc3339(),
                seen_at.to_rfc3339(),
            ),
        )?;
        Ok(conn.last_insert_rowid())
    }
}

/// セッションにプレイヤーを追加
pub fn add_player_to_session(
    conn: &Connection,
    session_id: i64,
    player_id: i64,
    display_name_history_id: i64,
    joined_at: DateTime<Utc>,
) -> Result<()> {
    conn.execute(
        "INSERT OR IGNORE INTO session_players (session_id, player_id, joined_at, display_name_history_id)
         VALUES (?1, ?2, ?3, ?4)",
        (session_id, player_id, joined_at.to_rfc3339(), display_name_history_id),
    )?;
    Ok(())
}

/// セッションからプレイヤーを退出させる
pub fn remove_player_from_session(
    conn: &Connection,
    session_id: i64,
    player_id: i64,
    left_at: DateTime<Utc>,
) -> Result<()> {
    conn.execute(
        "UPDATE session_players SET left_at = ?1 WHERE session_id = ?2 AND player_id = ?3",
        (left_at.to_rfc3339(), session_id, player_id),
    )?;
    Ok(())
}

/// セッションのプレイヤー一覧を取得（その時の名前と現在の名前を含む）
pub fn get_players_in_session(conn: &Connection, session_id: i64) -> Result<Vec<SessionPlayer>> {
    let mut stmt = conn.prepare(
        "SELECT p.id, p.display_name, pnh.display_name, p.user_id, p.first_seen_at, p.last_seen_at
         FROM players p
         INNER JOIN session_players sp ON p.id = sp.player_id
         INNER JOIN player_name_history pnh ON sp.display_name_history_id = pnh.id
         WHERE sp.session_id = ?1
         ORDER BY sp.joined_at",
    )?;

    let players = stmt
        .query_map([session_id], |row| {
            Ok(SessionPlayer {
                id: row.get(0)?,
                display_name: row.get(1)?,              // 現在の名前
                display_name_at_join: row.get(2)?,      // その時の名前
                user_id: row.get(3)?,
                first_seen_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(4)?)
                    .unwrap()
                    .with_timezone(&Utc),
                last_seen_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(5)?)
                    .unwrap()
                    .with_timezone(&Utc),
            })
        })?
        .collect::<Result<Vec<_>>>()?;

    Ok(players)
}
