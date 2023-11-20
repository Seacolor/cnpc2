// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{fs::{self, File}, io::Write};
use std::collections::HashMap;

use encoding_rs;
use placeholder::render;
use ini::Ini;

use serde::{Deserialize, Serialize};

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
async fn get_traits() -> Result<TraitCollection, String> {
    let traits: Vec<Trait> = vec![
        Trait::new(0, 2, "あなたの肉体は刻々と変化する", "Your body is changing every moment."),
        Trait::new(1, 0, "流れ星を見て幸運を手にする", "Sighting a falling star brings you luck."),
        Trait::new(2, 0, "苦行を通じてHPを増やす", "Being an ascetic increases your HP."),
        Trait::new(3, 0, "吸血鬼の友人から血の吸い方を学ぶ", "Your vampire friend teaches you how to suck blood."),
        Trait::new(4, 0, "猫を追いかけて速くなる", "Chasing cats increases your speed."),
        Trait::new(5, 0, "腕相撲の特訓で筋力を高める", "Training arm wrestling increases your strength."),
        Trait::new(6, 0, "周りを注意深く観察し、罠の発見を容易にする", "You search around carefully to spot traps."),
        Trait::new(7, 0, "受身を会得しPVを上昇させる", "You establish your own defensive style."),
        Trait::new(8, 0, "敵の動きを見極め華麗に回避する", "You learn how to dodge attacks."),
        Trait::new(9, 0, "頭を壁にぶつけて耐久力を高める", "Pushing your head against a wall makes you tougher."),
        Trait::new(10, 0, "回避の技術に磨きをかける", "You improve your evasion skill."),
        Trait::new(11, 0, "遺伝子組み換えを行いMPを増やす", "You gain MP by using gene manipulation."),
        Trait::new(12, 0, "日々瞑想し詠唱技術を高める", "Daily meditation increases your casting skill."),
        Trait::new(13, 0, "夢の中で近くへの瞬間移動を思いつく", "You hit upon an idea of insta-teleport from a dream."),
        Trait::new(14, 0, "酒を飲んでいるうちに火を噴けるようになる", "Too much drinking makes you want to breath fire."),
        Trait::new(15, 0, "地底での生活が暗黒への耐性を高める", "Living in darkness makes you resistant to darkness."),
        Trait::new(16, 0, "行商人の元で働き交渉の技術を学ぶ", "Working under a trader improves your negotiation skill."),
        Trait::new(17, 0, "神官の下で修行し信仰を深める", "Training under a priest deepens your faith."),
        Trait::new(18, 0, "貴族の毒見係のおかげで毒への耐性を得る", "Being a taster for a noble grants you resistance to poison."),
        Trait::new(19, 0, "長年の修行により二刀流の技術を磨く", "Years of training increases your two weapon fighting skill."),
        Trait::new(20, 0, "ストリートファイトに興じ格闘技の技を得る", "You learn tricks of martial arts from fighting in streets."),
        Trait::new(21, 0, "セクシーなポーズを学ぶ", "You learn sexy poses."),
        Trait::new(22, 0, "突如として眠りの手を覚える", "Suddenly, you know how to make everyone sleep."),
        Trait::new(23, 0, "毒の研究の結果、毒の手を覚える", "Researching poisons leads you to a method to poison others."),
        Trait::new(24, 0, "長年の特訓の成果がスタミナを高める", "Years of training strengthens your stamina."),
        Trait::new(38, 0, "会計の技能を磨き税金を減らす", "Working as an accountant reduces your tax payment."),
        Trait::new(39, 0, "補給係としての経験が、給料物資の品質を上げる", "Working as a quartermaster gives you better payoff."),
        Trait::new(40, 0, "指導を続けるうちに、仲間を一時的に強くする方法を見つける", "You find how to strengthen your allies for a short time."),
        Trait::new(42, 0, "祈祷を捧げることで、呪いの言葉の効力を弱める", "Your prayer weakens the power of cursed whisperings."),
        Trait::new(43, 0, "盾の扱いを身につけ、盾での攻撃機会を増やす", "Mastering shield increases your chance of shield bash."),
        Trait::new(44, 0, "優しい笑顔を習得し、敵が逃げ出すのを防ぐ", "Your gentle face prevents your enemies from escaping."),
        Trait::new(46, 0, "実戦で長剣の扱いを学ぶ", ""),
        Trait::new(47, 0, "実戦で短剣の扱いを学ぶ", ""),
        Trait::new(48, 0, "実戦で斧の扱いを学ぶ", ""),
        Trait::new(49, 0, "実戦で鈍器の扱いを学ぶ", ""),
        Trait::new(50, 0, "実戦で槍の扱いを学ぶ", ""),
        Trait::new(51, 0, "実戦で杖の扱いを学ぶ", ""),
        Trait::new(53, 0, "実戦で鎌の扱いを学ぶ", ""),
        Trait::new(54, 0, "実戦で弓の扱いを学ぶ", ""),
        Trait::new(55, 0, "実戦でクロスボウの扱いを学ぶ", ""),
        Trait::new(56, 0, "実戦で銃器の扱いを学ぶ", ""),
        Trait::new(57, 0, "実戦で投擲の扱いを学ぶ", ""),
        Trait::new(58, 0, "戦場の経験が死中に活を求める", ""),
        Trait::new(59, 0, "理性を捨て、防御力と引き換えに攻撃力を高める", ""),
        Trait::new(60, 0, "山に籠って狩りの技術を磨く", ""),
        Trait::new(61, 0, "エクソシストの下で悪魔祓いの技術を盗む", ""),
        Trait::new(62, 0, "騎士団で高潔な精神を学ぶ", ""),
        Trait::new(63, 0, "達人の下で二刀流における立ち回りを学ぶ", ""),
        Trait::new(64, 0, "薪割りで斧の腕を鍛え敵を斬り刻む", ""),
        Trait::new(65, 0, "毎日の素振りで鈍器の破壊力が増す", ""),
        Trait::new(66, 0, "草原で遠矢の腕を鍛える", ""),
        Trait::new(67, 0, "指揮を執り、仲間の継戦能力を高める", ""),
        Trait::new(68, 0, "達人の下で武器格闘の修行を積む", ""),
        Trait::new(69, 0, "木を殴り続けて拳が硬くなる", ""),
        Trait::new(70, 0, "大陸に渡り鍛冶の技術を学ぶ", ""),
        Trait::new(71, 0, "踊り子の足捌きで戦闘時の速度を上昇させる", ""),
        Trait::new(72, 0, "酒場で歌い続けた経験が歌唱の技術を高める", ""),
        Trait::new(73, 0, "社交界で人を虜にする術を学ぶ", ""),
        Trait::new(74, 0, "娼館で気持ちいいことの技術を学ぶ", ""),
        Trait::new(75, 0, "剣技の熟達が打ち払いとなます斬りを可能とする", ""),
        Trait::new(76, 0, "最前線で重戦士としての経験を積む", ""),
        Trait::new(77, 0, "軍人の知り合いから矢弾の倹約術を教わる", ""),
        Trait::new(78, 0, "多くの書物を読破した経験が速読を可能とする", ""),
        Trait::new(79, 0, "魔術師の下で修行し魔法の威力を高める", ""),
        Trait::new(80, 0, "学問を修め効率的な学習法を習得する", ""),
        Trait::new(81, 0, "農兵としての経験が戦術を高める", ""),
        Trait::new(82, 0, "度重なる死闘が戦術を高める", ""),
        Trait::new(83, 0, "魔術士の下で調合の技術を学ぶ", ""),
        Trait::new(84, 0, "魔術士の下で薬の知識を学ぶ", ""),
        Trait::new(85, 0, "姿勢を正して弓を的中させる", ""),
        Trait::new(86, 0, "晒を巻いて料理の修行に出る", ""),
        Trait::new(87, 0, "一心に投げ続け投擲攻撃の命中と威力を上げる", ""),
        Trait::new(88, 0, "炎を支配し火炎魔法を強化する", ""),
        Trait::new(89, 0, "氷を支配し冷気魔法を強化する", ""),
        Trait::new(90, 0, "雷を支配し電撃魔法を強化する", ""),
        Trait::new(91, 0, "闇を支配し暗黒魔法を強化する", ""),
        Trait::new(92, 0, "夢を支配し幻惑魔法を強化する", ""),
        Trait::new(95, 0, "音を支配し音魔法を強化する", ""),
        Trait::new(96, 0, "裏世界で殺しの技術を磨く", ""),
        Trait::new(97, 0, "仲間に背中を任せて闘いに集中する", ""),
        Trait::new(98, 0, "厳しい修行の末に精霊を封じる力を得る", ""),
        Trait::new(99, 0, "川の主との勝負が釣りの腕を鍛える", ""),
        Trait::new(100, 0, "建設を主導して工費を抑える", ""),
        Trait::new(101, 0, "投資を続けて勘を磨く", ""),
        Trait::new(102, 0, "放浪の末に旅に慣れる", ""),
        Trait::new(103, 0, "菓子職人に弟子入りして菓子作りを学ぶ", ""),
        Trait::new(104, 0, "森暮らしの末に動物と話せるようになる", ""),
        Trait::new(105, 0, "魔獣を調教して仲間を巻き込まないブレスを仕込む", "Train a monster to plant a breath that doesn't involve his friends."),
        Trait::new(150, 2, "火炎耐性", ""),
        Trait::new(151, 2, "冷気耐性", ""),
        Trait::new(152, 2, "毒耐性", ""),
        Trait::new(153, 2, "魔法耐性", ""),
        Trait::new(154, 2, "あなたには追加の成長ボーナスが与えられる", "You receive extra bonus points."),
        Trait::new(155, 2, "暗黒耐性", ""),
        Trait::new(156, 2, "あなたはマナの反動を軽減できる", "You take less damages from the mana reaction."),
        Trait::new(157, 2, "あなたは朦朧状態にならない", "You won't be dim."),
        Trait::new(158, 2, "あなたの食料の消化は遅い", "Your digestion is slow."),
        Trait::new(159, 2, "あなたはより多く採取できる", "You can gather more materials."),
        Trait::new(160, 2, "あなたは高い耐性をもっている", "You have outstanding resistances."),
        Trait::new(161, 2, "あなたは1s以上の物を装備できない[DV上昇]", "You can't wear equipment weight more than 1s. [DV++]"),
        Trait::new(162, 2, "あなたは罪悪感を感じない[カルマ上限-20]", "You don't feel guilty. [Karma limit -20]"),
        Trait::new(163, 2, "あなたの周りでは質の高いエンチャントが生成される", "Quality stuff are generated around you."),
        Trait::new(164, 2, "あなたが受ける物理ダメージは軽減される", "You are given physical damage reduction."),
        Trait::new(165, 2, "あなたの元素魔法は強化されている", "Elemental spells you cast are empowered."),
        Trait::new(166, 2, "あなたは周囲の狂気を緩和する", "You are surrounded by an aura that cures sanity."),
        Trait::new(167, 2, "あなたは萌える", "You moe."),
        Trait::new(168, 2, "あなたのエーテル病の進行は遅い", "Your body slows the Ether Disease progress."),
        Trait::new(169, 2, "あなたは良い心を持っている[カルマ上限+20]", "You are a good man. [Karma limit +20]"),
        Trait::new(170, 2, "あなたの周りでは多くの財宝が生成される", "More treasure are generated around you."),
        Trait::new(171, 0, "魔法の素質を失う", ""),
        Trait::new(172, 0, "虚弱体質により体力を失う", ""),
        Trait::new(173, 0, "判断力の欠如が行動を躊躇う", ""),
        Trait::new(174, 0, "他の神の存在を認めない", ""),
        Trait::new(175, 0, "神の存在を認めない", ""),
        Trait::new(176, 0, "異常な食欲が際限無く食べ物を求める", ""),
        Trait::new(177, 0, "世渡り下手が他者からの反感を招く", ""),
        Trait::new(178, 0, "貞操観念が気持ちいいことを禁ずる", ""),
        Trait::new(179, 0, "生涯に渡る貞潔を宣立する", ""),
        Trait::new(180, 0, "射撃の快感に囚われる", ""),
        Trait::new(181, 0, "独特の倫理観が殺人を恐れる", ""),
        Trait::new(182, 0, "育ちの違いが読み書きを許さない", ""),
        Trait::new(183, 0, "何故か動物に嫌われる", ""),
        Trait::new(184, 0, "相容れない精神から味方を失う", ""),
        Trait::new(185, 0, "爛れた日々を送る", ""),
        Trait::new(186, 0, "酒浸りの日々を送る", ""),
        Trait::new(187, 0, "博打の快感に囚われる", ""),
        Trait::new(188, 0, "決して手を血で汚さない", ""),
    ];
    Ok(TraitCollection { list: traits })
}

