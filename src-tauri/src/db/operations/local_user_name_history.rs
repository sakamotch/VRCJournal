use rusqlite::{Connection, Result, OptionalExtension};
use chrono::{DateTime, Utc};

/// ローカルユーザーの名前履歴を作成または更新
pub fn upsert_local_user_name_history(
    conn: &Connection,
    local_user_id: i64,
    display_name: &str,
    seen_at: DateTime<Utc>,
) -> Result<i64> {
    // 既存の名前履歴を確認
    let existing: Option<i64> = conn
        .query_row(
            "SELECT id FROM local_user_name_history WHERE local_user_id = ?1 AND display_name = ?2",
            (local_user_id, display_name),
            |row| row.get(0),
        )
        .optional()?;

    if let Some(id) = existing {
        // 既存の名前履歴のlast_seen_atを更新
        conn.execute(
            "UPDATE local_user_name_history SET last_seen_at = ?1 WHERE id = ?2",
            (seen_at.to_rfc3339(), id),
        )?;
        Ok(id)
    } else {
        // 新しい名前履歴を作成
        conn.execute(
            "INSERT INTO local_user_name_history (local_user_id, display_name, first_seen_at, last_seen_at)
             VALUES (?1, ?2, ?3, ?4)",
            (
                local_user_id,
                display_name,
                seen_at.to_rfc3339(),
                seen_at.to_rfc3339(),
            ),
        )?;
        Ok(conn.last_insert_rowid())
    }
}
