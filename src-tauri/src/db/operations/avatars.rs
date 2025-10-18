use rusqlite::{Connection, Result, OptionalExtension};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Avatar {
    pub id: i64,
    pub avatar_id: Option<String>,
    pub avatar_name: String,
    pub first_seen_at: DateTime<Utc>,
    pub last_seen_at: DateTime<Utc>,
}

/// アバターを作成または更新（名前ベース）
pub fn upsert_avatar_by_name(
    conn: &Connection,
    avatar_name: &str,
    avatar_id: Option<&str>,
    seen_at: DateTime<Utc>,
) -> Result<i64> {
    // 既存のアバターを確認（名前で検索）
    let existing: Option<i64> = conn
        .query_row(
            "SELECT id FROM avatars WHERE avatar_name = ?1",
            [avatar_name],
            |row| row.get(0),
        )
        .optional()?;

    if let Some(id) = existing {
        // 既存アバターの最終確認時刻を更新
        // avatar_idが提供された場合は更新
        if let Some(aid) = avatar_id {
            conn.execute(
                "UPDATE avatars SET last_seen_at = ?1, avatar_id = ?2 WHERE id = ?3",
                (seen_at.to_rfc3339(), aid, id),
            )?;
        } else {
            conn.execute(
                "UPDATE avatars SET last_seen_at = ?1 WHERE id = ?2",
                (seen_at.to_rfc3339(), id),
            )?;
        }
        Ok(id)
    } else {
        // 新規アバターを作成
        conn.execute(
            "INSERT INTO avatars (avatar_id, avatar_name, first_seen_at, last_seen_at)
             VALUES (?1, ?2, ?3, ?4)",
            (
                avatar_id,
                avatar_name,
                seen_at.to_rfc3339(),
                seen_at.to_rfc3339(),
            ),
        )?;
        Ok(conn.last_insert_rowid())
    }
}

/// アバター使用履歴を記録
pub fn record_avatar_usage(
    conn: &Connection,
    instance_id: i64,
    player_id: i64,  // NOT NULL: ローカルプレイヤー or リモートプレイヤー
    avatar_id: i64,  // NOT NULL: avatarsテーブルへの参照
    changed_at: DateTime<Utc>,
) -> Result<()> {
    conn.execute(
        "INSERT INTO avatar_usages (instance_id, player_id, avatar_id, changed_at)
         VALUES (?1, ?2, ?3, ?4)",
        (
            instance_id,
            player_id,
            avatar_id,
            changed_at.to_rfc3339(),
        ),
    )?;
    Ok(())
}

/// インスタンス内のアバター使用履歴を取得
#[derive(Debug, Clone)]
pub struct AvatarUsage {
    pub player_id: i64,
    pub display_name: String,
    pub is_local: bool,  // ローカルプレイヤーかどうか
    pub avatar_name: String,
    pub changed_at: DateTime<Utc>,
}

pub fn get_avatar_usages_in_instance(
    conn: &Connection,
    instance_id: i64,
) -> Result<Vec<AvatarUsage>> {
    let mut stmt = conn.prepare(
        "SELECT au.player_id, p.display_name, p.is_local, a.avatar_name, au.changed_at
         FROM avatar_usages au
         INNER JOIN players p ON au.player_id = p.id
         INNER JOIN avatars a ON au.avatar_id = a.id
         WHERE au.instance_id = ?1
         ORDER BY au.changed_at",
    )?;

    let usages = stmt
        .query_map([instance_id], |row| {
            Ok(AvatarUsage {
                player_id: row.get(0)?,
                display_name: row.get(1)?,
                is_local: row.get(2)?,
                avatar_name: row.get(3)?,
                changed_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(4)?)
                    .unwrap()
                    .with_timezone(&Utc),
            })
        })?
        .collect::<Result<Vec<_>>>()?;

    Ok(usages)
}
