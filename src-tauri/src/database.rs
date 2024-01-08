use std::str::FromStr;

use futures::TryStreamExt;
use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    Row, SqlitePool,
};

use crate::{
    Action,
    Element,
    Skill,
    Trait,
    TraitValue,
    Item,
    Text,
    CaseGroup,
    Case,
};

/// このモジュール内の関数の戻り値型
type DbResult<T> = Result<T, Box<dyn std::error::Error>>;

/// SQLiteのコネクションプールを作成して返す
pub(crate) async fn create_sqlite_pool(database_url: &str) -> DbResult<SqlitePool> {
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
    // ファイルパス
    let database_url = format!("{}\\data\\o_skill.db", dir_path);

    let pool = create_sqlite_pool(&database_url).await?;

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
    // ファイルパス
    let database_url = format!("{}\\data\\o_skill.db", dir_path);

    let pool = create_sqlite_pool(&database_url).await?;

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
    // ファイルパス
    let database_url = format!("{}\\data\\o_skill.db", dir_path);

    let pool = create_sqlite_pool(&database_url).await?;

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

pub(crate) async fn get_traits(pool: &SqlitePool) -> DbResult<Vec<Trait>> {
    const SQL: &str = "SELECT * FROM oo_trait ORDER BY id ASC";
    let mut rows = sqlx::query(SQL).fetch(pool);

    let mut traits = Vec::new();
    while let Some(row) = rows.try_next().await? {
        let id: i64 = row.try_get("id")?;
        let group: i64 = row.try_get("group")?;
        let text: &str = row.try_get("text")?;
        let text_e: &str = row.try_get("text_e")?;
        traits.push(Trait::new(id, group, text, text_e));
    }

    Ok(traits)
}

pub(crate) async fn get_trait_values(pool: &SqlitePool) -> DbResult<Vec<TraitValue>> {
    const SQL: &str = "SELECT * FROM oo_trait_value ORDER BY id,value ASC";
    let mut rows = sqlx::query(SQL).fetch(pool);

    let mut trait_values = Vec::new();
    while let Some(row) = rows.try_next().await? {
        let id: i64 = row.try_get("id")?;
        let value: i64 = row.try_get("value")?;
        let text: &str = row.try_get("text")?;
        let text_e: &str = row.try_get("text_e")?;
        trait_values.push(TraitValue::new(id, value, text, text_e));
    }

    Ok(trait_values)
}

pub(crate) async fn get_items(pool: &SqlitePool) -> DbResult<Vec<Item>> {
    const SQL: &str = "SELECT * FROM oo_item ORDER BY id ASC";
    let mut rows = sqlx::query(SQL).fetch(pool);

    let mut items = Vec::new();
    while let Some(row) = rows.try_next().await? {
        let id: &str = row.try_get("dbid")?;
        let reftype: i64 = row.try_get("reftype")?;
        let reftypeminor: i64 = row.try_get("reftypeminor")?;
        let name: &str = row.try_get("name")?;
        let name_e: &str = row.try_get("name_e")?;
        items.push(Item::new(id, reftype, reftypeminor, name, name_e));
    }

    Ok(items)
}

pub(crate) async fn get_texts(pool: &SqlitePool) -> DbResult<Vec<Text>> {
    const SQL: &str = "SELECT * FROM oo_text ORDER BY id ASC";
    let mut rows = sqlx::query(SQL).fetch(pool);

    let mut texts = Vec::new();
    while let Some(row) = rows.try_next().await? {
        let tag: &str = row.try_get("tag")?;
        let label: &str = row.try_get("label")?;
        texts.push(Text::new(tag, label));
    }

    Ok(texts)
}

pub(crate) async fn get_case_groups(pool: &SqlitePool) -> DbResult<Vec<CaseGroup>> {
    const SQL: &str = "SELECT * FROM oo_case_group ORDER BY id ASC";
    let mut rows = sqlx::query(SQL).fetch(pool);

    let mut case_groups = Vec::new();
    while let Some(row) = rows.try_next().await? {
        let expression: &str = row.try_get("expression")?;
        let label: &str = row.try_get("label")?;
        case_groups.push(CaseGroup::new(expression, label));
    }

    Ok(case_groups)
}

pub(crate) async fn get_cases(pool: &SqlitePool) -> DbResult<Vec<Case>> {
    const SQL: &str = "SELECT * FROM oo_case ORDER BY id ASC";
    let mut rows = sqlx::query(SQL).fetch(pool);

    let mut cases = Vec::new();
    while let Some(row) = rows.try_next().await? {
        let expression: &str = row.try_get("expression")?;
        let value: &str = row.try_get("value")?;
        let args_size: i64 = row.try_get("args_size")?;
        let args_type: &str = row.try_get("args_type")?;
        let label: &str = row.try_get("label")?;
        cases.push(Case::new(expression, value, args_size, args_type, label));
    }

    Ok(cases)
}