#[tauri::command]
async fn get_trait_values() -> Result<TraitValueCollection, String> {
    let trait_values: Vec<TraitValue> = vec![
        TraitValue::new(0, 1, "あなたの肉体は刻々と変化する", "Your body is changing every moment."),
        TraitValue::new(1, 1, "あなたは幸運の持ち主だ", "You are lucky"),
        TraitValue::new(1, 2, "あなたは類稀な幸運の持ち主だ", "You can rely on a good dose of luck."),
        TraitValue::new(1, 3, "あなたは幸運の女神の寵愛を受けている", "The goddess of luck smiles on you."),
        TraitValue::new(2, 1, "あなたは苦行者の見習いだ[HP+5%]", "You are an apprentice of an ascetic. [HP+5%]"),
        TraitValue::new(2, 2, "あなたは立派な苦行者だ[HP+10%]", "You are a journeyman of an ascetic. [HP+10%]"),
        TraitValue::new(2, 3, "あなたは熟練した苦行者だ[HP+15%]", "You are an expert of an ascetic. [HP+15%]"),
        TraitValue::new(2, 4, "あなたは苦行の達人だ[HP+20%]", "You are a master of an ascetic. [HP+20%]"),
        TraitValue::new(2, 5, "あなたは伝説的名苦行者だ[HP+25%]", "You are an legendary ascetic. [HP+25%]"),
        TraitValue::new(3, 1, "あなたは血を吸うことができる", "You can suck blood now."),
        TraitValue::new(4, 1, "あなたは速く走ることができる[速度+5]", "You can run fast. [SPD+5]"),
        TraitValue::new(4, 2, "あなたは猫よりも速い[速度+10]", "You can run faster than a cat. [SPD+10]"),
        TraitValue::new(5, 1, "あなたは腕相撲が強い[筋力+3]", "You are an arm wrestler. [STR+3]"),
        TraitValue::new(5, 2, "あなたは腕相撲の達人だ[筋力+6]", "You are a good arm wrestler. [STR+6]"),
        TraitValue::new(5, 3, "あなたは腕相撲のチャンピオンだ[筋力+9]", "You are a master of arm wrestling. [STR+9]"),
        TraitValue::new(6, 1, "あなたは罠に注意している[探知の潜在能力上昇+30%]", "You are cautious about traps. [Increases the potential rate Detection by 30]"),
        TraitValue::new(6, 2, "あなたは罠に警戒している[探知の潜在能力上昇+60%]", "You are very cautious about traps. [Increases the potential rate Detection by 60]"),
        TraitValue::new(7, 1, "あなたの防御は石のように硬い[PV+5]", "Your defense is as hard as stone. [PV+5]"),
        TraitValue::new(7, 2, "あなたの守備は鉄壁だ[PV+10]", "Your defense is as hard as iron. [PV+10]"),
        TraitValue::new(7, 3, "あなたの防御は鋼のように固い[PV+15]", "Your defense is as hard as steel. [PV+15]"),
        TraitValue::new(8, 1, "あなたは見切れる[見切りの潜在能力上昇+40%]", "You can somewhat dodge enemy attacks. [Increases the potential rate Greater Evasion by 40]"),
        TraitValue::new(8, 2, "あなたはかなり見切れる[見切りの潜在能力上昇+80%]", "You can dodge enemy attacks. [Increases the potential rate Greater Evasion by 80]"),
        TraitValue::new(8, 3, "あなたの見切りは伝説的だ[見切りの潜在能力上昇+120%]", "You can really dodge enemy attacks. [Increases the potential rate Greater Evasion by 120]"),
        TraitValue::new(9, 1, "あなたは我慢強い[耐久+3]", "You are tough. [CON+3]"),
        TraitValue::new(9, 2, "あなたはとても我慢強い[耐久+6]", "You are pretty tough. [CON+6]"),
        TraitValue::new(9, 3, "あなたは恐ろしく我慢強い[耐久+9]", "You are awfully tough. [CON+9]"),
        TraitValue::new(10, 1, "あなたは回避に長けている[回避の潜在能力上昇+20%]", "You can somewhat evade enemy attacks. [Increases the potential rate Evasion by 20]"),
        TraitValue::new(10, 2, "あなたは華麗に回避する[回避の潜在能力上昇+40%]", "You can evade enemy attacks. [Increases the potential rate Evasion by 40]"),
        TraitValue::new(10, 3, "あなたは回避の達人だ[回避の潜在能力上昇+60%]", "You can really evade enemy attacks. [Increases the potential rate Evasion by 60]"),
        TraitValue::new(11, 1, "あなたは魔力の遺伝子を持っている[MP+5%]", "Your genes contain magic. [MP+5%]"),
        TraitValue::new(11, 2, "あなたは魔力の細胞を持っている[MP+10%]", "Your cells contain magica [MP+10%]"),
        TraitValue::new(11, 3, "あなたには魔力の血液が流れている[MP+15%]", "Your blood contain magic [MP+15%]"),
        TraitValue::new(11, 4, "あなたの肉体は魔力を帯びている[MP+20%]", "Your body is made of magic. [MP+20%]"),
        TraitValue::new(11, 5, "あなたは魔力を支配している[MP+25%]", "Magic dominats your entire body. [MP+25%"),
        TraitValue::new(12, 1, "あなたの集中力は高い[詠唱の潜在能力上昇+40%]", "You concentrate while casting. [Increases the potential rate Casting by 40]"),
        TraitValue::new(12, 2, "あなたは極度に集中できる[詠唱の潜在能力上昇+80%]", "You concentrate more while casting. [Increases the potential rate Casting by 80]"),
        TraitValue::new(13, 1, "あなたは空間を曲げられる", "You can insta-teleport to nearby tiles."),
        TraitValue::new(14, 1, "あなたは口から火を噴ける", "You can breath fire."),
        TraitValue::new(15, 1, "あなたは暗闇に慣れている[暗黒耐性強化]", "You don't fear darkness. [RES Darkness+]"),
        TraitValue::new(15, 2, "あなたは闇の中で生活できる[暗黒耐性強化]", "You can dance in darkness. [RES Darkness++]"),
        TraitValue::new(16, 1, "あなたは交渉が上手い[交渉の潜在能力上昇+40%]", "You are a negotiator. [Increases the potential rate Negotiation by 40]"),
        TraitValue::new(16, 2, "あなたの交渉は神ががっている[交渉の潜在能力上昇+80%]", "You are a great negotiator. [Increases the potential rate Negotiation by 80]"),
        TraitValue::new(17, 1, "あなたは信仰に篤い[信仰の潜在能力上昇+40%]", "You really believe in god. [Increases the potential rate Faith by 40]"),
        TraitValue::new(17, 2, "あなたは猛烈な信者だ[信仰の潜在能力上昇+80%]", "You are a zealot. [Increases the potential rate Faith by 80]"),
        TraitValue::new(18, 1, "あなたは毒に免疫がある[毒耐性強化]", "You have a torerance to poison. [RES Poison+]"),
        TraitValue::new(18, 2, "あなたには毒に強い免疫がある[毒耐性強化]", "You have a strong tolerance to poison. [RES Poison++]"),
        TraitValue::new(19, 1, "あなたは2.0sまでの武器を片手で扱える", ""),
        TraitValue::new(19, 2, "あなたは2.5sまでの武器を片手で扱える", ""),
        TraitValue::new(19, 3, "あなたは3.0sまでの武器を片手で扱える", ""),
        TraitValue::new(19, 4, "あなたは3.5sまでの武器を片手で扱える", ""),
        TraitValue::new(19, 5, "あなたは4.0sまでの武器を片手で扱える", ""),
        TraitValue::new(20, 1, "あなたは腕っぷしが強い[格闘の攻撃倍率+5%]", "[Unarmed Dmg multi+5%]"),
        TraitValue::new(20, 2, "あなたは喧嘩に負けない[格闘の攻撃倍率+15%]", "[Unarmed Dmg multi+15%]"),
        TraitValue::new(20, 3, "あなたは格闘技を会得している[格闘の攻撃倍率+35%]", "You know martial arts. [Unarmed Dmg multi+35%]"),
        TraitValue::new(20, 4, "あなたは格闘技の達人だ[格闘の攻撃倍率+70%]", "[Unarmed Dmg multi+70%]"),
        TraitValue::new(20, 5, "あなたは拳聖だ[格闘の攻撃倍率+100%]", "You mastered martial arts. [Unarmed Dmg multi+100%]"),
        TraitValue::new(21, 1, "あなたはセクシーだ[魅力+4]", "You are sexy. [CHR+4]"),
        TraitValue::new(21, 2, "あなたはとんでもない色魔だ[魅力+8] ", "You are a lady killer. [CHR+8]"),
        TraitValue::new(22, 1, "あなたは催眠をかけることができる", "You can hypnotize single target."),
        TraitValue::new(23, 1, "あなたは毒を盛ることができる", "You can poison a creature."),
        TraitValue::new(24, 1, "あなたは持久力がある[スタミナ+]", "You have good stamina. [Stamina+]"),
        TraitValue::new(24, 2, "あなたはかなり持久力がある[スタミナ++]", "You have very good stamina. [Stamina++]"),
        TraitValue::new(24, 3, "あなたの持久力は恐ろしい[スタミナ上昇]", "You have outstanding stamina. [Stamina+++]"),
        TraitValue::new(38, 1, "あなたは税金の計算に強い[税金7%減]", "You can set expenses off against tax. [TAX-7%]"),
        TraitValue::new(38, 2, "あなたは税金の計算にとても強い[税金15%減]", "You are good at setting expenses off against tax. [TAX-15%]"),
        TraitValue::new(39, 1, "あなたはたまに質の高い補給品を受け取る", "You sometimes receive quality supplies."),
        TraitValue::new(39, 2, "あなたは頻繁に質の高い補給品を受け取る", "You frequently receive quality supplies."),
        TraitValue::new(40, 1, "あなたは仲間を一時的に強くできる", "You can temporally strengthen your allies."),
        TraitValue::new(42, 1, "あなたの祈りは呪いの言葉を掻き消す", "Your prayer nullifies cursed whisperings."),
        TraitValue::new(43, 1, "あなたは盾で攻撃する機会を多く得る", "You can bash with your shield more frequently."),
        TraitValue::new(43, 2, "あなたは盾で追加攻撃する機会を得る", " "),
        TraitValue::new(43, 3, "あなたは盾で追加攻撃する機会を多く得る", " "),
        TraitValue::new(44, 1, "あなたの攻撃は敵を恐怖させない", "Your attacks don't cause enemies to run."),
        TraitValue::new(46, 1, "あなたは長剣の扱いを知っている[長剣の攻撃倍率+5%]", "[Long Sword Dmg multi+5%]"),
        TraitValue::new(46, 2, "あなたは長剣の扱いに長けている[長剣の攻撃倍率+15%]", "[Long Sword Dmg multi+15%]"),
        TraitValue::new(46, 3, "あなたは長剣を巧みに操る[長剣の攻撃倍率+35%]", "[Long Sword Dmg multi+35%]"),
        TraitValue::new(46, 4, "あなたは長剣の達人だ[長剣の攻撃倍率+70%]", "[Long Sword Dmg multi+70%]"),
        TraitValue::new(46, 5, "あなたは長剣を極めている[長剣の攻撃倍率+100%]", "[Long Sword Dmg multi+100%]"),
        TraitValue::new(47, 1, "あなたは短剣の扱いを知っている[短剣の攻撃倍率+5%]", "[Short Sword Dmg multi+5%]"),
        TraitValue::new(47, 2, "あなたは短剣の扱いに長けている[短剣の攻撃倍率+15%]", "[Short Sword Dmg multi+15%]"),
        TraitValue::new(47, 3, "あなたは短剣を巧みに操る[短剣の攻撃倍率+35%]", "[Short Sword Dmg multi+35%]"),
        TraitValue::new(47, 4, "あなたは短剣の達人だ[短剣の攻撃倍率+70%]", "[Short Sword Dmg multi+70%]"),
        TraitValue::new(47, 5, "あなたは短剣を極めている[短剣の攻撃倍率+100%]", "[Short Sword Dmg multi+100%]"),
        TraitValue::new(48, 1, "あなたは斧の扱いを知っている[斧の攻撃倍率+5%]", "[Axe Dmg multi+5%]"),
        TraitValue::new(48, 2, "あなたは斧の扱いに長けている[斧の攻撃倍率+15%]", "[Axe Dmg multi+15%]"),
        TraitValue::new(48, 3, "あなたは斧を巧みに操る[斧の攻撃倍率+35%]", "[Axe Dmg multi+35%]"),
        TraitValue::new(48, 4, "あなたは斧の達人だ[斧の攻撃倍率+70%]", "[Axe Dmg multi+70%]"),
        TraitValue::new(48, 5, "あなたは斧を極めている[斧の攻撃倍率+100%]", "[Axe Dmg multi+100%]"),
        TraitValue::new(49, 1, "あなたは鈍器の扱いを知っている[鈍器の攻撃倍率+5%]", "[Blunt Dmg multi+5%]"),
        TraitValue::new(49, 2, "あなたは鈍器の扱いに長けている[鈍器の攻撃倍率+15%]", "[Blunt Dmg multi+15%]"),
        TraitValue::new(49, 3, "あなたは鈍器を巧みに操る[鈍器の攻撃倍率+35%]", "[Blunt Dmg multi+35%]"),
        TraitValue::new(49, 4, "あなたは鈍器の達人だ[鈍器の攻撃倍率+70%]", "[Blunt Dmg multi+70%]"),
        TraitValue::new(49, 5, "あなたは鈍器を極めている[鈍器の攻撃倍率+100%]", "[Blunt Dmg multi+100%]"),
        TraitValue::new(50, 1, "あなたは槍の扱いを知っている[槍の攻撃倍率+5%]", "[Polearm Dmg multi+5%]"),
        TraitValue::new(50, 2, "あなたは槍の扱いに長けている[槍の攻撃倍率+15%]", "[Polearm Dmg multi+15%]"),
        TraitValue::new(50, 3, "あなたは槍を巧みに操る[槍の攻撃倍率+35%]", "[Polearm Dmg multi+35%]"),
        TraitValue::new(50, 4, "あなたは槍の達人だ[槍の攻撃倍率+70%]", "[Polearm Dmg multi+70%]"),
        TraitValue::new(50, 5, "あなたは槍を極めている[槍の攻撃倍率+100%]", "[Polearm Dmg multi+100%]"),
        TraitValue::new(51, 1, "あなたは杖の扱いを知っている[杖の攻撃倍率+5%]", "[Stave Dmg multi+5%]"),
        TraitValue::new(51, 2, "あなたは杖の扱いに長けている[杖の攻撃倍率+15%]", "[Stave Dmg multi+15%]"),
        TraitValue::new(51, 3, "あなたは杖を巧みに操る[杖の攻撃倍率+35%]", "[Stave Dmg multi+35%]"),
        TraitValue::new(51, 4, "あなたは杖の達人だ[杖の攻撃倍率+70%]", "[Stave Dmg multi+70%]"),
        TraitValue::new(51, 5, "あなたは杖を極めている[杖の攻撃倍率+100%]", "[Stave Dmg multi+100%]"),
        TraitValue::new(53, 1, "あなたは鎌の扱いを知っている[鎌の攻撃倍率+5%]", "[Scythe Dmg multi+5%]"),
        TraitValue::new(53, 2, "あなたは鎌の扱いに長けている[鎌の攻撃倍率+15%]", "[Scythe Dmg multi+15%]"),
        TraitValue::new(53, 3, "あなたは鎌を巧みに操る[鎌の攻撃倍率+35%]", "[Scythe Dmg multi+35%]"),
        TraitValue::new(53, 4, "あなたは鎌の達人だ[鎌の攻撃倍率+70%]", "[Scythe Dmg multi+70%]"),
        TraitValue::new(53, 5, "あなたは鎌を極めている[鎌の攻撃倍率+100%]", "[Scythe Dmg multi+100%]"),
        TraitValue::new(54, 1, "あなたは弓の扱いを知っている[弓の攻撃倍率+5%]", "[Bow Dmg multi+5%]"),
        TraitValue::new(54, 2, "あなたは弓の扱いに長けている[弓の攻撃倍率+15%]", "[Bow Dmg multi+15%]"),
        TraitValue::new(54, 3, "あなたは弓を巧みに操る[弓の攻撃倍率+35%]", "[Bow Dmg multi+35%]"),
        TraitValue::new(54, 4, "あなたは弓の達人だ[弓の攻撃倍率+70%]", "[Bow Dmg multi+70%]"),
        TraitValue::new(54, 5, "あなたは弓を極めている[弓の攻撃倍率+100%]", "[Bow Dmg multi+100%]"),
        TraitValue::new(55, 1, "あなたはクロスボウの扱いを知っている[クロスボウの攻撃倍率+5%]", "[Crossbow Dmg multi+5%]"),
        TraitValue::new(55, 2, "あなたはクロスボウの扱いに長けている[クロスボウの攻撃倍率+15%]", "[Crossbow Dmg multi+15%]"),
        TraitValue::new(55, 3, "あなたはクロスボウを巧みに操る[クロスボウの攻撃倍率+35%]", "[Crossbow Dmg multi+35%]"),
        TraitValue::new(55, 4, "あなたはクロスボウの達人だ[クロスボウの攻撃倍率+70%]", "[Crossbow Dmg multi+70%]"),
        TraitValue::new(55, 5, "あなたはクロスボウを極めている[クロスボウの攻撃倍率+100%]", "[Crossbow Dmg multi+100%]"),
        TraitValue::new(56, 1, "あなたは銃器の扱いを知っている[銃器の攻撃倍率+5%]", "[Firearm Dmg multi+5%]"),
        TraitValue::new(56, 2, "あなたは銃器の扱いに長けている[銃器の攻撃倍率+15%]", "[Firearm Dmg multi+15%]"),
        TraitValue::new(56, 3, "あなたは銃器を巧みに操る[銃器の攻撃倍率+35%]", "[Firearm Dmg multi+35%]"),
        TraitValue::new(56, 4, "あなたは銃器の達人だ[銃器の攻撃倍率+70%]", "[Firearm Dmg multi+70%]"),
        TraitValue::new(56, 5, "あなたは銃器を極めている[銃器の攻撃倍率+100%]", "[Firearm Dmg multi+100%]"),
        TraitValue::new(57, 1, "あなたは投擲の扱いを知っている[投擲の攻撃倍率+5%]", "[Throwing Dmg multi+5%]"),
        TraitValue::new(57, 2, "あなたは投擲の扱いに長けている[投擲の攻撃倍率+15%]", "[Throwing Dmg multi+15%]"),
        TraitValue::new(57, 3, "あなたは投擲を巧みに操る[投擲の攻撃倍率+35%]", "[Throwing Dmg multi+35%]"),
        TraitValue::new(57, 4, "あなたは投擲の達人だ[投擲の攻撃倍率+70%]", "[Throwing Dmg multi+70%]"),
        TraitValue::new(57, 5, "あなたは投擲を極めている[投擲の攻撃倍率+100%]", "[Throwing Dmg multi+100%]"),
        TraitValue::new(58, 1, "あなたは致死ダメージを受けても死なず、少し回復する", ""),
        TraitValue::new(58, 2, "あなたは致死ダメージを受けても死なず、回復する", ""),
        TraitValue::new(58, 3, "あなたは致死ダメージを受けても死なず、かなり回復する", ""),
        TraitValue::new(58, 4, "あなたは致死ダメージを受けても死なず、全快する", ""),
        TraitValue::new(59, 1, "あなたは激怒する", ""),
        TraitValue::new(59, 2, "あなたは敵に容赦しない[敵発見時に怒り]", ""),
        TraitValue::new(59, 3, "あなたは殺戮兵器だ[敵発見時に怒り 怒り強化]", ""),
        TraitValue::new(60, 1, "あなたは野生動物に対して威力をよく通す", ""),
        TraitValue::new(60, 2, "あなたは野生動物に対して強力な威力を発揮する", ""),
        TraitValue::new(61, 1, "あなたは悪魔に対して威力をよく通す", ""),
        TraitValue::new(61, 2, "あなたは悪魔に対して強力な威力を発揮する", ""),
        TraitValue::new(62, 1, "あなたは仲間をかばえる", ""),
        TraitValue::new(63, 1, "あなたは二刀流を修めている[二刀流で回避+(二刀流の平方根+二刀流の10%)]", ""),
        TraitValue::new(63, 2, "あなたは複数の武器を巧みに操る[二刀流で回避+(二刀流の平方根+二刀流の30%)]", ""),
        TraitValue::new(63, 3, "あなたは二刀流を極めている[二刀流で回避+(二刀流の平方根+二刀流の50%)]", ""),
        TraitValue::new(64, 1, "あなたは斧で浅い切り傷を負わせる機会を得る", ""),
        TraitValue::new(64, 2, "あなたは斧で切り傷を負わせる機会を得る", ""),
        TraitValue::new(64, 3, "あなたは斧で深い切り傷を負わせる機会を得る", ""),
        TraitValue::new(65, 1, "あなたは鈍器で朦朧とさせる機会をごく稀に得る", ""),
        TraitValue::new(65, 2, "あなたは鈍器で朦朧とさせる機会を稀に得る", ""),
        TraitValue::new(65, 3, "あなたは鈍器で朦朧とさせる機会を得る", ""),
        TraitValue::new(66, 1, "あなたの弓は遠くの敵を射抜ける", ""),
        TraitValue::new(66, 2, "あなたの弓は遠くの敵を正確に射抜ける", ""),
        TraitValue::new(66, 3, "あなたの弓は遠くの敵を確実に射抜ける", ""),
        TraitValue::new(67, 1, "あなたの指揮は適切だ[仲間の治癒+(あなたの治癒の平方根+あなたの治癒の10%) 仲間の瞑想+(あなたの瞑想の平方根+あなたの瞑想の10%)]", ""),
        TraitValue::new(67, 2, "あなたの指揮は優れている[仲間の治癒+(あなたの治癒の平方根+あなたの治癒の30%) 仲間の瞑想+(あなたの瞑想の平方根+あなたの瞑想の30%)]", ""),
        TraitValue::new(67, 3, "あなたの指揮は伝説的だ[仲間の治癒+(あなたの治癒の平方根+あなたの治癒の50%) 仲間の瞑想+(あなたの瞑想の平方根+あなたの瞑想の50%)]", ""),
        TraitValue::new(68, 1, "あなたは武器を身体の一部のように扱える[両手持ちで武器+(格闘の5%)]", ""),
        TraitValue::new(68, 2, "あなたは武器を身体の一部のように扱える[両手持ちで武器+(格闘の15%)]", ""),
        TraitValue::new(68, 3, "あなたは武器を身体の一部のように扱える[両手持ちで武器+(格闘の35%)]", ""),
        TraitValue::new(68, 4, "あなたは武器を身体の一部のように扱える[両手持ちで武器+(格闘の70%)]", ""),
        TraitValue::new(68, 5, "あなたは武器を身体の一部のように扱える[両手持ちで武器+(格闘の100%)]", ""),
        TraitValue::new(69, 1, "あなたの拳は鍛えられている[稀に朦朧させる クリティカルの貫通率+5%]", ""),
        TraitValue::new(69, 2, "あなたの拳は岩を砕く[稀に朦朧させる クリティカルの貫通率+15%]", ""),
        TraitValue::new(69, 3, "あなたの拳は鉄をねじ曲げる[稀に朦朧させる クリティカルの貫通率+35%]", ""),
        TraitValue::new(69, 4, "あなたの拳は鉄を打ち抜く[稀に朦朧させる クリティカルの貫通率+70%]", ""),
        TraitValue::new(69, 5, "あなたの拳は全てを砕く[稀に朦朧させる クリティカルの貫通率+100%]", ""),
        TraitValue::new(70, 1, "あなたは鍛冶で傑作を作り上げる機会を多く得る", ""),
        TraitValue::new(70, 2, "あなたは鍛冶で傑作を作り上げる機会をとても多く得る", ""),
        TraitValue::new(70, 3, "あなたは鍛冶で傑作を作り上げる機会を非常に多く得る", ""),
        TraitValue::new(71, 1, "あなたは敵発見時に加速する[20%]", ""),
        TraitValue::new(71, 2, "あなたは敵発見時に加速する[60%]", ""),
        TraitValue::new(71, 3, "あなたは敵発見時に加速する[100%]", ""),
        TraitValue::new(72, 1, "あなたの歌唱は魅了的だ[歌唱力+ 魅了10% 報酬品質+]", ""),
        TraitValue::new(72, 2, "あなたの歌唱は多くを魅了する[歌唱力++ 魅了30% 報酬品質+]", ""),
        TraitValue::new(72, 3, "あなたの歌唱は全てを魅了する[歌唱力+++ 魅了70% 報酬品質+]", ""),
        TraitValue::new(73, 1, "あなたの話は面白い[興味+ 友好上限+]", ""),
        TraitValue::new(73, 2, "あなたの話はとても面白い[興味++ 友好上限++]", ""),
        TraitValue::new(73, 3, "あなたの話術は相手を虜にできる[興味+++ 友好上限+++]", ""),
        TraitValue::new(74, 1, "あなたの気持ちいいことは評判だ", ""),
        TraitValue::new(74, 2, "あなたは気持ちいいことの達人だ", ""),
        TraitValue::new(74, 3, "あなたは気持ちいいことを極めている", ""),
        TraitValue::new(75, 1, "あなたは敵の攻撃を捌ける[被るダメージを稀に無効]", ""),
        TraitValue::new(75, 2, "あなたは剣術の達人だ[被るダメージを稀に無効 追加打撃の回数+]", ""),
        TraitValue::new(75, 3, "あなたは伝説的名剣士だ[被るダメージを稀に無効 追加打撃の回数++]", ""),
        TraitValue::new(76, 1, "あなたは11.0sまでの武器を扱える[クリティカルダメージ強化]", ""),
        TraitValue::new(76, 2, "あなたは13.0sまでの武器を扱える[クリティカルダメージ強化]", ""),
        TraitValue::new(76, 3, "あなたは17.0sまでの武器を扱える[クリティカルダメージ強化]", ""),
        TraitValue::new(76, 4, "あなたは24.0sまでの武器を扱える[クリティカルダメージ強化]", ""),
        TraitValue::new(76, 5, "あなたは30.0sまでの武器を扱える[クリティカルダメージ強化]", ""),
        TraitValue::new(77, 1, "あなたは稀に特殊弾の消費を抑える[10%]", ""),
        TraitValue::new(77, 2, "あなたは特殊弾の消費を抑える[30%]", ""),
        TraitValue::new(77, 3, "あなたは特殊弾の倹約術を極めている[70%]", ""),
        TraitValue::new(78, 1, "あなたは魔法書を速読できる", " "),
        TraitValue::new(79, 1, "あなたは魔法の威力が最大になる機会を得る[1% 消費ストック+]", ""),
        TraitValue::new(79, 2, "あなたは魔法の威力が最大になる機会を得る[3% 消費ストック+]", ""),
        TraitValue::new(79, 3, "あなたは魔法の威力が最大になる機会を得る[5% 消費ストック+]", ""),
        TraitValue::new(79, 4, "あなたは魔法の威力が最大になる機会を得る[7% 消費ストック+]", ""),
        TraitValue::new(79, 5, "あなたは魔法の威力が最大になる機会を得る[10% 消費ストック+]", ""),
        TraitValue::new(80, 1, "あなたは頭が良い[魔法の潜在能力回復量+]", ""),
        TraitValue::new(80, 2, "あなたはとても頭がいい[魔法の潜在能力回復量++]", ""),
        TraitValue::new(80, 3, "あなたの頭脳は国宝級だ[魔法の潜在能力回復量+++]", ""),
        TraitValue::new(81, 1, "あなたは武器を鍬のように使える[戦術+(栽培の平方根+栽培の5%)]", ""),
        TraitValue::new(81, 2, "あなたは農兵だ[戦術+(栽培の平方根+栽培の15%)]", ""),
        TraitValue::new(81, 3, "あなたは優れた農兵だ[戦術+(栽培の平方根+栽培の35%)]", ""),
        TraitValue::new(82, 1, "あなたは歴戦の戦士だ[戦術の潜在能力上昇+40%]", "[Increases the potential rate Tactics by 40]"),
        TraitValue::new(82, 2, "あなたは戦いの修羅だ[戦術の潜在能力上昇+80%]", "[Increases the potential rate Tactics by 80]"),
        TraitValue::new(83, 1, "あなたはポーションを調合できる[成功率を10%上昇]", ""),
        TraitValue::new(83, 2, "あなたはポーションの調合が上手い[成功率を30%上昇]", ""),
        TraitValue::new(83, 3, "あなたの調合は大評判だ[成功率を50%上昇]", ""),
        TraitValue::new(84, 1, "あなたはポーションの用法を知っている[ポーションの効果+10%]", ""),
        TraitValue::new(84, 2, "あなたはポーションに詳しい[ポーションの効果+30%]", ""),
        TraitValue::new(84, 3, "あなたはポーションの全てを知っている[ポーションの効果+50%]", ""),
        TraitValue::new(85, 1, "あなたの弓はよく当たる[弓の命中+(弓の平方根+弓の10%)]", ""),
        TraitValue::new(85, 2, "あなたの弓は正確だ[弓の命中+(弓の平方根+弓の30%)]", ""),
        TraitValue::new(85, 3, "あなたの弓は的を外さない[弓の命中+(弓の平方根+弓の50%)]", ""),
        TraitValue::new(86, 1, "あなたは料理人の見習いだ[料理の消費SP-10%]", ""),
        TraitValue::new(86, 2, "あなたは料理人だ[料理の消費SP-30%]", ""),
        TraitValue::new(86, 3, "あなたは料理の達人だ[料理の消費SP-50%]", ""),
        TraitValue::new(87, 1, "あなたは遠投上手だ", ""),
        TraitValue::new(87, 2, "あなたは遠投の達人だ", ""),
        TraitValue::new(87, 3, "あなたの遠投は伝説的だ", ""),
        TraitValue::new(88, 1, "あなたの火炎魔法は強化されている[+10%]", "Fire spells you cast are empowered. [+10%]"),
        TraitValue::new(88, 2, "あなたの火炎魔法は強化されている[+30% ボール標的指定可]", "Fire spells you cast are empowered. [+30% Target ball]"),
        TraitValue::new(88, 3, "あなたの火炎魔法は強化されている[+50% ボール標的指定可]", "Fire spells you cast are empowered. [+50% Target ball]"),
        TraitValue::new(89, 1, "あなたの冷気魔法は強化されている[+10%]", "Cold spells you cast are empowered. [+10%]"),
        TraitValue::new(89, 2, "あなたの冷気魔法は強化されている[+30% ボール標的指定可]", "Cold spells you cast are empowered. [+30% Target ball]"),
        TraitValue::new(89, 3, "あなたの冷気魔法は強化されている[+50% ボール標的指定可]", "Cold spells you cast are empowered. [+50% Target ball]"),
        TraitValue::new(90, 1, "あなたの電撃魔法は強化されている[+10%]", "Lightning spells you cast are empowered. [+10%]"),
        TraitValue::new(90, 2, "あなたの電撃魔法は強化されている[+30% ボール標的指定可]", "Lightning spells you cast are empowered. [+30% Target ball]"),
        TraitValue::new(90, 3, "あなたの電撃魔法は強化されている[+50% ボール標的指定可]", "Lightning spells you cast are empowered. [+50% Target ball]"),
        TraitValue::new(91, 1, "あなたの暗黒魔法は強化されている[+10%]", "Darkness spells you cast are empowered. [+10%]"),
        TraitValue::new(91, 2, "あなたの暗黒魔法は強化されている[+30% ボール標的指定可]", "Darkness spells you cast are empowered. [+30% Target ball]"),
        TraitValue::new(91, 3, "あなたの暗黒魔法は強化されている[+50% ボール標的指定可]", "Darkness spells you cast are empowered. [+50% Target ball]"),
        TraitValue::new(92, 1, "あなたの幻惑魔法は強化されている[+10%]", "Mind spells you cast are empowered. [+10%]"),
        TraitValue::new(92, 2, "あなたの幻惑魔法は強化されている[+30% ボール標的指定可]", "Mind spells you cast are empowered. [+30% Target ball]"),
        TraitValue::new(92, 3, "あなたの幻惑魔法は強化されている[+50% ボール標的指定可]", "Mind spells you cast are empowered. [+50% Target ball]"),
        TraitValue::new(95, 1, "あなたの音魔法は強化されている[+10%]", "Sound spells you cast are empowered. [+10%]"),
        TraitValue::new(95, 2, "あなたの音魔法は強化されている[+30% ボール標的指定可]", "Sound spells you cast are empowered. [+30% Target ball]"),
        TraitValue::new(95, 3, "あなたの音魔法は強化されている[+50% ボール標的指定可]", "Sound spells you cast are empowered. [+50% Target ball]"),
        TraitValue::new(96, 1, "あなたは獲物を仕留める[寝ている相手に攻撃倍率+100%]", "[Sneak attack Bonus+100%]"),
        TraitValue::new(96, 2, "あなたは獲物を確実に仕留める[寝ている相手に攻撃倍率+300%]", "[Sneak attack Bonus+300%]"),
        TraitValue::new(96, 3, "あなたは獲物を絶対に仕留める[寝ている相手に攻撃倍率+500%]", "[Sneak attack Bonus+500%]"),
        TraitValue::new(97, 1, "あなたは勇者だ[仲間のHPが75%以上の時、自分の攻撃と速度+5%]", ""),
        TraitValue::new(97, 2, "あなたは勇者の中の勇者だ[仲間のHPが75%以上の時、自分の攻撃と速度+15%]", ""),
        TraitValue::new(97, 3, "あなたは伝説的勇者だ[仲間のHPが75%以上の時、自分の攻撃と速度+35%]", ""),
        TraitValue::new(98, 1, "あなたは精霊に対して威力をよく通す", ""),
        TraitValue::new(98, 2, "あなたは精霊に対して強力な威力を発揮する", ""),
        TraitValue::new(99, 1, "あなたは釣り上手だ[高価な魚の確率+100%]", ""),
        TraitValue::new(99, 2, "あなたは釣りを極めている[高価な魚の確率+300%]", ""),
        TraitValue::new(100, 1, "あなたは大工の見習いだ[権利書の料金-10%]", "[Deed cost-10%]"),
        TraitValue::new(100, 2, "あなたは大工だ[権利書の料金-30%]", "[Deed cost-30%]"),
        TraitValue::new(100, 3, "あなたは名工だ[権利書の料金-50%]", "[Deed cost-50%]"),
        TraitValue::new(101, 1, "あなたは投資が上手い[投資の潜在能力上昇+40%]", "[Increases the potential rate Investing by 40]"),
        TraitValue::new(101, 2, "あなたの投資は神ががっている[投資の潜在能力上昇+80%]", "[Increases the potential rate Investing by 80]"),
        TraitValue::new(102, 1, "あなたは旅に慣れている[旅歩きの潜在能力上昇+40%]", "[Increases the potential rate Traveling by 40]"),
        TraitValue::new(102, 2, "あなたは旅を極めている[旅歩きの潜在能力上昇+80%]", "[Increases the potential rate Traveling by 80]"),
        TraitValue::new(103, 1, "あなたの作る菓子は美味しい[作る菓子とデザートの経験値+10%]", ""),
        TraitValue::new(103, 2, "あなたの作る菓子はとても美味しい[作る菓子とデザートの経験値+30%]", ""),
        TraitValue::new(103, 3, "あなたの菓子作りは神ががっている[作る菓子とデザートの経験値+50%]", ""),
        TraitValue::new(104, 1, "あなたは野生動物と友達だ", ""),
        TraitValue::new(105, 1, "あなたはペットのブレスの巻き込みを防ぐ[ダメージ軽減+]", "You prevent the entrapment of your pet's breath. [damage-]"),
        TraitValue::new(105, 2, "あなたはペットのブレスの巻き込みを防ぐ[ダメージ軽減++]", "You prevent the entrapment of your pet's breath. [damage--]"),
        TraitValue::new(105, 3, "あなたはペットのブレスの巻き込みを防ぐ[ダメージ軽減+++]", "You prevent the entrapment of your pet's breath. [damage---]"),
        TraitValue::new(150, -2, "あなたはかなり火炎に弱い", "You have strong weakness to fire."),
        TraitValue::new(150, -1, "あなたは火炎に弱い", "You have weakness to fire."),
        TraitValue::new(150, 1, "あなたは火炎に耐性がある", "You have resistance to fire."),
        TraitValue::new(150, 2, "あなたは火炎にかなりの耐性がある", "You have strong resistance to fire."),
        TraitValue::new(151, -2, "あなたはかなり冷気に弱い", "You have strong weakness to cold."),
        TraitValue::new(151, -1, "あなたは冷気に弱い", "You have weakness to cold."),
        TraitValue::new(151, 1, "あなたは冷気に耐性がある", "You have resistance to cold."),
        TraitValue::new(151, 2, "あなたは冷気にかなりの耐性がある", "You have strong resistance to cold."),
        TraitValue::new(152, -2, "あなたはかなり毒に弱い", "You have strong weakness to poison."),
        TraitValue::new(152, -1, "あなたは毒に弱い", "You have weakness to poison."),
        TraitValue::new(152, 1, "あなたは毒に耐性がある", "You have resistance to poison."),
        TraitValue::new(152, 2, "あなたは毒にかなりの耐性がある", "You have strong resistance to poison."),
        TraitValue::new(153, -2, "あなたはかなり魔法に弱い", "You have strong weakness to magic."),
        TraitValue::new(153, -1, "あなたは魔法に弱い", "You have weakness to magic."),
        TraitValue::new(153, 1, "あなたは魔法に耐性がある", "You have resistance to magic."),
        TraitValue::new(153, 2, "あなたは魔法にかなりの耐性がある", "You have strong resistance to magic."),
        TraitValue::new(154, 1, "あなたには追加の成長ボーナスが与えられる", "You receive extra bonus points."),
        TraitValue::new(155, -2, "あなたはかなり暗黒に弱い", "You have strong weakness to darkness."),
        TraitValue::new(155, -1, "あなたは暗黒に弱い", "You have weakness to darkness."),
        TraitValue::new(155, 1, "あなたは暗黒に耐性がある", "You have resistance to darkness."),
        TraitValue::new(155, 2, "あなたは暗黒にかなりの耐性がある", "You have strong resistance to darkness."),
        TraitValue::new(156, 1, "あなたはマナの反動を軽減できる", "You take less damages from the mana reaction."),
        TraitValue::new(157, 1, "あなたは朦朧状態にならない", "You won't be dim."),
        TraitValue::new(158, 1, "あなたの食料の消化は遅い", "Your digestion is slow."),
        TraitValue::new(159, 1, "あなたはより多く採取できる", "You can gather more materials."),
        TraitValue::new(160, 1, "あなたは高い耐性をもっている", "You have outstanding resistances."),
        TraitValue::new(161, 1, "あなたは1s以上の物を装備できない[DV上昇]", "You can't wear equipment weight more than 1s. [DV++]"),
        TraitValue::new(162, 1, "あなたは罪悪感を感じない[カルマ上限-20]", "You don't feel guilty. [Karma limit -20]"),
        TraitValue::new(163, 1, "あなたの周りでは質の高いエンチャントが生成される", "Quality stuff are generated around you."),
        TraitValue::new(164, 1, "あなたが受ける物理ダメージは軽減される", "You are given physical damage reduction."),
        TraitValue::new(165, 1, "あなたの元素魔法は強化されている", "Elemental spells you cast are empowered."),
        TraitValue::new(166, 1, "あなたは周囲の狂気を緩和する", "You are surrounded by an aura that cures sanity."),
        TraitValue::new(167, 1, "あなたは萌える", "You moe."),
        TraitValue::new(168, 1, "あなたのエーテル病の進行は遅い", "Your body slows the Ether Disease progress."),
        TraitValue::new(169, 1, "あなたは良い心を持っている[カルマ上限+20]", "You are a good man. [Karma limit +20]"),
        TraitValue::new(170, 1, "あなたの周りでは多くの財宝が生成される", "More treasure are generated around you."),
        TraitValue::new(171, -1, "あなたは魔法が使えない", "You don't cast spells."),
        TraitValue::new(172, -3, "あなたは今にも死にそうだ[HP-75%]", "[HP-75%]"),
        TraitValue::new(172, -2, "あなたは虚弱だ[HP-50%]", "[HP-50%]"),
        TraitValue::new(172, -1, "あなたは貧弱だ[HP-25%]", "[HP-25%]"),
        TraitValue::new(173, -1, "あなたは優柔不断だ[たまに行動キャンセル]", "You are indecisiveness. [Random cancel action]"),
        TraitValue::new(174, -1, "あなたは改宗できない", "You are not convertion."),
        TraitValue::new(175, -1, "あなたは神を信じない", "You are one's lifelong eyth."),
        TraitValue::new(176, -1, "あなたは唐突に何かを食べずにはいられない[不定期食事]", ""),
        TraitValue::new(177, -1, "あなたは人との関係に軋轢を生みやすい[友好低下]", ""),
        TraitValue::new(178, -1, "あなたは気持ちいいことができない", ""),
        TraitValue::new(179, -1, "あなたは結婚できない", ""),
        TraitValue::new(180, -1, "あなたは射撃中毒だ", "You are trigger happy."),
        TraitValue::new(181, -1, "あなたは人を殺すことに抵抗がある[殺人でカルマ減少]", ""),
        TraitValue::new(182, -1, "あなたは字が読めない", ""),
        TraitValue::new(183, -1, "あなたは動物に嫌われている", ""),
        TraitValue::new(184, -1, "あなたはペットを持てない", "You are not pet."),
        TraitValue::new(185, -1, "あなたは気持ちいいことをしたくてたまらない", ""),
        TraitValue::new(186, -1, "あなたはアルコール依存症だ", ""),
        TraitValue::new(187, -1, "あなたは賭博中毒だ", " "),
        TraitValue::new(188, -1, "あなたは打撃と射撃ができない", "You can't attack and fire."),
    ];
    Ok(TraitValueCollection { list: trait_values })
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
async fn get_items() -> Result<ItemCollection, String> {
    let items: Vec<Item> = vec![
        Item::new("", 0, 0, "指定なし", "not set"),
        Item::new("0", 0, 0, "なし", "none"),
        Item::new("743", 0, 0, "カスタムアイテム", "custom item"),
        Item::new("10000", 10000, 0, "カテゴリ:近接武器", "category: melee weapon"),
        Item::new("12000", 12000, 0, "カテゴリ:頭防具", "category: head"),
        Item::new("14000", 14000, 0, "カテゴリ:盾", "category: shield"),
        Item::new("16000", 16000, 0, "カテゴリ:体防具", "category: body"),
        Item::new("18000", 18000, 0, "カテゴリ:足防具", "category: leg"),
        Item::new("19000", 19000, 0, "カテゴリ:腰防具", "category: waist"),
        Item::new("20000", 20000, 0, "カテゴリ:背中防具", "category: back"),
        Item::new("22000", 22000, 0, "カテゴリ:腕防具", "category: arm"),
        Item::new("24000", 24000, 0, "カテゴリ:遠隔武器", "category: shoot"),
        Item::new("32000", 32000, 0, "カテゴリ:指防具", "category: ring"),
        Item::new("34000", 34000, 0, "カテゴリ:首防具", "category: neck"),
        Item::new("10001", 10000, 10001, "サブカテゴリ:大剣", "sub category: claymore"),
        Item::new("10002", 10000, 10002, "サブカテゴリ:長剣/刀/ライトセーバー", "sub category: "),
        Item::new("10003", 10000, 10003, "サブカテゴリ:短剣/忍刀/包丁", "sub category: long sword"),
        Item::new("10004", 10000, 10004, "サブカテゴリ:棍棒", "sub category: club"),
        Item::new("10005", 10000, 10005, "サブカテゴリ:大槌", "sub category: hammer"),
        Item::new("10006", 10000, 10006, "サブカテゴリ:杖/長杖", "sub category: staff"),
        Item::new("10007", 10000, 10007, "サブカテゴリ:槍", "sub category: spear"),
        Item::new("10008", 10000, 10008, "サブカテゴリ:鉾槍", "sub category: halberd"),
        Item::new("10009", 10000, 10009, "サブカテゴリ:斧", "sub category: axe"),
        Item::new("10010", 10000, 10010, "サブカテゴリ:大斧", "sub category: bardish"),
        Item::new("10011", 10000, 10011, "サブカテゴリ:鎌/大鎌", "sub category: scythe"),
        Item::new("12001", 12000, 12001, "サブカテゴリ:兜", "sub category: helm"),
        Item::new("12002", 12000, 12002, "サブカテゴリ:帽子", "sub category: hat"),
        Item::new("14003", 14000, 14003, "サブカテゴリ:盾", "sub category: shield"),
        Item::new("16001", 16000, 16001, "サブカテゴリ:鎧", "sub category: armor"),
        Item::new("16003", 16000, 16003, "サブカテゴリ:服/法衣", "sub category: robe"),
        Item::new("18001", 18000, 18001, "サブカテゴリ:重靴", "sub category: heavy boots"),
        Item::new("18002", 18000, 18002, "サブカテゴリ:靴", "sub category: boots"),
        Item::new("19001", 19000, 19001, "サブカテゴリ:腰当", "sub category: girdle"),
        Item::new("20001", 20000, 20001, "サブカテゴリ:外套/羽/翼", "sub category: cloak"),
        Item::new("22001", 22000, 22001, "サブカテゴリ:篭手", "sub category: gauntlets"),
        Item::new("22003", 22000, 22003, "サブカテゴリ:細工篭手/手袋", "sub category: gloves"),
        Item::new("24001", 24000, 24001, "サブカテゴリ:短弓/長弓/骨弓", "sub category: bow"),
        Item::new("24003", 24000, 24003, "サブカテゴリ:機械弓", "sub category: crossbow"),
        Item::new("24020", 24000, 24020, "サブカテゴリ:拳銃/機関銃/散弾銃", "sub category: gun"),
        Item::new("24021", 24000, 24021, "サブカテゴリ:光子銃", "sub category: laser gun"),
        Item::new("24030", 24000, 24030, "サブカテゴリ:投擲武器", "sub category: throw"),
        Item::new("25001", 25000, 25001, "サブカテゴリ:矢束", "sub category: arrow"),
        Item::new("25002", 25000, 25002, "サブカテゴリ:クロスボウの矢束", "sub category: bolt"),
        Item::new("25020", 25000, 25020, "サブカテゴリ:弾丸", "sub category: bullet"),
        Item::new("25030", 25000, 25030, "サブカテゴリ:光子弾", "sub category: energy cell"),
        Item::new("32001", 32000, 32001, "サブカテゴリ:指輪", "sub category: ring"),
        Item::new("34001", 34000, 34001, "サブカテゴリ:首輪/ペンダント", "sub category: amulet"),
        Item::new("1", 10000, 10002, "長剣", "long sword"),
        Item::new("2", 10000, 10003, "短剣", "dagger"),
        Item::new("3", 10000, 10009, "手斧", "hand axe"),
        Item::new("4", 10000, 10004, "棍棒", "club"),
        Item::new("5", 12000, 12002, "魔法帽", "magic hat"),
        Item::new("6", 12000, 12002, "フェアリーハット", "fairy hat"),
        Item::new("7", 16000, 16001, "鎧", "breastplate"),
        Item::new("8", 16000, 16003, "法衣", "robe"),
        Item::new("9", 22000, 22003, "細工篭手", "decorated gloves"),
        Item::new("10", 22000, 22001, "厚篭手", "thick gauntlets"),
        Item::new("11", 18000, 18001, "重靴", "heavy boots"),
        Item::new("12", 18000, 18001, "合成靴", "composite boots"),
        Item::new("13", 32000, 32001, "装飾の指輪", "decorative ring"),
        Item::new("56", 10000, 10002, "《ディアボロス》", "<Diablo>"),
        Item::new("57", 10000, 10002, "《斬鉄剣》", "<Zantetsu>"),
        Item::new("58", 24000, 24001, "長弓", "long bow"),
        Item::new("59", 14000, 14003, "騎士盾", "knight shield"),
        Item::new("60", 24000, 24020, "拳銃", "pistol"),
        Item::new("61", 25000, 25001, "矢束", "arrow"),
        Item::new("62", 25000, 25020, "弾丸", "bullet"),
        Item::new("63", 10000, 10011, "《虚無の大鎌》", "<Scythe of Void>"),
        Item::new("64", 10000, 10002, "《モーンブレイド》", "<Mournblade>"),
        Item::new("65", 20000, 20001, "軽外套", "light cloak"),
        Item::new("66", 19000, 19001, "腰当", "girdle"),
        Item::new("67", 34000, 34001, "装飾首輪", "decorative amulet"),
        Item::new("73", 10000, 10002, "《ラグナロク》", "<Ragnarok>"),
        Item::new("206", 10000, 10003, "《エーテルダガー》", "<Ether Dagger>"),
        Item::new("207", 24000, 24001, "《異形の森の弓》", "<Bow of Vinderre>"),
        Item::new("210", 24000, 24030, "小石", "stone"),
        Item::new("211", 10000, 10011, "鎌", "sickle"),
        Item::new("212", 10000, 10006, "杖", "staff"),
        Item::new("213", 10000, 10007, "長槍", "spear"),
        Item::new("224", 10000, 10002, "刀", "katana"),
        Item::new("225", 10000, 10003, "海賊刀", "scimitar"),
        Item::new("226", 10000, 10010, "戦斧", "battle axe"),
        Item::new("227", 10000, 10005, "大槌", "hammer"),
        Item::new("228", 10000, 10007, "三叉槍", "trident"),
        Item::new("229", 10000, 10006, "長棒", "long staff"),
        Item::new("230", 24000, 24001, "短弓", "short bow"),
        Item::new("231", 24000, 24020, "機関銃", "machine gun"),
        Item::new("232", 10000, 10001, "大剣", "claymore"),
        Item::new("234", 10000, 10010, "大斧", "bardish"),
        Item::new("235", 10000, 10008, "鉾槍", "halberd"),
        Item::new("266", 10000, 10003, "忍刀", "wakizashi"),
        Item::new("355", 22000, 22003, "《火炎竜ヴェスダの篭手》", "<Gloves of Vesda>"),
        Item::new("356", 10000, 10004, "《ブラッドムーン》", "<Blood Moon>"),
        Item::new("357", 32000, 32001, "《鋼鉄竜の指輪》", "<Ring of Steel Dragon>"),
        Item::new("358", 10000, 10006, "《狂気の杖》", "<Staff of Insanity>"),
        Item::new("359", 10000, 10007, "《ランキス》", "<Rankis>"),
        Item::new("360", 32000, 32001, "《パルミア・プライド》", "<Palmia Pride>"),
        Item::new("435", 16000, 16001, "厚鎧", "banded mail"),
        Item::new("436", 16000, 16001, "重層鎧", "plate mail"),
        Item::new("437", 16000, 16001, "輪鎧", "ring mail"),
        Item::new("438", 16000, 16001, "合成鎧", "composite mail"),
        Item::new("439", 16000, 16001, "綴り鎧", "chain mail"),
        Item::new("440", 16000, 16003, "法王衣", "pope robe"),
        Item::new("441", 16000, 16003, "軽鎧", "light mail"),
        Item::new("442", 16000, 16003, "防護服", "coat"),
        Item::new("443", 16000, 16003, "胴衣", "breast plate"),
        Item::new("444", 16000, 16003, "防弾服", "bulletproof jacket"),
        Item::new("445", 22000, 22003, "手袋", "gloves"),
        Item::new("446", 22000, 22001, "重層篭手", "plate gauntlets"),
        Item::new("447", 22000, 22003, "軽手袋", "light gloves"),
        Item::new("448", 22000, 22001, "合成篭手", "composite gauntlets"),
        Item::new("449", 14000, 14003, "小盾", "small shield"),
        Item::new("450", 14000, 14003, "丸型盾", "round shield"),
        Item::new("451", 14000, 14003, "盾", "shield"),
        Item::new("452", 14000, 14003, "合成盾", "large shield"),
        Item::new("453", 14000, 14003, "長盾", "kite shield"),
        Item::new("454", 14000, 14003, "重層盾", "tower shield"),
        Item::new("455", 18000, 18002, "履物", "shoes"),
        Item::new("456", 18000, 18002, "靴", "boots"),
        Item::new("457", 18000, 18002, "厚靴", "tight boots"),
        Item::new("458", 18000, 18001, "装甲靴", "armored boots"),
        Item::new("459", 19000, 19001, "合成腰当", "composite girdle"),
        Item::new("460", 19000, 19001, "重層腰当", "plate girdle"),
        Item::new("461", 20000, 20001, "防護外套", "armored cloak"),
        Item::new("462", 20000, 20001, "外套", "cloak"),
        Item::new("463", 12000, 12002, "羽帽子", "feather hat"),
        Item::new("464", 12000, 12002, "重兜", "heavy helm"),
        Item::new("465", 12000, 12001, "騎士兜", "knight helm"),
        Item::new("466", 12000, 12001, "兜", "helm"),
        Item::new("467", 12000, 12001, "合金兜", "composite helm"),
        Item::new("468", 34000, 34001, "ペリドット", "peridot"),
        Item::new("469", 34000, 34001, "護符", "talisman"),
        Item::new("470", 34000, 34001, "首当て", "neck guard"),
        Item::new("471", 34000, 34001, "お守り", "charm"),
        Item::new("472", 34000, 34001, "細工首輪", "bejeweled amulet"),
        Item::new("473", 34000, 34001, "結婚首輪", "engagement amulet"),
        Item::new("474", 32000, 32001, "合金指輪", "composite ring"),
        Item::new("475", 32000, 32001, "指当て", "armored ring"),
        Item::new("476", 32000, 32001, "指輪", "ring"),
        Item::new("477", 32000, 32001, "結婚指輪", "engagement ring"),
        Item::new("482", 24000, 24003, "機械弓", "crossbow"),
        Item::new("483", 25000, 25002, "クロスボウの矢束", "bolt"),
        Item::new("496", 24000, 24020, "散弾銃", "shot gun"),
        Item::new("512", 24000, 24021, "光子銃", "laser gun"),
        Item::new("513", 25000, 25030, "光子弾", "energy cell"),
        Item::new("514", 24000, 24021, "《レールガン》", "<Rail Gun>"),
        Item::new("520", 20000, 20001, "翼", "wing"),
        Item::new("552", 20000, 20001, "羽", "feather"),
        Item::new("556", 18000, 18002, "セブンリーグブーツ", "seven league boots"),
        Item::new("557", 20000, 20001, "ヴィンデールクローク", "vindale cloak"),
        Item::new("558", 32000, 32001, "オーロラリング", "aurora ring"),
        Item::new("627", 12000, 12001, "《賢者の兜》", "<Sage's Helm>"),
        Item::new("633", 24000, 24030, "ギャルのパンティー", "panty"),
        Item::new("661", 18000, 18002, "《ダル=イ=サリオン》", "<Dal-i-thalion>"),
        Item::new("664", 32000, 32001, "スピードの指輪", "speed ring"),
        Item::new("673", 24000, 24001, "《ウィンドボウ》", "<Wind Bow>"),
        Item::new("674", 24000, 24020, "《ウィンチェスター・プレミアム》", "<Winchester Premium>"),
        Item::new("675", 10000, 10011, "《クミロミサイズ》", "<Kumiromi Sythe>"),
        Item::new("676", 10000, 10006, "《エレメンタルスタッフ》", "<Elemental Staff)"),
        Item::new("677", 10000, 10007, "《ホーリーランス》", "<Holy Rance>"),
        Item::new("678", 10000, 10003, "《ラッキーダガー》", "<Lucky Dagger>"),
        Item::new("679", 10000, 10005, "《大地の大槌》", "Gaia Hammer>"),
        Item::new("695", 10000, 10010, "《破壊の斧》", "<Axe of Destruction>"),
        Item::new("705", 34000, 34001, "《乞食のペンダント》", "<Begger's Pendant>"),
        Item::new("713", 24000, 24030, "手裏剣", "shuriken"),
        Item::new("714", 24000, 24030, "手榴弾", "grenade"),
        Item::new("716", 24000, 24030, "《バニラロック》", "<Vanilla Rock>"),
        Item::new("718", 24000, 24030, "《シーナのパンティー》", "<Shena's Panty>"),
        Item::new("719", 10000, 10001, "《クレイモア》", "<Claymore>"),
        Item::new("722", 34000, 34001, "《アルバレスト》", "<Arbalest>"),
        Item::new("723", 34000, 34001, "《ツインエッジ》", "<Twin Edge>"),
        Item::new("725", 24000, 24030, "《キルキルピアノ》", "<Kill Kill Piano>"),
        Item::new("726", 14000, 14003, "《アル・ウード》", "<Al'ud>"),
        Item::new("727", 14000, 14003, "《棘の盾》", "<Shield ofThorn>"),
        Item::new("728", 19000, 19001, "《紅凛》", "<Crimson Plate>"),
        Item::new("735", 10000, 10011, "大鎌", "scythe"),
        Item::new("739", 10000, 10011, "《フリージアの尻尾》", "<Frisia's Tail>"),
        Item::new("740", 34000, 34001, "《謎の貝》", "<Unknown Shell>"),
        Item::new("741", 10000, 10002, "《飛竜刀》", "<Hiryu-To>"),
        Item::new("757", 12000, 12001, "《五本角の兜》", "<Five Horned Helm>"),
        Item::new("758", 24000, 24020, "《マウザーC96カスタム》", "<Mauser C96 Custom>"),
        Item::new("759", 10000, 10002, "ライトセーバー", "lightsabre"),
        Item::new("781", 10000, 10003, "包丁", "kitchen knife"),
        Item::new("788", 24000, 24001, "骨弓", "skull bow"),
    ];
    Ok(ItemCollection { list: items })
}

#[tauri::command]
async fn get_texts() -> Result<TextCollection, String> {
    let texts: Vec<Text> = vec![
        Text::new("%txtCalm", "通常の待機状態"),
        Text::new("%txtAggro", "交戦する"),
        Text::new("%txtDead", "ミンチにされた"),
        Text::new("%txtKilled", "ミンチにした"),
        Text::new("%txtWelcome", "出迎え"),
        Text::new("%txtDialog", "話しかけた"),
        Text::new("%txtabuse", "罵倒する"),
        Text::new("%txtmarriage", "婚約を申し込まれた"),
        Text::new("%txtanata", "遺伝子を残す"),
        Text::new("%txtiyayo", "遺伝子を残すのを断る"),
        Text::new("%txtnakanaka", "気持ちいいことを受ける時（デフォだと「なかなかの身体つきだ、買った」みたいなやつ）"),
        Text::new("%txtikuyo", "気持ちいいことを受ける時2（デフォだと上の直後の「いくよ！」みたいなやつ）"),
        Text::new("%txtkiyameru", "気持ちいいことをするで「やめる」を選んだ時（デフォだと「冷やかしか？」みたいなやつ）"),
        Text::new("%txtkuyasii", "気持ちいいことをしている最中"),
        Text::new("%txtjigo", "気持ちいいことの事後"),
        Text::new("%txtnoru", "乗馬される"),
        Text::new("%txtoriru", "乗馬状態を解除される"),
        Text::new("%txtbiyaku", "媚薬入りの食べ物を食べた"),
        Text::new("%txttiti", "媚薬で乳や卵を産む"),
        Text::new("%txtsaite", "媚薬を渡されて叩き割る"),
        Text::new("%txtsand", "サンドバッグに吊るされている"),
        Text::new("%txtnikorose", "気が狂っている"),
        Text::new("%txtkya", "聴診器を当てた"),
        Text::new("%txttyohazusu", "聴診器を外した"),
        Text::new("%txtsibaru", "紐で縛った"),
        Text::new("%txthodoku", "紐をほどいた"),
        Text::new("%txtturusu", "サンドバッグに吊るされる"),
        Text::new("%txtsorosu", "サンドバッグから降ろされる"),
        Text::new("%txtsnaguru", "サンドバッグに吊るされて殴られている"),
        Text::new("%txtomiyage", "おみやげを渡された"),
        Text::new("%txtyubikubi", "結婚指輪、首輪を渡された時（「顔を赤らめた」のあと）"),
        Text::new("%txttoriage", "結婚指輪、首輪を取り上げようとした時（「飲み込んだ」のあと）"),
        Text::new("%txtpbou", "ペットを冒険者にしたあと話しかけた"),
        Text::new("%txtexthank", "ペットを冒険者にしたあとアイテム交換をした"),
        Text::new("%txtexhiya", "ペットを冒険者にしたあとアイテム交換を持ちかけて、見合うアイテムを持っていないor交換をやめた"),
        Text::new("%txtgoei", "ペットを冒険者にしたあと護衛の依頼を持ちかけた"),
        Text::new("%txtyatou", "ペットを冒険者にしたあと護衛を依頼した"),
        Text::new("%txthihiya", "ペットを冒険者にしたあと護衛の依頼を持ちかけてやめた"),
        Text::new("%txtumaku", "ペットを冒険者にしたあと仲間に誘って承諾された"),
        Text::new("%txttikara", "ペットを冒険者にしたあと仲間に誘ってお断りされた時（力の差がありすぎる場合）"),
        Text::new("%txt0free", "ペットを冒険者にしたあと仲間に誘ってお断りされた時（ペット枠が空いていない場合）"),
        Text::new("%txtokoto", "ペットを冒険者にしたあと仲間に誘ってお断りされた時（友好度が足りないor雇用回数が足りない場合）"),
        Text::new("%txtallykilled{0}", "仲間の{0}がミンチにされた時"),
        Text::new("%txtallykilleddefault", "仲間がミンチにされた時"),
        Text::new("%txtsibui", "腐った物を食べた"),
        Text::new("%txtnamaniku", "生肉を食べた"),
        Text::new("%txtkona", "生の小麦粉を食べた"),
        Text::new("%txtnamamen", "生麺を食べた"),
        Text::new("%txtheibon", "その他の未調理の物を食べた"),
        Text::new("%txt1_2", "ランク1～2の料理（失敗料理）を食べた"),
        Text::new("%txt3_4", "ランク3～4の料理を食べた"),
        Text::new("%txt5_6", "ランク5～6の料理を食べた"),
        Text::new("%txt7_8", "ランク7～8の料理を食べた"),
        Text::new("%txt9saiko", "ランク9（最高級）の料理を食べた"),
        Text::new("%txtkaradake", "気持ちいいことが中断された"),
        Text::new("%txtyanwari", "婚約を断る"),
        Text::new("%txtkunren", "訓練所に行こうとして訓練費用がなかった"),
        Text::new("%txtonaka", "自動食事をしようとして手元に食べ物がなかった"),
        Text::new("%txthinsi", "瀕死"),
        Text::new("%txtkodukuri", "子供を作る"),
        Text::new("%txtlayhand", "レイハンドを使う"),
        Text::new("%txtakita", "演奏を聞かされて飽きた"),
        Text::new("%txturusai", "演奏を聞かされて投石する"),
        Text::new("%txtthrowrock", "演奏を聞かされた際にデフォルトだと「○○は石を投げた」となる部分"),
        Text::new("%txtbravo", "演奏を聞かされて褒める"),
        Text::new("%txtbatou", "デフォルトだと「○○は××を罵倒した」となる部分"),
        Text::new("%txtparasite", "寄生された"),
        Text::new("%txtumare", "寄生されている時（デフォルトだと「なにかが産まれそうだよ！」の部分）"),
        Text::new("%txttobidasi", "何かが腹を破って飛び出した"),
        Text::new("%txttoketa", "エイリアンを溶かした"),
        Text::new("%txtsing", "PCの演奏に合わせて歌っているとき"),
        Text::new("%txtcast{0}", "{0}の魔法を唱えたとき"),
        Text::new("%txtpornobook{0}", "PCが{0}のエロ本を読んでいるとき"),
        Text::new("%txtpornobookdefault", "PCがエロ本を読んでいるとき（キャラ番号の指定なしの場合）"),
        Text::new("%txtactbefore{0}", "{0}の技能を使う前の台詞"),
        Text::new("%txtactafter{0}", "{0}の技能を使った後の台詞"),
        Text::new("%txtuzimushi", "狂気の眼差しのメッセージ"),
        Text::new("%txtcaststyle", "魔法を使用した時のメッセージ"),
        Text::new("%txtcaststyle2", "自己回復、自己強化系の魔法を使用した時のメッセージ"),
        Text::new("%txtswarm", "スウォームを使用した時のメッセージ"),
        Text::new("%txtkisei", "相手に寄生した時のメッセージ"),
        Text::new("%txtmilk", "ミルクを飲んだ時"),
        Text::new("%txtmilkcurse", "呪われたミルクを飲んだ時"),
        Text::new("%txtsake", "酒を飲んだ時"),
        Text::new("%txtsakecurse", "呪われた酒を飲んだ時"),
        Text::new("%txtyopparai", "酔い状態の時"),
    ];
    Ok(TextCollection { list: texts })
}

#[tauri::command]
async fn get_case_groups() -> Result<CaseGroupCollection, String> {
    let case_groups: Vec<CaseGroup> = vec![
        CaseGroup::new("", "条件なし"),
        CaseGroup::new("$when", "時間に関係する条件式"),
        CaseGroup::new("$where", "現在地に関係する条件式"),
        CaseGroup::new("$weather", "天気に関係する条件式"),
        CaseGroup::new("$impression", "好感度に関する条件式"),
        CaseGroup::new("$condition", "状態異常に関係する条件式"),
        CaseGroup::new("$PCcondition", "プレイヤーキャラクターの状態異常に関係する条件式"),
        CaseGroup::new("$karma", "カルマに関係する条件式"),
        CaseGroup::new("$cash", "所持金に関係する条件式"),
        CaseGroup::new("$PCcash", "プレイヤーキャラクターの所持金に関係する条件式"),
        CaseGroup::new("$fame", "名声に関係する条件式"),
        CaseGroup::new("$PCfame", "プレイヤーキャラクターの名声に関係する条件式"),
        CaseGroup::new("$religion", "信仰に関係する条件式"),
        CaseGroup::new("$PCreligion", "プレイヤーキャラクターの信仰に関係する条件式"),
        CaseGroup::new("$action", "複数ターン使用する行動に関係する条件式"),
        CaseGroup::new("$PCaction", "プレイヤーキャラクターの複数ターン使用する行動に関係する条件式"),
        CaseGroup::new("$sex", "性別に関係する条件式"),
        CaseGroup::new("$PCsex", "プレイヤーキャラクターの性別に関係する条件式"),
        CaseGroup::new("$race", "種族に関係する条件式"),
        CaseGroup::new("$PCrace", "プレイヤーキャラクターの種族に関係する条件式"),
        CaseGroup::new("$class", "職業に関係する条件式"),
        CaseGroup::new("$PCclass", "プレイヤーキャラクターの職業に関係する条件式"),
        CaseGroup::new("$comparison", "要素の数値の比較に関係する条件式 "),
        CaseGroup::new("$pet", "発言者がプレイヤーのペットのとき"),
        CaseGroup::new("$random", "X%の確率を満たしたとき"),
        CaseGroup::new("$married", "結婚しているとき"),
        CaseGroup::new("$stethoscope", "聴診器を使われているとき"),
        CaseGroup::new("$tied", "紐で結ばれているとき"),
        CaseGroup::new("$ridden", "騎乗されているとき"),
        CaseGroup::new("$agreement", "冒険者として契約中のとき"),
        CaseGroup::new("$anorexia", "拒食症のとき"),
        CaseGroup::new("$layhand", "レイハンドが使えるとき"),
        CaseGroup::new("$incognito", "変装しているとき"),
    ];
    Ok(CaseGroupCollection { list: case_groups })
}

#[tauri::command]
async fn get_cases() -> Result<CaseCollection, String> {
    let cases: Vec<Case> = vec![
        Case::new("", "", 0, "none", "条件なし"),
        Case::new("$when", "$when year {0}", 1, "year", "{0}年のとき"),
        Case::new("$when", "$when year {0} - {1}", 2, "year", "{0}年から{1}年のとき"),
        Case::new("$when", "$when year - {0}", 1, "year", "{0}年以下のとき"),
        Case::new("$when", "$when year {0} -", 1, "year", "{0}年以上のとき"),
        Case::new("$when", "$when month {0}", 1, "month", "{0}月のとき"),
        Case::new("$when", "$when month {0} - {1}", 2, "month", "{0}月から{1}月のとき"),
        Case::new("$when", "$when date {0}", 1, "date", "{0}日のとき"),
        Case::new("$when", "$when date {0} - {1}", 2, "date", "{0}日から{1}日のとき"),
        Case::new("$when", "$when month {0} - {1}", 2, "month", "{0}月から{1}月のとき"),
        Case::new("$when", "$when hour {0}", 1, "hour", "{0}時のとき"),
        Case::new("$when", "$when hour {0} - {1}", 2, "hour", "{0}時から{1}時のとき"),
        Case::new("$when", "$when time {0}:{1}", 2, "time", "{0}時{1}分のとき"),
        Case::new("$when", "$when time {0}:{1} - {2}:{3}", 4, "time", "{0}時{1}分から{2}時{3}分のとき"),
        Case::new("$when", "$when Midnight", 0, "none", "深夜のとき"),
        Case::new("$when", "$when Dawn", 0, "none", "夜明けのとき"),
        Case::new("$when", "$when Morning", 0, "none", "朝のとき"),
        Case::new("$when", "$when Noon", 0, "none", "昼のとき"),
        Case::new("$when", "$when Dusk", 0, "none", "宵のとき"),
        Case::new("$when", "$when Night", 0, "none", "夜のとき"),
        Case::new("$where", "$where North_Tyris", 0, "none", "グローバルマップにいるとき"),
        Case::new("$where", "$where Home", 0, "none", "家にいるとき"),
        Case::new("$where", "$where Party", 0, "none", "演奏依頼でパーティ会場にいるとき"),
        Case::new("$where", "$where Outskirts", 0, "none", "退治依頼で街近辺にいるとき"),
        Case::new("$where", "$where Urban_Area", 0, "none", "討伐依頼で市街地にいるとき"),
        Case::new("$where", "$where Crop", 0, "none", "収穫依頼で街周辺の畑にいるとき"),
        Case::new("$where", "$where Town", 0, "none", "主要な町（ヴェルニース、ポート・カプール、ヨウィン、ダルフィ、パルミア、ルミエスト、ノイエル、ラーナ）のどこかにいるとき"),
        Case::new("$where", "$where Vernis", 0, "none", "ヴェルニースにいるとき"),
        Case::new("$where", "$where Port_Kapul", 0, "none", "ポート・カプールにいるとき"),
        Case::new("$where", "$where Yowyn", 0, "none", "ヨウィンにいるとき"),
        Case::new("$where", "$where Derphy", 0, "none", "ダルフィにいるとき"),
        Case::new("$where", "$where Palmia", 0, "none", "パルミアにいるとき"),
        Case::new("$where", "$where Lumiest", 0, "none", "ルミエストにいるとき"),
        Case::new("$where", "$where Noyel", 0, "none", "ノイエルにいるとき"),
        Case::new("$where", "$where Larna", 0, "none", "ラーナにいるとき"),
        Case::new("$where", "$where Cyber_Dome", 0, "none", "アクリ・テオラにいるとき"),
        Case::new("$where", "$where Pet_Arena", 0, "none", "コロシアムにいるとき"),
        Case::new("$where", "$where Truce_Ground", 0, "none", "神々の休戦地にいるとき"),
        Case::new("$where", "$where Graveyard", 0, "none", "ルミエスト墓所にいるとき"),
        Case::new("$where", "$where Embassy", 0, "none", "パルミア大使館にいるとき"),
        Case::new("$where", "$where Museum", 0, "none", "博物館にいるとき"),
        Case::new("$where", "$where Shop", 0, "none", "自分の店にいるとき"),
        Case::new("$where", "$where Storage_House", 0, "none", "倉庫にいるとき"),
        Case::new("$where", "$where Ranch", 0, "none", "牧場にいるとき"),
        Case::new("$where", "$where Shelter", 0, "none", "シェルターにいるとき"),
        Case::new("$where", "$where Sister", 0, "none", "妹の館にいるとき"),
        Case::new("$where", "$where Pyramid", 0, "none", "ピラミッドにいるとき"),
        Case::new("$where", "$where Jail", 0, "none", "牢獄にいるとき"),
        Case::new("$where", "$where Mountain_Pass", 0, "none", "山道にいるとき"),
        Case::new("$where", "$where Wilderness", 0, "none", "野外にいるとき"),
        Case::new("$where", "$where Show_House", 0, "none", "ハウスドームにいるとき"),
        Case::new("$where", "$where Lesimas", 0, "none", "レシマスにいるとき"),
        Case::new("$where", "$where Void", 0, "none", "すくつにいるとき"),
        Case::new("$where", "$where Nefia", 0, "none", "ネフィアにいるとき"),
        Case::new("$where", "$where floor {0}", 1, "floor", "{0}階にいるとき"),
        Case::new("$where", "$where floor {0} -", 1, "floor", "{0}階より深い階層にいるとき"),
        Case::new("$where", "$where floor - {0}", 1, "floor", "{0}階より浅い階層にいるとき"),
        Case::new("$where", "$where floor {0} - {1}", 2, "floor", "{0}階より深く{1}階より浅い階層にいるとき"),
        Case::new("$weather", "$weather Sun", 0, "none", "晴れのとき"),
        Case::new("$weather", "$weather Rain", 0, "none", "雨が降っているとき"),
        Case::new("$weather", "$weather Snow", 0, "none", "雪が降っているとき"),
        Case::new("$weather", "$weather Hard_rain", 0, "none", "雷雨が降っているとき"),
        Case::new("$weather", "$weather Etherwind", 0, "none", "エーテル風が吹いているとき"),
        Case::new("$impression", "$impression Foe", 0, "none", "関係が天敵のとき"),
        Case::new("$impression", "$impression Hate", 0, "none", "関係が嫌いのとき"),
        Case::new("$impression", "$impression Annoying", 0, "none", "関係がうざいのとき"),
        Case::new("$impression", "$impression Normal", 0, "none", "関係が普通のとき"),
        Case::new("$impression", "$impression Amiable", 0, "none", "関係が好意的のとき"),
        Case::new("$impression", "$impression Friend", 0, "none", "関係が友達のとき"),
        Case::new("$impression", "$impression Fellow", 0, "none", "関係が親友のとき"),
        Case::new("$impression", "$impression Soul_Mate", 0, "none", "関係が魂の友のとき"),
        Case::new("$impression", "$impression Love", 0, "none", "関係が*Love*のとき"),
        Case::new("$impression", "$impression {0}", 1, "impression", "好感度が{0}のとき"),
        Case::new("$impression", "$impression {0} - {1}", 2, "impression", "好感度が{0}以上で{1}以下の時"),
        Case::new("$impression", "$impression {0} -", 1, "impression", "好感度が{0}以上の時"),
        Case::new("$impression", "$impression - {0}", 1, "impression", "好感度が{0}以下の時"),
        Case::new("$condition", "$condition Poisoned", 0, "none", "発言者が毒、または猛毒の状態異常になっているとき"),
        Case::new("$condition", "$condition Sleep", 0, "none", "発言者が睡眠、または爆睡の状態異常になっているとき"),
        Case::new("$condition", "$condition Paralyzed", 0, "none", "発言者が麻痺の状態異常になっているとき"),
        Case::new("$condition", "$condition Blinded", 0, "none", "発言者が盲目の状態異常になっているとき"),
        Case::new("$condition", "$condition Confused", 0, "none", "発言者が混乱の状態異常になっているとき"),
        Case::new("$condition", "$condition Fear", 0, "none", "発言者が恐怖の状態異常になっているとき"),
        Case::new("$condition", "$condition Dim", 0, "none", "発言者が朦朧、混濁、または気絶の状態異常になっているとき"),
        Case::new("$condition", "$condition Drunk", 0, "none", "発言者が酔払いの状態異常になっているとき"),
        Case::new("$condition", "$condition Bleeding", 0, "none", "発言者が切り傷、出血、または大出血の状態異常になっているとき"),
        Case::new("$condition", "$condition Wet", 0, "none", "発言者が濡れの状態異常になっているとき"),
        Case::new("$condition", "$condition Insane", 0, "none", "発言者が不安定、狂気、または崩壊の状態異常になっているとき"),
        Case::new("$condition", "$condition Sick", 0, "none", "発言者が病気または重病の状態異常になっているとき"),
        Case::new("$condition", "$condition Fury", 0, "none", "発言者が激怒または狂乱の状態異常になっているとき"),
        Case::new("$PCcondition", "$PCcondition Normal", 0, "none", "プレイヤーキャラクターが状態異常になっていないとき"),
        Case::new("$PCcondition", "$PCcondition sleepiness {0}", 1, "sleepiness", "プレイヤーキャラクターの眠気が{0}のとき"),
        Case::new("$PCcondition", "$PCcondition sleepiness {0} -", 1, "sleepiness", "プレイヤーキャラクターの眠気が{0}以上のとき"),
        Case::new("$PCcondition", "$PCcondition sleepiness - {0}", 1, "sleepiness", "プレイヤーキャラクターの眠気が{0}以下のとき"),
        Case::new("$PCcondition", "$PCcondition sleepiness {0} - {1}", 2, "sleepiness", "プレイヤーキャラクターの眠気が{0}以上{1}以下のとき"),
        Case::new("$PCcondition", "$PCcondition Poisoned", 0, "none", "プレイヤーキャラクターが毒、または猛毒の状態異常になっているとき"),
        Case::new("$PCcondition", "$PCcondition Sleep", 0, "none", "プレイヤーキャラクターが睡眠、または爆睡の状態異常になっているとき"),
        Case::new("$PCcondition", "$PCcondition Paralyzed", 0, "none", "プレイヤーキャラクターが麻痺の状態異常になっているとき"),
        Case::new("$PCcondition", "$PCcondition Blinded", 0, "none", "プレイヤーキャラクターが盲目の状態異常になっているとき"),
        Case::new("$PCcondition", "$PCcondition Confused", 0, "none", "プレイヤーキャラクターが混乱の状態異常になっているとき"),
        Case::new("$PCcondition", "$PCcondition Fear", 0, "none", "プレイヤーキャラクターが恐怖の状態異常になっているとき"),
        Case::new("$PCcondition", "$PCcondition Dim", 0, "none", "プレイヤーキャラクターが朦朧、混濁、または気絶の状態異常になっているとき"),
        Case::new("$PCcondition", "$PCcondition Drunk", 0, "none", "プレイヤーキャラクターが酔払いの状態異常になっているとき"),
        Case::new("$PCcondition", "$PCcondition Bleeding", 0, "none", "プレイヤーキャラクターが切り傷、出血、または大出血の状態異常になっているとき"),
        Case::new("$PCcondition", "$PCcondition Wet", 0, "none", "プレイヤーキャラクターが濡れの状態異常になっているとき"),
        Case::new("$PCcondition", "$PCcondition Insane", 0, "none", "プレイヤーキャラクターが不安定、狂気、または崩壊の状態異常になっているとき"),
        Case::new("$PCcondition", "$PCcondition Sick", 0, "none", "プレイヤーキャラクターが病気または重病の状態異常になっているとき"),
        Case::new("$PCcondition", "$PCcondition Fury", 0, "none", "プレイヤーキャラクターが激怒または狂乱の状態異常になっているとき"),
        Case::new("$karma", "$karma {0}", 1, "karma", "カルマが{0}のとき"),
        Case::new("$karma", "$karma {0} -", 1, "karma", "カルマが{0}以上のとき"),
        Case::new("$karma", "$karma - {0}", 1, "karma", "カルマが{0}以下のとき"),
        Case::new("$karma", "$karma {0} - {1}", 2, "karma", "カルマが{0}以上{1}以下のとき"),
        Case::new("$cash", "$cash {0}", 1, "cash", "発言者の所持金が{0}のとき"),
        Case::new("$cash", "$cash {0} -", 1, "cash", "発言者の所持金が{0}以上のとき"),
        Case::new("$cash", "$cash - {0}", 1, "cash", "発言者の所持金が{0}以下のとき"),
        Case::new("$cash", "$cash {0} - {1}", 2, "cash", "発言者の所持金が{0}以上{1}以下のとき"),
        Case::new("$PCcash", "$PCcash {0}", 1, "cash", "プレイヤーキャラクターの所持金が{0}のとき"),
        Case::new("$PCcash", "$PCcash {0} -", 1, "cash", "プレイヤーキャラクターの所持金が{0}以上のとき"),
        Case::new("$PCcash", "$PCcash - {0}", 1, "cash", "プレイヤーキャラクターの所持金が{0}以下のとき"),
        Case::new("$PCcash", "$PCcash {0} - {1}", 2, "cash", "プレイヤーキャラクターの所持金が{0}以上{1}以下のとき"),
        Case::new("$fame", "$fame {0}", 1, "fame", "発言者の名声が{0}のとき"),
        Case::new("$fame", "$fame {0} -", 1, "fame", "発言者の名声が{0}以上のとき"),
        Case::new("$fame", "$fame - {0}", 1, "fame", "発言者の名声が{0}以下のとき"),
        Case::new("$fame", "$fame {0} - {1}", 2, "fame", "発言者の名声が{0}以上{1}以下のとき"),
        Case::new("$PCfame", "$PCfame {0}", 1, "fame", "プレイヤーキャラクターの名声が{0}のとき"),
        Case::new("$PCfame", "$PCfame {0} -", 1, "fame", "プレイヤーキャラクターの名声が{0}以上のとき"),
        Case::new("$PCfame", "$PCfame - {0}", 1, "fame", "プレイヤーキャラクターの名声が{0}以下のとき"),
        Case::new("$PCfame", "$PCfame {0} - {1}", 2, "fame", "プレイヤーキャラクターの名声が{0}以上{1}以下のとき"),
        Case::new("$religion", "$religion same", 0, "none", "発言者とプレイヤーキャラクターが同じ神を信仰しているとき"),
        Case::new("$religion", "$religion Eyth", 0, "none", "発言者が無のエイスを信仰しているとき"),
        Case::new("$religion", "$religion Mani", 0, "none", "発言者が機械のマニを信仰しているとき"),
        Case::new("$religion", "$religion Lulwy", 0, "none", "発言者が風のルルウィを信仰しているとき"),
        Case::new("$religion", "$religion Itzpalt", 0, "none", "発言者が元素のイツパトロルを信仰しているとき"),
        Case::new("$religion", "$religion Ehekatl", 0, "none", "発言者が幸運のエヘカトルを信仰しているとき"),
        Case::new("$religion", "$religion Opatos", 0, "none", "発言者が地のオパートスを信仰しているとき"),
        Case::new("$religion", "$religion Jure", 0, "none", "発言者が癒しのジュアを信仰しているとき"),
        Case::new("$religion", "$religion Kumiromi", 0, "none", "発言者が収穫のクミロミを信仰しているとき"),
        Case::new("$PCreligion", "$PCreligion Eyth", 0, "none", "プレイヤーキャラクターが無のエイスを信仰しているとき"),
        Case::new("$PCreligion", "$PCreligion Mani", 0, "none", "プレイヤーキャラクターが機械のマニを信仰しているとき"),
        Case::new("$PCreligion", "$PCreligion Lulwy", 0, "none", "プレイヤーキャラクターが風のルルウィを信仰しているとき"),
        Case::new("$PCreligion", "$PCreligion Itzpalt", 0, "none", "プレイヤーキャラクターが元素のイツパトロルを信仰しているとき"),
        Case::new("$PCreligion", "$PCreligion Ehekatl", 0, "none", "プレイヤーキャラクターが幸運のエヘカトルを信仰しているとき"),
        Case::new("$PCreligion", "$PCreligion Opatos", 0, "none", "プレイヤーキャラクターが地のオパートスを信仰しているとき"),
        Case::new("$PCreligion", "$PCreligion Jure", 0, "none", "プレイヤーキャラクターが癒しのジュアを信仰しているとき"),
        Case::new("$PCreligion", "$PCreligion Kumiromi", 0, "none", "プレイヤーキャラクターが収穫のクミロミを信仰しているとき"),
        Case::new("$action", "$action Performance", 0, "none", "発言者が演奏をしているとき"),
        Case::new("$action", "$action Eat", 0, "none", "発言者が食事をしているとき"),
        Case::new("$PCaction", "$PCaction Performance", 0, "none", "プレイヤーキャラクターが演奏をしているとき"),
        Case::new("$PCaction", "$PCaction Dig", 0, "none", "プレイヤーキャラクターが壁を掘っているとき"),
        Case::new("$PCaction", "$PCaction Reading", 0, "none", "プレイヤーキャラクターが読書をしているとき"),
        Case::new("$PCaction", "$PCaction Fishing", 0, "none", "プレイヤーキャラクターが釣りをしているとき"),
        Case::new("$PCaction", "$PCaction Harvesting", 0, "none", "プレイヤーキャラクターが採取をしているとき"),
        Case::new("$PCaction", "$PCaction Search", 0, "none", "プレイヤーキャラクターが探索をしているとき"),
        Case::new("$PCaction", "$PCaction Eat", 0, "none", "プレイヤーキャラクターが食事をしているとき"),
        Case::new("$sex", "$sex same", 0, "none", "発言者とプレイヤーキャラクターの性別が同じとき"),
        Case::new("$sex", "$sex Male", 0, "none", "発言者が男性のとき"),
        Case::new("$sex", "$sex Female", 0, "none", "発言者が女性のとき"),
        Case::new("$PCsex", "$PCsex Male", 0, "none", "プレイヤーキャラクターが男性のとき"),
        Case::new("$PCsex", "$PCsex Female", 0, "none", "プレイヤーキャラクターが女性のとき"),
        Case::new("$race", "$race same", 0, "none", "発言者とプレイヤーキャラクターの種族が同じとき"),
        Case::new("$race", "$race {0}", 1, "race", "発言者の種族が{0}のとき"),
        Case::new("$PCrace", "$PCrace {0}", 1, "race", "プレイヤーキャラクターの種族が{0}のとき"),
        Case::new("$class", "$class same", 0, "none", "発言者とプレイヤーキャラクターの職業が同じとき"),
        Case::new("$class", "$class {0}", 1, "class", "発言者の職業が{0}のとき"),
        Case::new("$PCclass", "$PCclass {0}", 1, "class", "プレイヤーキャラクターの職業が{0}のとき"),
        Case::new("$comparison", "$comparison {0}", 1, "comparison", "{0}のとき"),
        Case::new("$pet", "$pet", 0, "none", "発言者がプレイヤーのペットのとき"),
        Case::new("$random", "$random {0}", 1, "percent", "{0}%の確率を満たしたとき"),
        Case::new("$married", "$married", 0, "none", "結婚しているとき"),
        Case::new("$stethoscope", "$stethoscope", 0, "none", "聴診器を使われているとき"),
        Case::new("$tied", "$tied", 0, "none", "紐で結ばれているとき"),
        Case::new("$ridden", "$ridden", 0, "none", "騎乗されているとき"),
        Case::new("$agreement", "$agreement", 0, "none", "冒険者として契約中のとき"),
        Case::new("$anorexia", "$anorexia", 0, "none", "拒食症のとき"),
        Case::new("$layhand", "$layhand", 0, "none", "レイハンドが使えるとき"),
        Case::new("$incognito", "$incognito", 0, "none", "変装しているとき"),
    ];
    Ok(CaseCollection { list: cases })
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
    let (res, _, _) = encoding_rs::SHIFT_JIS.decode(&s);
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
    let (encoded, _, _) =  encoding_rs::SHIFT_JIS.encode(&string);
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
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
