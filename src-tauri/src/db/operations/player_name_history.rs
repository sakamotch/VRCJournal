use chrono::{DateTime, Utc};
use rusqlite::{Connection, OptionalExtension, Result};

#[derive(Debug, Clone)]
pub struct PlayerNameHistory {
    pub id: i64,
    pub player_id: i64,
    pub display_name: String,
    pub first_seen_at: DateTime<Utc>,
    pub last_seen_at: DateTime<Utc>,
}

/// プレイヤーの名前履歴を作成または更新
/// 同じplayer_idで同じdisplay_nameの履歴があればlast_seen_atを更新、なければ新規作成
pub fn upsert_player_name_history(
    conn: &Connection,
    player_id: i64,
    display_name: &str,
    seen_at: DateTime<Utc>,
) -> Result<i64> {
    // 既存の名前履歴を確認（同じplayer_idで同じdisplay_name）
    let existing: Option<i64> = conn
        .query_row(
            "SELECT id FROM player_name_history WHERE player_id = ?1 AND display_name = ?2",
            (player_id, display_name),
            |row| row.get(0),
        )
        .optional()?;

    if let Some(id) = existing {
        // 既存の名前履歴のlast_seen_atを更新
        conn.execute(
            "UPDATE player_name_history SET last_seen_at = ?1 WHERE id = ?2",
            (seen_at.to_rfc3339(), id),
        )?;
        Ok(id)
    } else {
        // 新しい名前履歴を作成
        conn.execute(
            "INSERT INTO player_name_history (player_id, display_name, first_seen_at, last_seen_at)
             VALUES (?1, ?2, ?3, ?4)",
            (
                player_id,
                display_name,
                seen_at.to_rfc3339(),
                seen_at.to_rfc3339(),
            ),
        )?;
        Ok(conn.last_insert_rowid())
    }
}

/// プレイヤーの全名前履歴を取得（新しい順）
pub fn get_player_name_history(
    conn: &Connection,
    player_id: i64,
) -> Result<Vec<PlayerNameHistory>> {
    let mut stmt = conn.prepare(
        "SELECT id, player_id, display_name, first_seen_at, last_seen_at
         FROM player_name_history
         WHERE player_id = ?1
         ORDER BY first_seen_at DESC",
    )?;

    let history = stmt
        .query_map([player_id], |row| {
            Ok(PlayerNameHistory {
                id: row.get(0)?,
                player_id: row.get(1)?,
                display_name: row.get(2)?,
                first_seen_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(3)?)
                    .unwrap()
                    .with_timezone(&Utc),
                last_seen_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(4)?)
                    .unwrap()
                    .with_timezone(&Utc),
            })
        })?
        .collect::<Result<Vec<_>>>()?;

    Ok(history)
}

/// 指定した名前履歴IDから表示名を取得
pub fn get_display_name_by_history_id(conn: &Connection, history_id: i64) -> Result<String> {
    conn.query_row(
        "SELECT display_name FROM player_name_history WHERE id = ?1",
        [history_id],
        |row| row.get(0),
    )
}
