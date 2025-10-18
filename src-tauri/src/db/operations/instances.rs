use chrono::{DateTime, Utc};
use rusqlite::{Connection, OptionalExtension, Result};

#[derive(Debug, Clone)]
pub struct Instance {
    pub id: i64,
    pub player_id: i64, // ローカルプレイヤー (is_local=1のplayer)
    pub started_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
    pub world_id: String,
    pub world_name: Option<String>,
    pub instance_id: String,
    pub status: String, // active, completed, interrupted
}

/// 新しいインスタンスを作成
pub fn create_instance(
    conn: &Connection,
    player_id: i64,
    started_at: DateTime<Utc>,
    world_id: &str,
    world_name: Option<&str>,
    instance_id: &str,
) -> Result<i64> {
    conn.execute(
        "INSERT INTO instances (player_id, started_at, world_id, world_name, instance_id)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        (
            player_id,
            started_at.to_rfc3339(),
            world_id,
            world_name,
            instance_id,
        ),
    )?;
    Ok(conn.last_insert_rowid())
}

/// インスタンスの終了時刻を更新し、状態をcompletedに設定
pub fn end_instance(conn: &Connection, instance_id: i64, ended_at: DateTime<Utc>) -> Result<()> {
    conn.execute(
        "UPDATE instances SET ended_at = ?1, status = 'completed' WHERE id = ?2",
        (ended_at.to_rfc3339(), instance_id),
    )?;
    Ok(())
}

/// 最新の未終了インスタンスを取得
pub fn get_latest_active_instance(conn: &Connection, player_id: i64) -> Result<Option<Instance>> {
    conn.query_row(
        "SELECT id, player_id, started_at, ended_at, world_id, world_name, instance_id, status
         FROM instances
         WHERE player_id = ?1 AND ended_at IS NULL
         ORDER BY started_at DESC
         LIMIT 1",
        [player_id],
        |row| {
            Ok(Instance {
                id: row.get(0)?,
                player_id: row.get(1)?,
                started_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(2)?)
                    .unwrap()
                    .with_timezone(&Utc),
                ended_at: row.get::<_, Option<String>>(3)?.map(|s| {
                    DateTime::parse_from_rfc3339(&s)
                        .unwrap()
                        .with_timezone(&Utc)
                }),
                world_id: row.get(4)?,
                world_name: row.get(5)?,
                instance_id: row.get(6)?,
                status: row.get(7)?,
            })
        },
    )
    .optional()
}

/// 指定したローカルプレイヤーのインスタンス一覧を取得
pub fn get_instances_by_player(
    conn: &Connection,
    player_id: i64,
    limit: usize,
    offset: usize,
) -> Result<Vec<Instance>> {
    let mut stmt = conn.prepare(
        "SELECT id, player_id, started_at, ended_at, world_id, world_name, instance_id, status
         FROM instances
         WHERE player_id = ?1
         ORDER BY started_at DESC
         LIMIT ?2 OFFSET ?3",
    )?;

    let instances = stmt
        .query_map((player_id, limit, offset), |row| {
            Ok(Instance {
                id: row.get(0)?,
                player_id: row.get(1)?,
                started_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(2)?)
                    .unwrap()
                    .with_timezone(&Utc),
                ended_at: row.get::<_, Option<String>>(3)?.map(|s| {
                    DateTime::parse_from_rfc3339(&s)
                        .unwrap()
                        .with_timezone(&Utc)
                }),
                world_id: row.get(4)?,
                world_name: row.get(5)?,
                instance_id: row.get(6)?,
                status: row.get(7)?,
            })
        })?
        .collect::<Result<Vec<_>>>()?;

    Ok(instances)
}
