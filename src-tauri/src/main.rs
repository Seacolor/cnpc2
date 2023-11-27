// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{fs::{self, File}, io::Write};
use std::collections::HashMap;

use encoding_rs::SHIFT_JIS;
use placeholder::render;
use ini::Ini;

use serde::{Deserialize, Serialize};
use tauri::{Manager, State};

pub(crate) mod path;
pub(crate) mod csv;
pub(crate) mod database;
pub(crate) mod parser;

#[derive(Debug, Serialize, Deserialize)]
pub struct RaceCollection {
    list: Vec<Race>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClassCollection {
    list: Vec<Class>,
}

pub trait MasterData {
    fn new(name: &str, id: &str, playable: bool) -> Self;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Race {
    name: String,
    id: String,
    playable: bool,
}

impl MasterData for Race {
    fn new(name: &str, id: &str, playable: bool) -> Self {
        Race {
            name: name.to_string(),
            id: id.to_string(),
            playable: playable,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Class {
    name: String,
    id: String,
    playable: bool,
}

impl MasterData for Class {
    fn new(name: &str, id: &str, playable: bool) -> Self {
        Class {
            name: name.to_string(),
            id: id.to_string(),
            playable: playable,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActionCollection {
    list: Vec<Action>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Action {
    id: i64,
    name: String,
    name_e: String,
}

impl Action {
    pub fn new(id: i64, name: &str, name_e: &str) -> Self {
        Action {
            id,
            name: name.to_string(),
            name_e: name_e.to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ElementCollection {
    list: Vec<Element>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Element {
    id: i64,
    name: String,
    name_e: String,
}

impl Element {
    pub fn new(id: i64, name: &str, name_e: &str) -> Self {
        Element {
            id,
            name: name.to_string(),
            name_e: name_e.to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResistValueCollection {
    list: Vec<ResistValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResistValue {
    value: i64,
    label: String,
    label_e: String,
}

impl ResistValue {
    pub fn new(value: i64, label: &str, label_e: &str) -> Self {
        ResistValue {
            value,
            label: label.to_string(),
            label_e: label_e.to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BitCollection {
    list: Vec<Bit>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bit {
    value: i64,
    label: String,
    label_e: String,
}

impl Bit {
    pub fn new(value: i64, label: &str, label_e: &str) -> Self {
        Bit {
            value,
            label: label.to_string(),
            label_e: label_e.to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SkillCollection {
    list: Vec<Skill>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Skill {
    id: i64,
    name: String,
    name_e: String,
}

impl Skill {
    pub fn new(id: i64, name: &str, name_e: &str) -> Self {
        Skill {
            id,
            name: name.to_string(),
            name_e: name_e.to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TraitCollection {
    list: Vec<Trait>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Trait {
    id: i64,
    group: i64,
    text: String,
    text_e: String,
}

impl Trait {
    pub fn new(id: i64, group: i64, text: &str, text_e: &str) -> Self {
        Trait {
            id,
            group,
            text: text.to_string(),
            text_e: text_e.to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TraitValueCollection {
    list: Vec<TraitValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TraitValue {
    id: i64,
    value: i64,
    text: String,
    text_e: String,
}

impl TraitValue {
    pub fn new(id: i64, value: i64, text: &str, text_e: &str) -> Self {
        TraitValue {
            id,
            value,
            text: text.to_string(),
            text_e: text_e.to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FigureCollection {
    list: Vec<Figure>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Figure {
    value: String,
}

impl Figure {
    pub fn new(value: &str) -> Self {
        Figure {
            value: value.to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemCollection {
    list: Vec<Item>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    id: String,
    reftype: i64,
    reftypeminor: i64,
    name: String,
    name_e: String,
}

impl Item {
    pub fn new(id: &str, reftype: i64, reftypeminor: i64, name: &str, name_e: &str) -> Self {
        Item {
            id: id.to_string(),
            reftype: reftype,
            reftypeminor: reftypeminor,
            name: name.to_string(),
            name_e: name_e.to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextCollection {
    list: Vec<Text>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Text {
    id: String,
    label: String,
}

impl Text {
    pub fn new(id: &str, label: &str) -> Self {
        Text {
            id: id.to_string(),
            label: label.to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CaseGroupCollection {
    list: Vec<CaseGroup>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CaseGroup {
    id: String,
    label: String,
}

impl CaseGroup {
    pub fn new(id: &str, label: &str) -> Self {
        CaseGroup {
            id: id.to_string(),
            label: label.to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CaseCollection {
    list: Vec<Case>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Case {
    id: String,
    value: String,
    values_size: i64,
    values_type: String,
    label: String,
}

impl Case {
    pub fn new(id: &str, value: &str, values_size: i64, values_type: &str, label: &str) -> Self {
        Case {
            id: id.to_string(),
            value: value.to_string(),
            values_size: values_size,
            values_type: values_type.to_string(),
            label: label.to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResist {
    id: i64,
    value: i64,
}

impl UserResist {
    pub fn new(id: i64, value: i64) -> Self {
        UserResist {
            id,
            value,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserSkill {
    id: i64,
    value: i64,
}

impl UserSkill {
    pub fn new(id: i64, value: i64) -> Self {
        UserSkill {
            id,
            value,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserTrait {
    id: i64,
    value: i64,
}

impl UserTrait {
    pub fn new(id: i64, value: i64) -> Self {
        UserTrait {
            id,
            value,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserTalk {
    jp: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserTextBody {
    case_value: String,
    values: Vec<String>,
    jp: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserText {
    id: String,
    value: String,
    bodies: Vec<UserTextBody>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InitEquip {
    head: String,
    head_custom_item_id: String,
    weapon1: String,
    weapon1_custom_item_id: String,
    shield: String,
    shield_custom_item_id: String,
    shoot: String,
    shoot_custom_item_id: String,
    ammo: String,
    ammo_custom_item_id: String,
    weapon2: String,
    weapon2_custom_item_id: String,
    body: String,
    body_custom_item_id: String,
    arm: String,
    arm_custom_item_id: String,
    leg: String,
    leg_custom_item_id: String,
    back: String,
    back_custom_item_id: String,
    waist: String,
    waist_custom_item_id: String,
    ring1: String,
    ring1_custom_item_id: String,
    ring2: String,
    ring2_custom_item_id: String,
    neck1: String,
    neck1_custom_item_id: String,
    neck2: String,
    neck2_custom_item_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserClass {
    name: String,
    id: String,
    playable: bool,
    str: i64,
    end: i64,
    dex: i64,
    per: i64,
    ler: i64,
    wil: i64,
    mag: i64,
    chr: i64,
    spd: i64,
    equip: i64,
    skill: Vec<UserSkill>,
    description: String,
    desc_e: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRace {
    name: String,
    id: String,
    id2: i64,
    playable: bool,
    sex: i64,
    pic: i64,
    pic2: i64,
    dv: i64,
    pv: i64,
    hp: i64,
    mp: i64,
    str: i64,
    end: i64,
    dex: i64,
    per: i64,
    ler: i64,
    wil: i64,
    mag: i64,
    chr: i64,
    spd: i64,
    melee_style: i64,
    cast_style: i64,
    resist: i64,
    age_rnd: i64,
    age: i64,
    blood: i64,
    breeder: i64,
    height: i64,
    skill: Vec<UserSkill>,
    race_trait: Vec<UserTrait>,
    figure: Vec<Figure>,
    description: String,
    desc_e: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Character {
    author: String,
    id: String,
    name: String,
    race: String,
    class: String,
    filter: Vec<String>,
    level: i64,
    relation: i64,
    sex: i64,
    fix_lv: i64,
    rare: i64,
    spawn_type: i64,
    ai_calm: i64,
    ai_move: i64,
    ai_dist: i64,
    ai_heal: i64,
    ai_act0: i64,
    ai_act1: i64,
    ai_act2: i64,
    ai_act3: i64,
    ai_act4: i64,
    ai_act_sub_freq: i64,
    ai_act_sub0: i64,
    ai_act_sub1: i64,
    ai_act_sub2: i64,
    ai_act_sub3: i64,
    ai_act_sub4: i64,
    melee_elem_id: i64,
    melee_elem_power: i64,
    resist: Vec<UserResist>,
    bit_on: Vec<i64>,
    transmissivity: i64,
    drop_shadow_type: i64,
    c_set_pos: i64,
    no_food_or_drink: bool,
    cnpc_role: i64,
    race_alias: String,
    class_alias: String,
    random_name: bool,
    chipref: i64,
    colref: i64,
    user_race_enabled: bool,
    user_race: UserRace,
    user_class_enabled: bool,
    user_class: UserClass,
    init_equip_enabled: bool,
    init_equip: InitEquip,
    txt_talk_order: bool,
    txt: Vec<UserText>,
    talk_enabled: bool,
    talk: UserTalk,
}

#[tauri::command]
async fn set_elona_dir(path: &str) -> Result<bool, String> {
    let dir_exists = std::fs::metadata(path).is_ok();
    if !dir_exists {
        return Err("Directory not found".into())
    }

     // ユーザのデータディレクトリ直下にコンフィグのディレクトリを作成する
    let config_dir = directories::ProjectDirs::from(
            "",
            "",
            "Custom NPC Editor 2"
        )
        .map(|dirs| dirs.config_dir().to_path_buf())
        // ホームディレクトリが取得できないときはカレントディレクトリを使う
        .unwrap_or_else(|| std::env::current_dir().expect("Cannot access the current directory"));
    let conf_file_path = config_dir.join("conf.ini");
    // コンフィグファイルが存在するかチェックする
    let config_exists = std::fs::metadata(&conf_file_path).is_ok();
    // 存在しないなら、ファイルを格納するためのディレクトリを作成する
    if !config_exists {
        let _ = std::fs::create_dir_all(&config_dir);
    }

    let mut conf = Ini::new();
    conf.with_section(Some("Elona"))
        .set("path", path);
    let mut f = File::create(conf_file_path).unwrap();
    conf.write_to(&mut f).unwrap();

    let ready = path::is_file_exists();
    Ok(ready)
}

#[tauri::command]
async fn is_ready() -> Result<bool, String> {
    let ready = path::is_file_exists();
    Ok(ready)
}

#[tauri::command]
async fn get_races() -> Result<RaceCollection, String> {
    if let Some(s) = path::read_path() {
        if let Ok(races) = csv::read_race(&s) {
            return Ok(RaceCollection { list: races })
        }
    }
    return Err("csv file not found".into());
}

#[tauri::command]
async fn get_classes() -> Result<ClassCollection, String> {
    if let Some(s) = path::read_path() {
        if let Ok(classes) = csv::read_class(&s) {
            return Ok(ClassCollection { list: classes })
        }
    }
    return Err("csv file not found".into());
}

#[tauri::command]
async fn get_actions() -> Result<ActionCollection, String> {
    if let Some(s) = path::read_path() {
        let mut actions: Vec<Action> = Vec::new();
        actions.extend([
            Action::new(0, "何もしない", "Nothing"),
            Action::new(-1, "近接攻撃", "Melee Attack"),
            Action::new(-2, "遠隔攻撃", "Ranged Attack"),
            Action::new(-3, "接近待ち", "Melee Attack or Wait"),
            Action::new(-4, "ランダムな方向に移動", "Random Move"),
        ]);

        let spells_and_skills = database::get_actions(&s)
            .await
            .map_err(|e| e.to_string())?;
        actions.extend(spells_and_skills);

        actions.extend([
            Action::new(-9995, "トマト投擲", "Throw Tomato"),
            Action::new(-9996, "塩投擲", "Throw Salt"),
            Action::new(-9997, "ポーション投擲・大", "Throw Potion(many)"),
            Action::new(-9998, "ポーション投擲・中", "Throw Potion(middle)"),
            Action::new(-9999, "ポーション投擲・小", "Throw Potion(few)"),
        ]);

        return Ok(ActionCollection { list: actions })
    }

    Err("database file not found.".into())
}

#[tauri::command]
async fn get_elements() -> Result<ElementCollection, String> {
    if let Some(s) = path::read_path() {
        let mut elements: Vec<Element> = Vec::new();
        elements.extend([
            Element::new(64, "無", "void"),
        ]);

        let registerd_elements = database::get_elements(&s)
            .await
            .map_err(|e| e.to_string())?;
        elements.extend(registerd_elements);

        return Ok(ElementCollection { list: elements })
    }

    Err("database file not found.".into())
}

#[tauri::command]
async fn get_resist_values() -> Result<ResistValueCollection, String> {
    let regist_values: Vec<ResistValue> = vec![
        ResistValue::new(-10, "致命的な弱点", "Criticaly Weak"),
        ResistValue::new(-9, "致命的な弱点", "Criticaly Weak"),
        ResistValue::new(-8, "致命的な弱点", "Criticaly Weak"),
        ResistValue::new(-7, "致命的な弱点", "Criticaly Weak"),
        ResistValue::new(-6, "致命的な弱点", "Criticaly Weak"),
        ResistValue::new(-5, "致命的な弱点", "Criticaly Weak"),
        ResistValue::new(-4, "致命的な弱点", "Criticaly Weak"),
        ResistValue::new(-3, "致命的な弱点", "Criticaly Weak"),
        ResistValue::new(-2, "致命的な弱点", "Criticaly Weak"),
        ResistValue::new(-1, "弱点", "Weak"),
        ResistValue::new(1, "弱い耐性", "Little"),
        ResistValue::new(2, "普通の耐性", "Normal"),
        ResistValue::new(3, "強い耐性", "Strong"),
        ResistValue::new(4, "素晴らしい耐性", "Superb"),
        ResistValue::new(5, "素晴らしい耐性", "Superb"),
        ResistValue::new(6, "素晴らしい耐性", "Superb"),
        ResistValue::new(7, "素晴らしい耐性", "Superb"),
        ResistValue::new(8, "素晴らしい耐性", "Superb"),
        ResistValue::new(9, "素晴らしい耐性", "Superb"),
        ResistValue::new(10, "素晴らしい耐性", "Superb"),
    ];
    Ok(ResistValueCollection { list: regist_values })
}

#[tauri::command]
async fn get_bits() -> Result<BitCollection, String> {
    let bits: Vec<Bit> = vec![
        Bit::new(5, "浮遊", "floats you."),
        Bit::new(6, "透明", "invisible you."),
        Bit::new(7, "透明な物を見る", "allows you to see invisible creatures."),
        Bit::new(8, "混乱を無効", "negates the effect of confusion."),
        Bit::new(9, "盲目を無効", "negates the effect of blindness."),
        Bit::new(10, "恐怖とそれによる逃亡を無効", "negates the effect of fear."),
        Bit::new(11, "睡眠を無効", "negates the effect of sleep."),
        Bit::new(12, "麻痺を無効", "negates the effect of paralysis."),
        Bit::new(13, "毒を無効", "negates the effect of poison."),
        Bit::new(14, "腐ったものを難なく消化する", "allows you to digest rotten food."),
        Bit::new(15, "アイテムを盗まれなくする", "protects you from thieves."),
        Bit::new(16, "変装中", "disguising yourself."),
        Bit::new(17, "死亡時に必ず所持金を落とす", "always drop your possessions when you die."),
        Bit::new(18, "自爆生物", "explosive creatures."),
        Bit::new(19, "死の宣告持ち", "death the death word breaks."),
        Bit::new(20, "連続魔法持ち", "cast several spells in a raw."),
        Bit::new(21, "レイハンド持ち", "use Lay on hand to heal a deadly wounded ally."),
        Bit::new(22, "適正騎乗生物", "feel comfortable."),
        Bit::new(23, "分裂生物", "split-able creature."),
        Bit::new(24, "自動発動型エンチャント装備中", "equipped with enchants that are automatically triggered."),
        Bit::new(25, "非適正騎乗生物(非力すぎる)", "too weak to carry you."),
        Bit::new(26, "魔法以外の属性ダメージに強い", "resistant to non-magical damage."),
        Bit::new(27, "分裂生物(キューブ)", "Cube-type split-able creature."),
        Bit::new(28, "金属生物", "resistance as metal."),
        Bit::new(29, "出血を抑える", "diminishs bleeding."),
        Bit::new(30, "バッシュの発生率アップ", "increases your chance of shield bash."),
        Bit::new(31, "地雷無効", "mines don't work."),
        Bit::new(32, "激怒する", "engulf in fury."),
    ];
    Ok(BitCollection { list: bits })
}

#[tauri::command]
async fn get_skills() -> Result<SkillCollection, String> {
    if let Some(s) = path::read_path() {
        let skills = database::get_skills(&s)
        .await
        .map_err(|e| e.to_string())?;
        return Ok(SkillCollection { list: skills })
    }
    Err("database file not found.".into())
}

#[tauri::command]
async fn get_traits(sqlite_pool: State<'_, sqlx::SqlitePool>) -> Result<TraitCollection, String> {
    let list = database::get_traits(&*sqlite_pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(TraitCollection { list })
}

#[tauri::command]
async fn get_trait_values(sqlite_pool: State<'_, sqlx::SqlitePool>) -> Result<TraitValueCollection, String> {
    let list = database::get_trait_values(&*sqlite_pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(TraitValueCollection { list })
}

#[tauri::command]
async fn get_figures() -> Result<FigureCollection, String> {
    let figures: Vec<Figure> = vec![
        Figure::new("頭"),
        Figure::new("首"),
        Figure::new("体"),
        Figure::new("背"),
        Figure::new("手"),
        Figure::new("指"),
        Figure::new("腕"),
        Figure::new("腰"),
        Figure::new("足"),
    ];
    Ok(FigureCollection { list: figures })
}

#[tauri::command]
async fn get_items(sqlite_pool: State<'_, sqlx::SqlitePool>) -> Result<ItemCollection, String> {
    let list = database::get_items(&*sqlite_pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(ItemCollection { list })
}

#[tauri::command]
async fn get_texts(sqlite_pool: State<'_, sqlx::SqlitePool>) -> Result<TextCollection, String> {
    let list = database::get_texts(&*sqlite_pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(TextCollection { list })
}

#[tauri::command]
async fn get_case_groups(sqlite_pool: State<'_, sqlx::SqlitePool>) -> Result<CaseGroupCollection, String> {
    let list = database::get_case_groups(&*sqlite_pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(CaseGroupCollection { list })
}

#[tauri::command]
async fn get_cases(sqlite_pool: State<'_, sqlx::SqlitePool>) -> Result<CaseCollection, String> {
    let list = database::get_cases(&*sqlite_pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(CaseCollection { list })
}

#[tauri::command]
async fn load_file(path: &str) -> Result<Character, String> {
    let file_exists = std::fs::metadata(path).is_ok();
    if !file_exists {
        return Err("File not found".into())
    }
    // 読み込み
    let s = fs::read(path).unwrap();
    // Shift_JISのバイト列(Vec<u8>) を UTF-8の文字列(String) に変換
    let (res, _, _) = SHIFT_JIS.decode(&s);
    let text = res.into_owned();
    let result = parser::parse(&text);
    if let Ok(_d) = result {
        return Ok(_d)
    }
    Err("File load error".into())
}

#[tauri::command]
async fn save_file(path: &str, data: Character) -> Result<(), String> {
    let mut text: Vec<String> = Vec::new();
    text.push("%Elona Custom Npc".into());
    text.push("".into());
    text.push(format!("author.\t\t\"{}\"", data.author));
    text.push(format!("name.\t\t\"{},{}\"", data.id, data.name));
    if data.user_race_enabled {
        text.push(format!("race.\t\t\"{}\"", data.user_race.id.to_string()));
    } else {
        text.push(format!("race.\t\t\"{}\"", data.race));
    }
    if data.user_class_enabled {
        text.push(format!("class.\t\t\"{}\"", data.user_class.id));
    } else {
        text.push(format!("class.\t\t\"{}\"", data.class));
    }
    let mut filter: Vec<String> = Vec::new();
    if data.filter.len() != 0 {
        filter.push("".into());
        filter.extend(data.filter);
        filter.push("".into());
    }
    text.push(format!("filter.\t\t\"{}\"", filter.join("/")));
    text.push(format!("level.\t\t\"{}\"", data.level));
    text.push(format!("relation.\t\"{}\"", data.relation));
    text.push(format!("sex.\t\t\"{}\"", data.sex));
    text.push(format!("fixLv.\t\t\"{}\"", data.fix_lv));
    text.push(format!("rare.\t\t\"{}\"", data.rare));
    text.push(format!("spawnType.\t\"{}\"", data.spawn_type));
    text.push(format!("aiCalm.\t\t\"{}\"", data.ai_calm));
    text.push(format!("aiMove.\t\t\"{}\"", data.ai_move));
    text.push(format!("aiDist.\t\t\"{}\"", data.ai_dist));
    text.push(format!("aiHeal.\t\t\"{}\"", data.ai_heal));
    text.push(format!("aiAct.\t\t\"{},{},{},{},{}\"", data.ai_act0, data.ai_act1, data.ai_act2, data.ai_act3, data.ai_act4));
    text.push(format!("aiActSubFreq.\t\"{}\"", data.ai_act_sub_freq));
    text.push(format!("aiActSub.\t\"{},{},{},{},{}\"", data.ai_act_sub0, data.ai_act_sub1, data.ai_act_sub2, data.ai_act_sub3, data.ai_act_sub4));
    text.push(format!("meleeElem.\t\"{},{}\"", data.melee_elem_id, data.melee_elem_power));
    let mut resist: Vec<String> = Vec::new();
    for _r in data.resist {
        resist.push(_r.id.to_string());
        resist.push(_r.value.to_string());
    }
    text.push(format!("resist.\t\t\"{}\"", resist.join(",")));
    text.push(format!("bitOn.\t\t\"{}\"", data.bit_on.iter().map(|s| s.to_string()).collect::<Vec<_>>().join(",")));
    text.push(format!("transmissivity.\t\"{}\"", data.transmissivity));
    text.push(format!("dropShadowType.\t\"{}\"", data.drop_shadow_type));
    text.push(format!("cSetPos.\t\"{}\"", data.c_set_pos));
    text.push(format!("noFoodOrDrink.\t\"{}\"",  if data.no_food_or_drink { "1" } else { "0" }));
    text.push(format!("cnpcRole.\t\"{}\"", data.cnpc_role));
    text.push(format!("raceAlias.\t\"{}\"", data.race_alias));
    text.push(format!("classAlias.\t\"{}\"", data.class_alias));
    text.push(format!("randomName.\t\"{}\"",  if data.random_name { "1" } else { "0" }));
    text.push(format!("chipref.\t\"{}\"", data.chipref));
    text.push(format!("colref.\t\t\"{}\"", data.colref));
    if data.user_race_enabled {
        let mut user_race: Vec<String> = Vec::new();
        user_race.push(data.user_race.name.to_string());
        user_race.push(data.user_race.id.to_string());
        user_race.push(data.user_race.id2.to_string());
        user_race.push(if data.user_race.playable { "1".into() } else { "".into() });
        user_race.push(data.user_race.sex.to_string());
        user_race.push(data.user_race.pic.to_string());
        user_race.push(data.user_race.pic2.to_string());
        user_race.push(data.user_race.dv.to_string());
        user_race.push(data.user_race.pv.to_string());
        user_race.push(data.user_race.hp.to_string());
        user_race.push(data.user_race.mp.to_string());
        user_race.push(data.user_race.str.to_string());
        user_race.push(data.user_race.end.to_string());
        user_race.push(data.user_race.dex.to_string());
        user_race.push(data.user_race.per.to_string());
        user_race.push(data.user_race.ler.to_string());
        user_race.push(data.user_race.wil.to_string());
        user_race.push(data.user_race.mag.to_string());
        user_race.push(data.user_race.chr.to_string());
        user_race.push(data.user_race.spd.to_string());
        user_race.push(data.user_race.melee_style.to_string());
        user_race.push(data.user_race.cast_style.to_string());
        user_race.push(data.user_race.resist.to_string());
        user_race.push(data.user_race.age_rnd.to_string());
        user_race.push(data.user_race.age.to_string());
        user_race.push(data.user_race.blood.to_string());
        user_race.push(data.user_race.breeder.to_string());
        user_race.push(data.user_race.height.to_string());
        user_race.push(data.user_race.skill.iter().map(|s| format!("{}", s.id * 10000 + s.value)).collect::<Vec<_>>().join("|"));
        user_race.push(data.user_race.race_trait.iter().map(|s| format!("{}", s.id * 100 + s.value)).collect::<Vec<_>>().join("|"));
        user_race.push(data.user_race.figure.iter().map(|f| f.value.to_string()).collect::<Vec<_>>().join("|"));
        user_race.push(data.user_race.description.to_string());
        user_race.push(data.user_race.desc_e.to_string());
        text.push(format!("userRace.\t\"{}\"", user_race.join(",")));
    }
    if data.user_class_enabled {
        let mut user_class: Vec<String> = Vec::new();
        user_class.push(data.user_class.name.to_string());
        user_class.push(data.user_class.id.to_string());
        user_class.push(if data.user_class.playable { "1".into() } else { "".into() });
        user_class.push(data.user_class.str.to_string());
        user_class.push(data.user_class.end.to_string());
        user_class.push(data.user_class.dex.to_string());
        user_class.push(data.user_class.per.to_string());
        user_class.push(data.user_class.ler.to_string());
        user_class.push(data.user_class.wil.to_string());
        user_class.push(data.user_class.mag.to_string());
        user_class.push(data.user_class.chr.to_string());
        user_class.push(data.user_class.spd.to_string());
        user_class.push(data.user_class.equip.to_string());
        user_class.push(data.user_class.skill.iter().map(|s| format!("{}", s.id * 10000 + s.value)).collect::<Vec<_>>().join("|"));
        user_class.push(data.user_class.description.to_string());
        user_class.push(data.user_class.desc_e.to_string());
        text.push(format!("userRace.\t\"{}\"", user_class.join(",")));
    }
    if data.init_equip_enabled {
        let mut init_equip: Vec<String> = Vec::new();
        if data.init_equip.head == "743" {
            init_equip.push(data.init_equip.head_custom_item_id.to_string());
        } else {
            init_equip.push(data.init_equip.head.to_string());
        }
        if data.init_equip.weapon1 == "743" {
            init_equip.push(data.init_equip.weapon1_custom_item_id.to_string());
        } else {
            init_equip.push(data.init_equip.weapon1.to_string());
        }
        if data.init_equip.shield == "743" {
            init_equip.push(data.init_equip.shield_custom_item_id.to_string());
        } else {
            init_equip.push(data.init_equip.shield.to_string());
        }
        if data.init_equip.shoot == "743" {
            init_equip.push(data.init_equip.shoot_custom_item_id.to_string());
        } else {
            init_equip.push(data.init_equip.shoot.to_string());
        }
        if data.init_equip.ammo == "743" {
            init_equip.push(data.init_equip.ammo_custom_item_id.to_string());
        } else {
            init_equip.push(data.init_equip.ammo.to_string());
        }
        if data.init_equip.weapon2 == "743" {
            init_equip.push(data.init_equip.weapon2_custom_item_id.to_string());
        } else {
            init_equip.push(data.init_equip.weapon2.to_string());
        }
        if data.init_equip.body == "743" {
            init_equip.push(data.init_equip.body_custom_item_id.to_string());
        } else {
            init_equip.push(data.init_equip.body.to_string());
        }
        if data.init_equip.arm == "743" {
            init_equip.push(data.init_equip.arm_custom_item_id.to_string());
        } else {
            init_equip.push(data.init_equip.arm.to_string());
        }
        if data.init_equip.leg == "743" {
            init_equip.push(data.init_equip.leg_custom_item_id.to_string());
        } else {
            init_equip.push(data.init_equip.leg.to_string());
        }
        if data.init_equip.back == "743" {
            init_equip.push(data.init_equip.back_custom_item_id.to_string());
        } else {
            init_equip.push(data.init_equip.back.to_string());
        }
        if data.init_equip.waist == "743" {
            init_equip.push(data.init_equip.waist_custom_item_id.to_string());
        } else {
            init_equip.push(data.init_equip.waist.to_string());
        }
        if data.init_equip.ring1 == "743" {
            init_equip.push(data.init_equip.ring1_custom_item_id.to_string());
        } else {
            init_equip.push(data.init_equip.ring1.to_string());
        }
        if data.init_equip.ring2 == "743" {
            init_equip.push(data.init_equip.ring2_custom_item_id.to_string());
        } else {
            init_equip.push(data.init_equip.ring2.to_string());
        }
        if data.init_equip.neck1 == "743" {
            init_equip.push(data.init_equip.neck1_custom_item_id.to_string());
        } else {
            init_equip.push(data.init_equip.neck1.to_string());
        }
        if data.init_equip.neck2 == "743" {
            init_equip.push(data.init_equip.neck2_custom_item_id.to_string());
        } else {
            init_equip.push(data.init_equip.neck2.to_string());
        }
        text.push(format!("initEquip.\t\"{}\"", init_equip.join(",")));
    }
    text.push("".into());
    if data.talk_enabled {
        text.push(data.talk.jp);
        text.push("".into());
    }
    if data.txt_talk_order {
        text.push("%txt_talk_order".into());
    }
    let mut txt: Vec<String> = Vec::new();
    for _t in data.txt {
        if _t.value != "" {
            let mut values = HashMap::new();
            values.insert(String::from("0"), _t.value);
            if let Ok(_s) = render(&_t.id, &values) {
                txt.push(format!("{},JP", _s));
            }
        } else {
            txt.push(format!("{},JP", _t.id));
        }
        for _b in _t.bodies {
            if _b.case_value != "" {
                if _b.values.len() != 0 {
                    let mut values = HashMap::new();
                    let mut i = 0;
                    while i < _b.values.len() {
                        values.insert(i.to_string(), _b.values.get(i).unwrap().to_string());
                        i += 1;
                    }
                    if let Ok(_s) = render(&_b.case_value, &values) {
                        txt.push(_s);
                    }
                } else {
                    txt.push(_b.case_value);
                }
            }
            for _m in _b.jp {
                txt.push(_m);
            }
        }
    }
    txt.push("%endTxt".into());
    text.push(txt.join("\r\n"));
    let string = text.join("\r\n");
    let (encoded, _, _) =  SHIFT_JIS.encode(&string);
    let mut f = File::create(path).unwrap();
    let _ = f.write_all(&encoded.into_owned());
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tauri::Builder::default()
        .invoke_handler(
            tauri::generate_handler![
                set_elona_dir,
                is_ready,
                get_races,
                get_classes,
                get_actions,
                get_elements,
                get_resist_values,
                get_bits,
                get_skills,
                get_traits,
                get_trait_values,
                get_figures,
                get_items,
                get_texts,
                get_case_groups,
                get_cases,
                load_file,
                save_file,
            ]
        )
        .setup(|app| {
            // このmain関数はasync fnではないので、asyncな関数を呼ぶのにblock_on関数を使う
            use tauri::async_runtime::block_on;

            // データベースのファイルパスを取得する
            let resource_path = app.path_resolver()
                .resolve_resource("resources/oo_data.db")
                .expect("failed to resolve resource");
            let database_dir_str = dunce::canonicalize(&resource_path)
                .unwrap()
                .to_string_lossy()
                .replace('\\', "/");
            let database_url = format!("sqlite://{}", database_dir_str);

            // SQLiteのコネクションプールを作成する
            let sqlite_pool = block_on(database::create_sqlite_pool(&database_url))?;
            app.manage(sqlite_pool);
        
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
