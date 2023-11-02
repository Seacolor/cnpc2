use std::str::FromStr;

use futures::TryStreamExt;
use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    Row, SqlitePool,
};

use crate::{Action, Element, Skill};

/// このモジュール内の関数の戻り値型
type DbResult<T> = Result<T, Box<dyn std::error::Error>>;

/// SQLiteのコネクションプールを作成して返す
pub(crate) async fn create_sqlite_pool(dir_path: &str) -> DbResult<SqlitePool> {
    // ファイルパス
    let database_url = format!("{}\\data\\o_skill.db", dir_path);

    // コネクションの設定
    let connection_options = SqliteConnectOptions::from_str(&database_url)?
        // 読み込み専用で開く
        .read_only(true);

    // 上の設定を使ってコネクションプールを作成する
    let sqlite_pool = SqlitePoolOptions::new()
        .connect_with(connection_options)
        .await?;

    Ok(sqlite_pool)
}

pub(crate) async fn get_actions(dir_path: &str) -> DbResult<Vec<Action>> {
    let pool = create_sqlite_pool(dir_path).await?;

    const SQL: &str = "SELECT * FROM o_skill WHERE id >= 400 AND name != '' ORDER BY id ASC";
    let mut rows = sqlx::query(SQL).fetch(&pool);

    let mut actions = Vec::new();
    while let Some(row) = rows.try_next().await? {
        let id: i64 = row.try_get("id")?;
        let name: &str = row.try_get("name")?;
        let name_e: &str = row.try_get("name-e")?;
        actions.push(Action::new(id, name, name_e));
    }

    Ok(actions)
}

pub(crate) async fn get_elements(dir_path: &str) -> DbResult<Vec<Element>> {
    let pool = create_sqlite_pool(dir_path).await?;

    const SQL: &str = "SELECT * FROM o_skill WHERE id >= 50 AND id < 100 AND name != '' ORDER BY id ASC";
    let mut rows = sqlx::query(SQL).fetch(&pool);

    let mut elements = Vec::new();
    while let Some(row) = rows.try_next().await? {
        let id: i64 = row.try_get("id")?;
        let name: &str = row.try_get("name")?;
        let name_e: &str = row.try_get("name-e")?;
        elements.push(Element::new(id, name, name_e));
    }

    Ok(elements)
}

pub(crate) async fn get_skills(dir_path: &str) -> DbResult<Vec<Skill>> {
    let pool = create_sqlite_pool(dir_path).await?;

    const SQL: &str = "SELECT * FROM o_skill WHERE id >= 100 AND id < 400 AND name != '' ORDER BY id ASC";
    let mut rows = sqlx::query(SQL).fetch(&pool);

    let mut skills = Vec::new();
    while let Some(row) = rows.try_next().await? {
        let id: i64 = row.try_get("id")?;
        let name: &str = row.try_get("name")?;
        let name_e: &str = row.try_get("name-e")?;
        skills.push(Skill::new(id, name, name_e));
    }

    Ok(skills)
}
