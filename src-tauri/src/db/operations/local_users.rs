use rusqlite::{Connection, Result, OptionalExtension};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct LocalUser {
    pub id: i64,
    pub display_name: String,
    pub user_id: String,
    pub first_authenticated_at: DateTime<Utc>,
    pub last_authenticated_at: DateTime<Utc>,
}

/// ローカルユーザーを作成または更新
pub fn upsert_local_user(
    conn: &Connection,
    display_name: &str,
    user_id: &str,
    authenticated_at: DateTime<Utc>,
) -> Result<i64> {
    // 既存のユーザーを確認
    let existing: Option<i64> = conn
        .query_row(
            "SELECT id FROM local_users WHERE user_id = ?1",
            [user_id],
            |row| row.get(0),
        )
        .optional()?;

    let local_user_id = if let Some(id) = existing {
        // 既存ユーザーの最終認証時刻と表示名を更新
        conn.execute(
            "UPDATE local_users SET last_authenticated_at = ?1, display_name = ?2 WHERE id = ?3",
            (authenticated_at.to_rfc3339(), display_name, id),
        )?;
        id
    } else {
        // 新規ユーザーを作成
        conn.execute(
            "INSERT INTO local_users (display_name, user_id, first_authenticated_at, last_authenticated_at)
             VALUES (?1, ?2, ?3, ?4)",
            (
                display_name,
                user_id,
                authenticated_at.to_rfc3339(),
                authenticated_at.to_rfc3339(),
            ),
        )?;
        conn.last_insert_rowid()
    };

    // 名前履歴を作成または更新
    super::upsert_local_user_name_history(conn, local_user_id, display_name, authenticated_at)?;

    Ok(local_user_id)
}

/// すべてのローカルユーザーを取得
pub fn get_all_local_users(conn: &Connection) -> Result<Vec<LocalUser>> {
    let mut stmt = conn.prepare(
        "SELECT id, display_name, user_id, first_authenticated_at, last_authenticated_at
         FROM local_users
         ORDER BY last_authenticated_at DESC",
    )?;

    let users = stmt
        .query_map([], |row| {
            Ok(LocalUser {
                id: row.get(0)?,
                display_name: row.get(1)?,
                user_id: row.get(2)?,
                first_authenticated_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(3)?)
                    .unwrap()
                    .with_timezone(&Utc),
                last_authenticated_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(4)?)
                    .unwrap()
                    .with_timezone(&Utc),
            })
        })?
        .collect::<Result<Vec<_>>>()?;

    Ok(users)
}

/// user_idでローカルユーザーを取得
pub fn get_local_user_by_user_id(conn: &Connection, user_id: &str) -> Result<Option<LocalUser>> {
    conn.query_row(
        "SELECT id, display_name, user_id, first_authenticated_at, last_authenticated_at
         FROM local_users
         WHERE user_id = ?1",
        [user_id],
        |row| {
            Ok(LocalUser {
                id: row.get(0)?,
                display_name: row.get(1)?,
                user_id: row.get(2)?,
                first_authenticated_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(3)?)
                    .unwrap()
                    .with_timezone(&Utc),
                last_authenticated_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(4)?)
                    .unwrap()
                    .with_timezone(&Utc),
            })
        },
    )
    .optional()
}
