use rusqlite::{Connection, Result, OptionalExtension};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Session {
    pub id: i64,
    pub local_user_id: i64,
    pub started_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
    pub world_id: String,
    pub world_name: Option<String>,
    pub instance_id: String,
}

/// 新しいセッションを作成
pub fn create_session(
    conn: &Connection,
    local_user_id: i64,
    started_at: DateTime<Utc>,
    world_id: &str,
    world_name: Option<&str>,
    instance_id: &str,
) -> Result<i64> {
    conn.execute(
        "INSERT INTO sessions (local_user_id, started_at, world_id, world_name, instance_id)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        (
            local_user_id,
            started_at.to_rfc3339(),
            world_id,
            world_name,
            instance_id,
        ),
    )?;
    Ok(conn.last_insert_rowid())
}

/// セッションの終了時刻を更新
pub fn end_session(conn: &Connection, session_id: i64, ended_at: DateTime<Utc>) -> Result<()> {
    conn.execute(
        "UPDATE sessions SET ended_at = ?1 WHERE id = ?2",
        (ended_at.to_rfc3339(), session_id),
    )?;
    Ok(())
}

/// 最新の未終了セッションを取得
pub fn get_latest_active_session(
    conn: &Connection,
    local_user_id: i64,
) -> Result<Option<Session>> {
    conn.query_row(
        "SELECT id, local_user_id, started_at, ended_at, world_id, world_name, instance_id
         FROM sessions
         WHERE local_user_id = ?1 AND ended_at IS NULL
         ORDER BY started_at DESC
         LIMIT 1",
        [local_user_id],
        |row| {
            Ok(Session {
                id: row.get(0)?,
                local_user_id: row.get(1)?,
                started_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(2)?)
                    .unwrap()
                    .with_timezone(&Utc),
                ended_at: row
                    .get::<_, Option<String>>(3)?
                    .map(|s| DateTime::parse_from_rfc3339(&s).unwrap().with_timezone(&Utc)),
                world_id: row.get(4)?,
                world_name: row.get(5)?,
                instance_id: row.get(6)?,
            })
        },
    )
    .optional()
}

/// 指定したローカルユーザーのセッション一覧を取得
pub fn get_sessions_by_local_user(
    conn: &Connection,
    local_user_id: i64,
    limit: usize,
    offset: usize,
) -> Result<Vec<Session>> {
    let mut stmt = conn.prepare(
        "SELECT id, local_user_id, started_at, ended_at, world_id, world_name, instance_id
         FROM sessions
         WHERE local_user_id = ?1
         ORDER BY started_at DESC
         LIMIT ?2 OFFSET ?3",
    )?;

    let sessions = stmt
        .query_map((local_user_id, limit, offset), |row| {
            Ok(Session {
                id: row.get(0)?,
                local_user_id: row.get(1)?,
                started_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(2)?)
                    .unwrap()
                    .with_timezone(&Utc),
                ended_at: row
                    .get::<_, Option<String>>(3)?
                    .map(|s| DateTime::parse_from_rfc3339(&s).unwrap().with_timezone(&Utc)),
                world_id: row.get(4)?,
                world_name: row.get(5)?,
                instance_id: row.get(6)?,
            })
        })?
        .collect::<Result<Vec<_>>>()?;

    Ok(sessions)
}
