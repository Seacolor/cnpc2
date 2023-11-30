use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref REGEX_TALK_ORDER: Regex = Regex::new(r"(?mR)^%txt_talk_order$").unwrap();
    static ref REGEX_TALK: Regex = Regex::new("(?sR)%txt_ucnpc_ev_b\r\n(.*?)\r\n%txt_ucnpc_ev_e").unwrap();
    static ref REGEX_CASE: Regex = Regex::new(r"(?m)^%?(\$.*?) (.*?)$").unwrap();
    static ref REGEX_COMMENT: Regex = Regex::new(r"(?m)^//.*?").unwrap();
    static ref REGEX_TAG_EN: Regex = Regex::new(r"(?im)^%.*?,en").unwrap();
    static ref REGEX_TAG_JP: Regex = Regex::new(r"(?im)^(%.*?,jp)").unwrap();
    static ref REGEX_EXPRESSION: Regex = Regex::new(r"(?m)^%?\$.*?$").unwrap();
    static ref REGEX_IMPRESSION_NUMBER: Regex = Regex::new(r"(?m)^(\d+)$").unwrap();
}

use crate::{Character, UserResist, UserRace, UserSkill, UserTrait, Figure, UserClass, InitEquip, UserText, UserTextBody, UserTalk};
struct Equip {
    id: String,
    custom_item_id: String,
}

struct TextCase {
    case_value: String,
    values: Vec<String>,
}

struct TextId {
    id: String,
    value: String,
}

struct Talk {
    jp: String,
    parsed_text: String,
}

fn parse_talk(text: &str) -> Option<Talk> {
    if let Some(mat) = REGEX_TALK.find(text) {
        let parsed_text = REGEX_TALK.replace(text, "").to_string();
        return Some(
            Talk {
                jp: mat.as_str().to_string(),
                parsed_text: parsed_text,
            }
        );
    }
    return None;
}

fn parse_case(text: &str) -> TextCase {
    let result1 = REGEX_CASE.captures(text);
    if result1.is_some() {
        let mut case_value = "".into();
        let mut values: Vec<String> = Vec::new();
        let caps1 = result1.unwrap();
        match &caps1[1] {
            "$when" => {
                let vv = caps1[2]
                    .split(" ")
                    .collect::<Vec<_>>();
                match vv[0] {
                    "time" => {
                        match vv.len() {
                            2 => {
                                let vvv = vv[1]
                                    .split(":")
                                    .collect::<Vec<_>>();
                                case_value = format!("{} {} {}:{}", &caps1[1], vv[0], "{0}", "{1}");
                                values.push(vvv[0].to_string());
                                values.push(vvv[1].to_string());
                            }
                            4 => {
                                let vvv1 = vv[1]
                                    .split(":")
                                    .collect::<Vec<_>>();
                                let vvv2 = vv[3]
                                    .split(":")
                                    .collect::<Vec<_>>();
                                case_value = format!("{} {} {}:{} {} {}:{}", &caps1[1], vv[0], "{0}", "{1}", vv[2], "{2}", "{3}");
                                values.push(vvv1[0].to_string());
                                values.push(vvv1[1].to_string());
                                values.push(vvv2[0].to_string());
                                values.push(vvv2[1].to_string());
                            }
                            _ => {
                                case_value = format!("{} {}", &caps1[1], vv[0]);
                            }
                        }
                    }
                    _ => {
                        match vv.len() {
                            2 => {
                                case_value = format!("{} {} {}", &caps1[1], vv[0], "{0}");
                                values.push(vv[1].to_string());
                            }
                            3 => {
                                if vv[1] == "-" {
                                    case_value = format!("{} {} {} {}", &caps1[1], vv[0], vv[1], "{0}");
                                    values.push(vv[2].to_string());
                                } else {
                                    case_value = format!("{} {} {} {}", &caps1[1], vv[0], "{0}", vv[2]);
                                    values.push(vv[1].to_string());
                                }
                            }
                            4 => {
                                case_value = format!("{} {} {} {} {}", &caps1[1], vv[0], "{0}", vv[2], "{1}");
                                values.push(vv[1].to_string());
                                values.push(vv[3].to_string());
                            }
                            5 => {
                                case_value = format!("{} {} {} {} {}", &caps1[1], vv[0], "{0}", vv[2], "{1}");
                                values.push(vv[1].to_string());
                                values.push(vv[3].to_string());
                            }
                            _ => {
                                case_value = format!("{} {}", &caps1[1], vv[0]);
                            }
                        }
                    }
                }
            }
            "$where" => {
                let vv = caps1[2]
                    .split(" ")
                    .collect::<Vec<_>>();
                match vv[0] {
                    "floor" => {
                        match vv.len() {
                            2 => {
                                case_value = format!("{} {} {}", &caps1[1], vv[0], "{0}");
                                values.push(vv[1].to_string());
                            }
                            3 => {
                                if vv[1] == "-" {
                                    case_value = format!("{} {} {} {}", &caps1[1], vv[0], vv[1], "{0}");
                                    values.push(vv[2].to_string());
                                } else {
                                    case_value = format!("{} {} {} {}", &caps1[1], vv[0], "{0}", vv[2]);
                                    values.push(vv[1].to_string());
                                }
                            }
                            4 => {
                                case_value = format!("{} {} {} {} {}", &caps1[1], vv[0], "{0}", vv[2], "{1}");
                                values.push(vv[1].to_string());
                                values.push(vv[3].to_string());
                            }
                            _ => {
                                case_value = format!("{} {}", &caps1[1], vv[0]);
                            }
                        }
                    }
                    _ => {
                        case_value = format!("{} {}", &caps1[1], vv[0]);
                    }
                }
            }
            "$weather" => {
                case_value = format!("{} {}", &caps1[1], &caps1[2]);
            }
            "$impression" => {
                let vv = caps1[2]
                    .split(" ")
                    .collect::<Vec<_>>();
                match vv.len() {
                    2 => {
                        if vv[0] == "-" {
                            case_value = format!("{} {} {}", &caps1[1], vv[0], "{0}");
                            values.push(vv[1].to_string());
                        } else {
                            case_value = format!("{} {} {}", &caps1[1], "{0}", vv[1]);
                            values.push(vv[0].to_string());
                        }
                    }
                    3 => {
                        case_value = format!("{} {} {} {}", &caps1[1], "{0}", vv[1], "{1}");
                        values.push(vv[0].to_string());
                        values.push(vv[2].to_string());
                    }
                    _ => {
                        if REGEX_IMPRESSION_NUMBER.is_match(vv[0]) {
                            case_value = format!("{} {}", &caps1[1], "{0}");
                            values.push(vv[0].to_string());
                        } else {
                            case_value = format!("{} {}", &caps1[1], vv[0]);
                        }
                    }
                }
            }
            "$condition" => {
                case_value = format!("{} {}", &caps1[1], &caps1[2]);
            }
            "$PCcondition" => {
                let vv = caps1[2]
                    .split(" ")
                    .collect::<Vec<_>>();
                match vv[0] {
                    "sleepiness" => {
                        match vv.len() {
                            2 => {
                                case_value = format!("{} {} {}", &caps1[1], vv[0], "{0}");
                                values.push(vv[1].to_string());
                            }
                            3 => {
                                if vv[1] == "-" {
                                    case_value = format!("{} {} {} {}", &caps1[1], vv[0], vv[1], "{0}");
                                    values.push(vv[2].to_string());
                                } else {
                                    case_value = format!("{} {} {} {}", &caps1[1], vv[0], "{0}", vv[2]);
                                    values.push(vv[1].to_string());
                                }
                            }
                            4 => {
                                case_value = format!("{} {} {} {} {}", &caps1[1], vv[0], "{0}", vv[2], "{1}");
                                values.push(vv[1].to_string());
                                values.push(vv[3].to_string());
                            }
                            _ => {
                                case_value = format!("{} {}", &caps1[1], vv[0]);
                            }
                        }
                    }
                    _ => {
                        case_value = format!("{} {}", &caps1[1], vv[0]);
                    }
                }
            }
            "$karma" => {
                let vv = caps1[2]
                    .split(" ")
                    .collect::<Vec<_>>();
                match vv.len() {
                    2 => {
                        if vv[0] == "-" {
                            case_value = format!("{} {} {}", &caps1[1], vv[0], "{0}");
                            values.push(vv[1].to_string());
                        } else {
                            case_value = format!("{} {} {}", &caps1[1], "{0}", vv[1]);
                            values.push(vv[0].to_string());
                        }
                    }
                    3 => {
                        case_value = format!("{} {} {} {}", &caps1[1], "{0}", vv[1], "{1}");
                        values.push(vv[0].to_string());
                        values.push(vv[2].to_string());
                    }
                    _ => {
                        case_value = format!("{} {}", &caps1[1], "{0}");
                        values.push(vv[0].to_string());
                    }
                }
            }
            "$cash" => {
                let vv = caps1[2]
                    .split(" ")
                    .collect::<Vec<_>>();
                match vv.len() {
                    2 => {
                        if vv[0] == "-" {
                            case_value = format!("{} {} {}", &caps1[1], vv[0], "{0}");
                            values.push(vv[1].to_string());
                        } else {
                            case_value = format!("{} {} {}", &caps1[1], "{0}", vv[1]);
                            values.push(vv[0].to_string());
                        }
                    }
                    3 => {
                        case_value = format!("{} {} {} {}", &caps1[1], "{0}", vv[1], "{1}");
                        values.push(vv[0].to_string());
                        values.push(vv[2].to_string());
                    }
                    _ => {
                        case_value = format!("{} {}", &caps1[1], "{0}");
                        values.push(vv[0].to_string());
                    }
                }
            }
            "$PCcash" => {
                let vv = caps1[2]
                    .split(" ")
                    .collect::<Vec<_>>();
                match vv.len() {
                    2 => {
                        if vv[0] == "-" {
                            case_value = format!("{} {} {}", &caps1[1], vv[0], "{0}");
                            values.push(vv[1].to_string());
                        } else {
                            case_value = format!("{} {} {}", &caps1[1], "{0}", vv[1]);
                            values.push(vv[0].to_string());
                        }
                    }
                    3 => {
                        case_value = format!("{} {} {} {}", &caps1[1], "{0}", vv[1], "{1}");
                        values.push(vv[0].to_string());
                        values.push(vv[2].to_string());
                    }
                    _ => {
                        case_value = format!("{} {}", &caps1[1], "{0}");
                        values.push(vv[0].to_string());
                    }
                }
            }
            "$fame" => {
                let vv = caps1[2]
                    .split(" ")
                    .collect::<Vec<_>>();
                match vv.len() {
                    2 => {
                        if vv[0] == "-" {
                            case_value = format!("{} {} {}", &caps1[1], vv[0], "{0}");
                            values.push(vv[1].to_string());
                        } else {
                            case_value = format!("{} {} {}", &caps1[1], "{0}", vv[1]);
                            values.push(vv[0].to_string());
                        }
                    }
                    3 => {
                        case_value = format!("{} {} {} {}", &caps1[1], "{0}", vv[1], "{1}");
                        values.push(vv[0].to_string());
                        values.push(vv[2].to_string());
                    }
                    _ => {
                        case_value = format!("{} {}", &caps1[1], "{0}");
                        values.push(vv[0].to_string());
                    }
                }
            }
            "$PCfame" => {
                let vv = caps1[2]
                    .split(" ")
                    .collect::<Vec<_>>();
                match vv.len() {
                    2 => {
                        if vv[0] == "-" {
                            case_value = format!("{} {} {}", &caps1[1], vv[0], "{0}");
                            values.push(vv[1].to_string());
                        } else {
                            case_value = format!("{} {} {}", &caps1[1], "{0}", vv[1]);
                            values.push(vv[0].to_string());
                        }
                    }
                    3 => {
                        case_value = format!("{} {} {} {}", &caps1[1], "{0}", vv[1], "{1}");
                        values.push(vv[0].to_string());
                        values.push(vv[2].to_string());
                    }
                    _ => {
                        case_value = format!("{} {}", &caps1[1], "{0}");
                        values.push(vv[0].to_string());
                    }
                }
            }
            "$religion" => {
                case_value = format!("{} {}", &caps1[1], &caps1[2]);
            }
            "$PCreligion" => {
                case_value = format!("{} {}", &caps1[1], &caps1[2]);
            }
            "$action" => {
                case_value = format!("{} {}", &caps1[1], &caps1[2]);
            }
            "$PCaction" => {
                case_value = format!("{} {}", &caps1[1], &caps1[2]);
            }
            "$sex" => {
                case_value = format!("{} {}", &caps1[1], &caps1[2]);
            }
            "$PCsex" => {
                case_value = format!("{} {}", &caps1[1], &caps1[2]);
            }
            "$race" => {
                case_value = format!("{} {}", &caps1[1], "{0}");
                values.push(caps1[2].to_string());
            }
            "$PCrace" => {
                case_value = format!("{} {}", &caps1[1], "{0}");
                values.push(caps1[2].to_string());
            }
            "$class" => {
                case_value = format!("{} {}", &caps1[1], "{0}");
                values.push(caps1[2].to_string());
            }
            "$PCclass" => {
                case_value = format!("{} {}", &caps1[1], "{0}");
                values.push(caps1[2].to_string());
            }
            "$comparison" => {
                case_value = format!("{} {}", &caps1[1], "{0}");
                values.push(caps1[2].to_string());
            }
            "$random" => {
                case_value = format!("{} {}", &caps1[1], "{0}");
                values.push(caps1[2].to_string());
            }
            _ => {}
        }
        TextCase {
            case_value: case_value,
            values: values,
        }
    } else {
        TextCase {
            case_value: text.to_string(),
            values: Vec::new(),
        }
    }
}

fn parse_id(text: &str) -> TextId {
    let _id: Vec<_> = text.split(",").collect();

    let tags = &["%txtcast", "%txtpornobook", "%txtallykilled", "%txtactbefore", "%txtactafter"];
    for tag in tags {
        if let Some(_) = text.find(tag) {
            let re = format!(r"^{}(.*?)$", tag);
            let regex = Regex::new(&re).unwrap();
            if let Some(caps) = regex.captures(_id[0]) {
                return TextId { id: format!(r"{}{}", tag, "{0}"), value: caps[1].to_string() }
            }
        }
    }

    return TextId { id: _id[0].to_string(), value: "".into() }
}

fn parse_text(text: &str) -> Option<Vec<UserText>> {
    let mut txt: Vec<UserText> = Vec::new();

    let mut current_section = UserText {
        id: "".into(),
        value: "".into(),
        bodies: Vec::new(),
    };
    let mut current_body = UserTextBody {
        case_value: "".into(),
        values: Vec::new(),
        jp: Vec::new(),
    };
    let mut current_jp = Vec::new();
    let mut is_parsing_section = false;

    for line in text.lines() {
        let trimmed_line = line.trim();

        if line == "%endTxt" {
            break;
        }
        if REGEX_COMMENT.is_match(trimmed_line) {
            continue;
        }
        if REGEX_TAG_EN.is_match(trimmed_line) {
            is_parsing_section = false;
            continue;
        }
        let result3 = REGEX_TAG_JP.captures(trimmed_line);
        if result3.is_some() {
            if !current_jp.is_empty() {
                current_body.jp = current_jp.clone();
                current_jp.clear();
                current_section.bodies.push(current_body);
                current_body = UserTextBody {
                    case_value: "".into(),
                    values: Vec::new(),
                    jp: Vec::new(),
                };
            }
            if !current_section.bodies.is_empty() {
                txt.push(current_section);
            }
            let caps = result3.unwrap();
            let id = parse_id(&caps[1]);
            current_section = UserText {
                id: id.id,
                value: id.value,
                bodies: Vec::new(),
            };
            is_parsing_section = true;
        } else {
            let result4 = REGEX_EXPRESSION.find(trimmed_line);
            if result4.is_some() {
                if !current_jp.is_empty() {
                    current_body.jp = current_jp.clone();
                    current_jp.clear();
                    current_section.bodies.push(current_body);
                    let case = parse_case(result4.unwrap().as_str());
                    current_body = UserTextBody {
                        case_value: case.case_value,
                        values: case.values,
                        jp: Vec::new(),
                    };
                }
            } else if is_parsing_section {
                current_jp.push(trimmed_line.to_string());
            }
        }
    }

    if !current_jp.is_empty() {
        current_body.jp = current_jp.clone();
        current_section.bodies.push(current_body);
    }
    if !current_section.bodies.is_empty() {
        txt.push(current_section);
    }

    Some(txt)
}

fn parse_text_talk_order(text: &str) -> bool {
    return REGEX_TALK_ORDER.is_match(text);
}

fn to_equip(text: &str) -> Option<Equip> {
    let e = match text {
        "" => Equip { id: "".into(), custom_item_id: "".into() },
        s => match s.parse::<i64>().unwrap_or_else(|_| 743) {
            743 => {
                Equip { id: "743".into(), custom_item_id: s.to_string() }
            },
            _ => {
                Equip { id: s.to_string(), custom_item_id: "".into() }
            },
        },
    };
    return Some(e);
}

fn parse_list(text: &str, pat: &str) -> Option<Vec<String>> {
    let split = text.split(pat);
    let _vec: Vec<&str> = split.collect();
    let vec = _vec.iter()
        .filter(|x| !x.is_empty())
        .map(|x| x.to_string())
        .collect::<Vec<_>>();
    return Some(vec);
}

fn parse_value(text: &str, re: &str) -> Option<String> {
    let regex = Regex::new(re).unwrap();
    match regex.captures(text) {
        Some(caps) => {
            return Some((&caps[1]).to_string());
        }
        None => return None,
    }
}

fn parse_values(text: &str, re: &str, pat: &str) -> Option<Vec<String>> {
    let result = parse_value(text, re);
    match result {
        Some(value) => {
            if value == "" {
                return None
            }
            let split = value.split(pat);
            let _vec: Vec<&str> = split.collect();
            let vec = _vec.iter()
                .map(|&x| x.to_string())
                .collect::<Vec<_>>();
            return Some(vec);
        }
        None => return None,
    }
}

pub(crate) fn parse(text: &str) -> Result<Character, String> {
    let author = parse_value(text, r#"author\.\s*"(.*?)""#).unwrap_or_else(|| String::from(""));
    let _name = parse_values(text, r#"name\.\s*"(.*?)""#, ",").unwrap_or_else(|| vec![String::from(""), String::from("")]);
    let id = &_name[0];
    let name = &_name[1];
    let race = parse_value(text, r#"race\.\s*"(.*?)""#).unwrap_or_else(|| String::from(""));
    let class = parse_value(text, r#"class\.\s*"(.*?)""#).unwrap_or_else(|| String::from(""));
    let filter = parse_values(text, r#"filter\.\s*"(.*?)""#, "/").unwrap_or_else(|| Vec::new()).iter().cloned().filter(|x| !x.is_empty()).collect();
    let level = parse_value(text, r#"level\.\s*"(.*?)""#).unwrap_or_else(|| String::from("1")).parse::<i64>().unwrap();
    let relation = parse_value(text, r#"relation\.\s*"(.*?)""#).unwrap_or_else(|| String::from("-1")).parse::<i64>().unwrap();
    let sex = parse_value(text, r#"sex\.\s*"(.*?)""#).unwrap_or_else(|| String::from("-1")).parse::<i64>().unwrap();
    let fix_lv = parse_value(text, r#"fixLv\.\s*"(.*?)""#).unwrap_or_else(|| String::from("0")).parse::<i64>().unwrap();
    let rare = parse_value(text, r#"rare\.\s*"(.*?)""#).unwrap_or_else(|| String::from("0")).parse::<i64>().unwrap();
    let spawn_type = parse_value(text, r#"spawnType\.\s*"(.*?)""#).unwrap_or_else(|| String::from("0")).parse::<i64>().unwrap();
    let ai_calm = parse_value(text, r#"aiCalm\.\s*"(.*?)""#).unwrap_or_else(|| String::from("1")).parse::<i64>().unwrap();
    let ai_move = parse_value(text, r#"aiMove\.\s*"(.*?)""#).unwrap_or_else(|| String::from("50")).parse::<i64>().unwrap();
    let ai_dist = parse_value(text, r#"aiDist\.\s*"(.*?)""#).unwrap_or_else(|| String::from("1")).parse::<i64>().unwrap();
    let ai_heal = parse_value(text, r#"aiHeal\.\s*"(.*?)""#).unwrap_or_else(|| String::from("0")).parse::<i64>().unwrap();
    let _ai_act = parse_values(text, r#"aiAct\.\s*"(.*?)""#, ",").unwrap_or_else(|| vec![String::from("0"), String::from("0"), String::from("0"), String::from("0"), String::from("0")]);
    let ai_act0 = _ai_act[0].parse::<i64>().unwrap();
    let ai_act1 = _ai_act[1].parse::<i64>().unwrap();
    let ai_act2 = _ai_act[2].parse::<i64>().unwrap();
    let ai_act3 = _ai_act[3].parse::<i64>().unwrap();
    let ai_act4 = _ai_act[4].parse::<i64>().unwrap();
    let ai_act_sub_freq = parse_value(text, r#"aiActSubFreq\.\s*"(.*?)""#).unwrap_or_else(|| String::from("0")).parse::<i64>().unwrap();
    let _ai_act_sub = parse_values(text, r#"aiActSub\.\s*"(.*?)""#, ",").unwrap_or_else(|| vec![String::from("0"), String::from("0"), String::from("0"), String::from("0"), String::from("0")]);
    let ai_act_sub0 = _ai_act_sub[0].parse::<i64>().unwrap();
    let ai_act_sub1 = _ai_act_sub[1].parse::<i64>().unwrap();
    let ai_act_sub2 = _ai_act_sub[2].parse::<i64>().unwrap();
    let ai_act_sub3 = _ai_act_sub[3].parse::<i64>().unwrap();
    let ai_act_sub4 = _ai_act_sub[4].parse::<i64>().unwrap();
    let _melee_elem = parse_values(text, r#"meleeElem\.\s*"(.*?)""#, ",").unwrap_or_else(|| vec![String::from("0"), String::from("0")]);
    let melee_elem_id = _melee_elem[0].parse::<i64>().unwrap();
    let melee_elem_power = _melee_elem[1].parse::<i64>().unwrap();
    let _resist = parse_values(text, r#"resist\.\s*"(.*?)""#, ",").unwrap_or_else(|| Vec::new())
        .iter()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let mut resist: Vec<UserResist> = Vec::new();
    let mut i = 0;
    while i < _resist.len() {
        resist.push(
            UserResist::new(
                _resist[i],
                _resist[i+1]
            )
        );
        i += 2
    }
    let bit_on = parse_values(text, r#"bitOn\.\s*"(.*?)""#, ",").unwrap_or_else(|| Vec::new())
        .iter()
        .cloned()
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let transmissivity = parse_value(text, r#"transmissivity\.\s*"(.*?)""#).unwrap_or_else(|| String::from("0")).parse::<i64>().unwrap();
    let drop_shadow_type = parse_value(text, r#"dropShadowType\.\s*"(.*?)""#).unwrap_or_else(|| String::from("0")).parse::<i64>().unwrap();
    let c_set_pos = parse_value(text, r#"cSetPos\.\s*"(.*?)""#).unwrap_or_else(|| String::from("16")).parse::<i64>().unwrap();
    let no_food_or_drink = match parse_value(text, r#"noFoodOrDrink\.\s*"(.*?)""#).unwrap_or_else(|| String::from("0")).as_str() {
        "1" => true,
        _ => false,
    };
    let cnpc_role = parse_value(text, r#"cnpcRole\.\s*"(.*?)""#).unwrap_or_else(|| String::from("0")).parse::<i64>().unwrap();
    let race_alias = parse_value(text, r#"raceAlias\.\s*"(.*?)""#).unwrap_or_else(|| String::from(""));
    let class_alias = parse_value(text, r#"classAlias\.\s*"(.*?)""#).unwrap_or_else(|| String::from(""));
    let random_name = match parse_value(text, r#"randomName\.\s*"(.*?)""#).unwrap_or_else(|| String::from("0")).as_str() {
        "1" => true,
        _ => false,
    };
    let chipref = parse_value(text, r#"chipref\.\s*"(.*?)""#).unwrap_or_else(|| String::from("0")).parse::<i64>().unwrap();
    let colref = parse_value(text, r#"colref\.\s*"(.*?)""#).unwrap_or_else(|| String::from("0")).parse::<i64>().unwrap();
    let mut user_race = UserRace {
        name: "".into(),
        id: "".into(),
        id2: 0,
        playable: false,
        sex: 0,
        pic: 0,
        pic2: 0,
        dv: 0,
        pv: 0,
        hp: 0,
        mp: 0,
        str: 0,
        end: 0,
        dex: 0,
        per: 0,
        ler: 0,
        wil: 0,
        mag: 0,
        chr: 0,
        spd: 0,
        melee_style: 0,
        cast_style: 0,
        resist: 0,
        age_rnd: 0,
        age: 0,
        blood: 0,
        breeder: 0,
        height: 0,
        skill: Vec::new(),
        race_trait: Vec::new(),
        figure: Vec::new(),
        description: "".into(),
        desc_e: "".into(),
    };
    let _user_race = parse_values(text, r#"userRace\.\s*"(.*?)""#, ",").unwrap_or_else(|| Vec::new());
    let user_race_enabled = _user_race.len() != 0;
    if user_race_enabled {
        user_race.name = _user_race[0].to_string();
        user_race.id = _user_race[1].to_string();
        user_race.id2 = _user_race[2].parse::<i64>().unwrap_or_else(|_| 0);
        user_race.playable = match _user_race[3].as_str() {
            "1" => true,
            _ => false,
        };
        user_race.sex = _user_race[4].parse::<i64>().unwrap_or_else(|_| 0);
        user_race.pic = _user_race[5].parse::<i64>().unwrap_or_else(|_| 0);
        user_race.pic2 = _user_race[6].parse::<i64>().unwrap_or_else(|_| 0);
        user_race.dv = _user_race[7].parse::<i64>().unwrap_or_else(|_| 0);
        user_race.pv = _user_race[8].parse::<i64>().unwrap_or_else(|_| 0);
        user_race.hp = _user_race[9].parse::<i64>().unwrap_or_else(|_| 0);
        user_race.mp = _user_race[10].parse::<i64>().unwrap_or_else(|_| 0);
        user_race.str = _user_race[11].parse::<i64>().unwrap_or_else(|_| 0);
        user_race.end = _user_race[12].parse::<i64>().unwrap_or_else(|_| 0);
        user_race.dex = _user_race[13].parse::<i64>().unwrap_or_else(|_| 0);
        user_race.per = _user_race[14].parse::<i64>().unwrap_or_else(|_| 0);
        user_race.ler = _user_race[15].parse::<i64>().unwrap_or_else(|_| 0);
        user_race.wil = _user_race[16].parse::<i64>().unwrap_or_else(|_| 0);
        user_race.mag = _user_race[17].parse::<i64>().unwrap_or_else(|_| 0);
        user_race.chr = _user_race[18].parse::<i64>().unwrap_or_else(|_| 0);
        user_race.spd = _user_race[19].parse::<i64>().unwrap_or_else(|_| 0);
        user_race.melee_style = _user_race[20].parse::<i64>().unwrap_or_else(|_| 0);
        user_race.cast_style = _user_race[21].parse::<i64>().unwrap_or_else(|_| 0);
        user_race.resist = _user_race[22].parse::<i64>().unwrap_or_else(|_| 0);
        user_race.age_rnd = _user_race[23].parse::<i64>().unwrap_or_else(|_| 0);
        user_race.age = _user_race[24].parse::<i64>().unwrap_or_else(|_| 0);
        user_race.blood = _user_race[25].parse::<i64>().unwrap_or_else(|_| 0);
        user_race.breeder = _user_race[26].parse::<i64>().unwrap_or_else(|_| 0);
        user_race.height = _user_race[27].parse::<i64>().unwrap_or_else(|_| 0);
        user_race.skill = parse_list(&_user_race[28], "|").unwrap_or_else(|| Vec::new())
            .iter()
            .map(|x| {
                let i = x.parse::<i64>().unwrap();
                UserSkill::new(i / 10000, i % 10000)
            })
            .collect::<Vec<_>>();
        user_race.race_trait = parse_list(&_user_race[29], "|").unwrap_or_else(|| Vec::new())
            .iter()
            .map(|x| {
                let i = x.parse::<i64>().unwrap();
                UserTrait::new(i / 100, i % 100)
            })
            .collect::<Vec<_>>();
        user_race.figure = parse_list(&_user_race[30], "|").unwrap_or_else(|| Vec::new())
            .iter()
            .map(|x| Figure::new(x))
            .collect::<Vec<_>>();
        user_race.description = _user_race[31].to_string();
        user_race.desc_e = _user_race[32].to_string();
    }
    let mut user_class = UserClass {
        name: "".into(),
        id: "".into(),
        playable: false,
        str: 0,
        end: 0,
        dex: 0,
        per: 0,
        ler: 0,
        wil: 0,
        mag: 0,
        chr: 0,
        spd: 0,
        equip: 0,
        skill: Vec::new(),
        description: "".into(),
        desc_e: "".into(),
    };
    let _user_class = parse_values(text, r#"userClass\.\s*"(.*?)""#, ",").unwrap_or_else(|| Vec::new());
    let user_class_enabled = _user_class.len() != 0;
    if user_class_enabled {
        user_class.name = _user_class[0].to_string();
        user_class.id = _user_class[1].to_string();
        user_class.playable = match _user_class[2].as_str() {
            "1" => true,
            _ => false,
        };
        user_class.str = _user_class[3].parse::<i64>().unwrap_or_else(|_| 0);
        user_class.end = _user_class[4].parse::<i64>().unwrap_or_else(|_| 0);
        user_class.dex = _user_class[5].parse::<i64>().unwrap_or_else(|_| 0);
        user_class.per = _user_class[6].parse::<i64>().unwrap_or_else(|_| 0);
        user_class.ler = _user_class[7].parse::<i64>().unwrap_or_else(|_| 0);
        user_class.wil = _user_class[8].parse::<i64>().unwrap_or_else(|_| 0);
        user_class.mag = _user_class[9].parse::<i64>().unwrap_or_else(|_| 0);
        user_class.chr = _user_class[10].parse::<i64>().unwrap_or_else(|_| 0);
        user_class.spd = _user_class[11].parse::<i64>().unwrap_or_else(|_| 0);
        user_class.equip = _user_class[12].parse::<i64>().unwrap_or_else(|_| 0);
        user_class.skill = parse_list(&_user_class[13], "|").unwrap_or_else(|| Vec::new())
            .iter()
            .map(|x| {
                let i = x.parse::<i64>().unwrap();
                UserSkill::new(i / 10000, i % 10000)
            })
            .collect::<Vec<_>>();
        user_class.description = _user_class[14].to_string();
        user_class.desc_e = _user_class[15].to_string();
    }
    let mut init_equip = InitEquip {
        head: "".into(),
        head_custom_item_id: "".into(),
        weapon1: "".into(),
        weapon1_custom_item_id: "".into(),
        shield: "".into(),
        shield_custom_item_id: "".into(),
        shoot: "".into(),
        shoot_custom_item_id: "".into(),
        ammo: "".into(),
        ammo_custom_item_id: "".into(),
        weapon2: "".into(),
        weapon2_custom_item_id: "".into(),
        body: "".into(),
        body_custom_item_id: "".into(),
        arm: "".into(),
        arm_custom_item_id: "".into(),
        leg: "".into(),
        leg_custom_item_id: "".into(),
        back: "".into(),
        back_custom_item_id: "".into(),
        waist: "".into(),
        waist_custom_item_id: "".into(),
        ring1: "".into(),
        ring1_custom_item_id: "".into(),
        ring2: "".into(),
        ring2_custom_item_id: "".into(),
        neck1: "".into(),
        neck1_custom_item_id: "".into(),
        neck2: "".into(),
        neck2_custom_item_id: "".into(),
    };
    let _init_equip = parse_values(text, r#"initEquip\.\s*"(.*?)""#, ",").unwrap_or_else(|| Vec::new());
    let init_equip_enabled = _init_equip.len() != 0;
    if init_equip_enabled {
        let _head = to_equip(&_init_equip[0]).unwrap();
        init_equip.head = _head.id;
        init_equip.head_custom_item_id = _head.custom_item_id;
        let _weapon1 = to_equip(&_init_equip[1]).unwrap();
        init_equip.weapon1 = _weapon1.id;
        init_equip.weapon1_custom_item_id = _weapon1.custom_item_id;
        let _shield = to_equip(&_init_equip[2]).unwrap();
        init_equip.shield = _shield.id;
        init_equip.shield_custom_item_id = _shield.custom_item_id;
        let _shoot = to_equip(&_init_equip[3]).unwrap();
        init_equip.shoot = _shoot.id;
        init_equip.shoot_custom_item_id = _shoot.custom_item_id;
        let _ammo = to_equip(&_init_equip[4]).unwrap();
        init_equip.ammo = _ammo.id;
        init_equip.ammo_custom_item_id = _ammo.custom_item_id;
        let _weapon2 = to_equip(&_init_equip[5]).unwrap();
        init_equip.weapon2 = _weapon2.id;
        init_equip.weapon2_custom_item_id = _weapon2.custom_item_id;
        let _body = to_equip(&_init_equip[6]).unwrap();
        init_equip.body = _body.id;
        init_equip.body_custom_item_id = _body.custom_item_id;
        let _arm = to_equip(&_init_equip[7]).unwrap();
        init_equip.arm = _arm.id;
        init_equip.arm_custom_item_id = _arm.custom_item_id;
        let _leg = to_equip(&_init_equip[8]).unwrap();
        init_equip.leg = _leg.id;
        init_equip.leg_custom_item_id = _leg.custom_item_id;
        let _back = to_equip(&_init_equip[9]).unwrap();
        init_equip.back = _back.id;
        init_equip.back_custom_item_id = _back.custom_item_id;
        let _waist = to_equip(&_init_equip[10]).unwrap();
        init_equip.waist = _waist.id;
        init_equip.waist_custom_item_id = _waist.custom_item_id;
        let _ring1 = to_equip(&_init_equip[11]).unwrap();
        init_equip.ring1 = _ring1.id;
        init_equip.ring1_custom_item_id = _ring1.custom_item_id;
        let _ring2 = to_equip(&_init_equip[12]).unwrap();
        init_equip.ring2 = _ring2.id;
        init_equip.ring2_custom_item_id = _ring2.custom_item_id;
        let _neck1 = to_equip(&_init_equip[13]).unwrap();
        init_equip.neck1 = _neck1.id;
        init_equip.neck1_custom_item_id = _neck1.custom_item_id;
        let _neck2 = to_equip(&_init_equip[14]).unwrap();
        init_equip.neck2 = _neck2.id;
        init_equip.neck2_custom_item_id = _neck2.custom_item_id;
    }
    let mut _parsed_text: String;
    let mut _jp = "".into();
    let mut talk_enabled = false;
    if let Some(t) = parse_talk(text) {
        talk_enabled = true;
        _jp = t.jp;
        _parsed_text = t.parsed_text;
    } else {
        _parsed_text = text.to_string();
    }
    let txt_talk_order = parse_text_talk_order(&_parsed_text);
    let txt = parse_text(&_parsed_text).unwrap_or_else(|| Vec::new());
    Ok(
        Character {
            author,
            id: id.to_string(),
            name: name.to_string(),
            race,
            class,
            filter,
            level,
            relation,
            sex,
            fix_lv,
            rare,
            spawn_type,
            ai_calm,
            ai_move,
            ai_dist,
            ai_heal,
            ai_act0,
            ai_act1,
            ai_act2,
            ai_act3,
            ai_act4,
            ai_act_sub_freq,
            ai_act_sub0,
            ai_act_sub1,
            ai_act_sub2,
            ai_act_sub3,
            ai_act_sub4,
            melee_elem_id,
            melee_elem_power,
            resist,
            bit_on,
            transmissivity,
            drop_shadow_type,
            c_set_pos,
            no_food_or_drink,
            cnpc_role,
            race_alias,
            class_alias,
            random_name,
            chipref,
            colref,
            user_race_enabled,
            user_race,
            user_class_enabled,
            user_class,
            init_equip_enabled,
            init_equip,
            txt_talk_order,
            txt,
            talk_enabled,
            talk: UserTalk { jp: _jp },
        }
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_talk_none() {
        let text = "%Elona Custom Npc\r\n\r\nauthor.		\"だれか\"\r\nname.		\"younger brother,おとおと\"\r\nrace.		\"norland\"\r\nclass.		\"predator\"\r\nfilter.		\"/man/cnpc/\"\r\nlevel.		\"1\"\r\nrelation.	\"-1\"\r\nsex.		\"-1\"\r\nfixLv.		\"4\"\r\nrare.		\"100\"\r\nspawnType.	\"0\"\r\naiCalm.		\"4\"\r\naiMove.		\"100\"\r\naiDist.		\"1\"\r\naiHeal.		\"640\"\r\naiAct.		\"-1,-1,-2,651,0\"\r\naiActSubFreq.	\"20\"\r\naiActSub.	\"610,610,0,0,0\"\r\nmeleeElem.	\"61,200\"\r\nresist.		\"50,3,51,-5\"\r\nbitOn.		\"5,23\"\r\n\r\n%txtCalm,JP\r\n「わーい」\r\n「おにいちゃん！」\r\n\r\n%txtCalm,EN\r\n\"Weee.\"\r\n\"Brother!\"\r\n\r\n%txtAggro,JP\r\n「てめー」\r\n\r\n%txtAggro,EN\r\n\"Scum!\"\r\n\r\n%txtDead,JP\r\n「ちんだ」\r\n\r\n%txtDead,EN\r\n\"I'm dead.\"\r\n\r\n%txtKilled,JP\r\n「ころしたよー」\r\n\r\n%txtKilled,EN\r\n\"I killed it.\"\r\n\r\n%txtWelcome,JP\r\n「おかえり」\r\n\r\n%txtWelcome,EN\r\n\"Welcome back.\"\r\n\r\n%txtDialog,JP\r\nなあに？\r\n（おとおとはあなたをじっとみている)\r\n%txtDialog,EN\r\nHi.\r\nWhat's up?\r\nDude...\r\n%endTxt";
        let result = parse_talk(text);
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_talk_simple() {
        let text = "%Elona Custom Npc\r\n\r\nauthor.		\"だれか\"\r\nname.		\"explomias,解説ロミアス\"\r\nrace.		\"elea\"\r\nclass.		\"archer\"\r\nfilter.		\"/man/cnpc/\"\r\nlevel.		\"100\"\r\nrelation.	\"0\"\r\nsex.		\"-1\"\r\nfixLv.		\"6\"\r\nrare.		\"100\"\r\nspawnType.	\"0\"\r\naiCalm.		\"4\"\r\naiMove.		\"100\"\r\naiDist.		\"1\"\r\naiHeal.		\"640\"\r\naiAct.		\"-1,-1,-2,651,0\"\r\naiActSubFreq.	\"20\"\r\naiActSub.	\"610,610,0,0,0\"\r\nmeleeElem.	\"61,200\"\r\nresist.		\"50,3,51,-5\"\r\nbitOn.		\"5,23\"\r\n\r\n\r\n\r\n%txt_ucnpc_ev_b\r\n%txtevstart,JP\r\nCNPCの選択肢会話機能についての説明を聞きたいのか？{c}聞きたい%yes{ev},,,,,{c}余計な世話だ%no{ev},,,,,\r\n%txtyes,JP\r\n具体的に何について聞きたいんだ？{c}作り方%hensyu1{ev},,,,,{c}可能なこと%kano{ev},,,,,{c}注意%chui{ev},,,,,\r\n%txtno,JP\r\n…本当に聞かないつもりか？{c}聞く%yes{ev},,,,,{c}聞かない%joudan{ev},,,,,\r\n%txtjoudan,JP\r\nおいおい、冗談だろう？{c}聞く%yes{ev},,,,,{c}聞かない%no{ev},,,,,\r\n%txthensyu1,JP\r\n通常のCNPCのテキストに追加して記述することで可能になる。{n}bitOn.の下あたりから記述するとよいだろう。{n}ちなみにfixLv.を6にしないとこの機能は使えないぞ。{c}(続く)%hensyu2{ev},,,,,\r\n%txthensyu2,JP\r\n記述の順番は「会話の内容、選択肢、イベントアクション」だ。{n}ここで説明するには長すぎるから割愛させてもらおう。{n}「会話の内容」では\"you\"や\"me\"や\"n\"などが使えるが、{n}選択肢では使用できないから気を付けるといい。{c}(続く)%hensyu3{ev},,,,,\r\n%txthensyu3,JP\r\nなにはともあれ、説明のテキストを読んで{n}実際に作ってみるのが一番理解しやすいだろう。{n}陰ながら応援しているよ（ニヤリ）{c}他にも聞きたい%yes{ev},,,,,{c}さようなら%bye{ev},,,,,\r\n%txtkano,JP\r\nこのような選択肢による会話分岐のほかに{n}会話後にキャラを敵対させたり消滅させたりすることが可能だ。{n}他にもエモーションアイコンや効果音を出したり{n}流れている音楽を変えることもできる。{n}ただ、アイテムやキャラクターの生成は{n}ゲームバランスを考えて不可能にしている。{n}何か試してみるか？{c}敵対%tekitai{ev},,,,,{c}消滅%evstart{ev}vanq,,,,,{c}エモーションアイコン%emo{ev}emolove,,,,,{c}効果音%koukaon{ev}curse3.wav,,,,,{c}(次へ)%kano2{ev},,,,,\r\n%txttekitai,JP\r\n…本当に試してしまったのか？{c}えー%bye{ev}gohos,,,,,\r\n%txtemo,JP\r\nエモーションアイコンを出したぞ。{c}見えない%emo2{ev},,,,,\r\n%txtemo2,JP\r\nおいおい、会話ウィンドウに隠れて見えないのは{n}少し考えれば分かることだろう。{n}これでエモーションアイコンを出すときは{n}会話を終了するときにするべきだというのが理解できたな。{n}（ロミアスはにやりと笑った）{c}さようなら%bye{ev},,,,,\r\n%txtkoukaon,JP\r\nあなたのエーテル抗体のポーションは黒く輝いた。{c}(続く)%koukaon2{ev},,,,,\r\n%txtkoukaon2,JP\r\n…おっと、驚かせてしまったかな。{n}もちろん、本当に呪われたわけではないから安心するといい。{n}{n}さて、他に何か聞きたいことはあるか？{c}ある%yes{ev},,,,,{c}ない%bye{ev},,,,,\r\n%txtongaku,JP\r\nどうだ、少しは緊張感のある音楽になっただろう。{n}もっとも、他人と自分が同じBGMを使っているとは限らないから、{n}場合によっては大変間抜けなことになる可能性もあるがな。{n}{n}さて、他に何か聞きたいことはあるか？{c}ある%yes{ev},,,,,{c}ない%bye{ev},,,,,\r\n%txtchui,JP\r\nそうだな、まずテキストは間違いのないように記述することだ。{n}上手く動作しないという場合は必ずテキストを見なおすべきだ。{n}それと抜けられない無限ループを作るのはやめておいた方がいい。{n}Shiftで会話から抜けられるとはいえ、やはり印象は悪い。{n}あとは…そうだな。{n}無限ループを避けるために{n}選択肢は300回までしか使えないようになっている。{n}もっとも、そんなに大量の文章を書く変態はいないだろうがね。{n}（ロミアスはにやりと笑った）{c}(続く)%chui2{ev},,,,,\r\n%txtchui2,JP\r\nさて、他に何か聞きたいことはあるか？{c}ある%yes{ev},,,,,{c}ない%bye{ev},,,,,\r\n%txtbye,JP\r\n\r\n%txtkano2,JP\r\nこのような選択肢による会話分岐のほかに{n}会話後にキャラを敵対させたり消滅させたりすることが可能だ。{n}他にもエモーションアイコンや効果音を出したり{n}流れている音楽を変えることもできる。{n}ただ、アイテムやキャラクターの生成は{n}ゲームバランスを考えて不可能にしている。{n}何か試してみるか？{c}BGM変更%ongaku{ev}mcLastBoss,,,,,{c}気持ちいいことしない？%kimoti{ev},,,,,{c}(戻る)%kano{ev},,,,,\r\n%txtkimoti,JP\r\nおいおい、照れるじゃないか。{c}いやん♪%bye{ev}iikoto,,,,,\r\n%txt_ucnpc_ev_e\r\n\r\n\r\n%txt_talk_order\r\n%txtCalm,JP\r\n\r\n%txtAggro,JP\r\n\r\n%txtDead,JP\r\n\r\n%txtKilled,JP\r\n\r\n%txtWelcome,JP\r\n\r\n%txtDialog,JP\r\n\r\n%txtabuse,JP\r\ndefault\r\n\r\n%txtmarriage,JP\r\ndefault\r\n\r\n%txtanata,JP\r\ndefault\r\n\r\n%txtiyayo,JP\r\ndefault\r\n\r\n%txtnakanaka,JP\r\ndefault\r\n\r\n%txtikuyo,JP\r\ndefault\r\n\r\n%txtkiyameru,JP\r\ndefault\r\n\r\n%txtkuyasii,JP\r\ndefault\r\n\r\n%txtjigo,JP\r\ndefault\r\n%txtnoru,JP\r\ndefault\r\n\r\n%txtoriru,JP\r\ndefault\r\n\r\n%txtbiyaku,JP\r\ndefault\r\n\r\n%txttiti,JP\r\ndefault\r\n\r\n%txtsaite,JP\r\ndefault\r\n\r\n%txtsand,JP\r\ndefault\r\n\r\n%txtnikorose,JP\r\ndefault\r\n\r\n%txtkya,JP\r\ndefault\r\n\r\n%txttyohazusu,JP\r\ndefault\r\n\r\n%txtsibaru,JP\r\ndefault\r\n\r\n%txthodoku,JP\r\ndefault\r\n\r\n%txtturusu,JP\r\ndefault\r\n\r\n%txtsorosu,JP\r\ndefault\r\n\r\n%txtsnaguru,JP\r\ndefault\r\n\r\n%txtomiyage,JP\r\ndefault\r\n\r\n%txtyubikubi,JP\r\ndefault\r\n\r\n%txttoriage,JP\r\ndefault\r\n\r\n%txtpbou,JP\r\ndefault\r\n\r\n%txtexthank,JP\r\ndefault\r\n\r\n%txtexhiya,JP\r\ndefault\r\n\r\n%txtgoei,JP\r\ndefault\r\n\r\n%txtyatou,JP\r\ndefault\r\n\r\n%txthihiya,JP\r\ndefault\r\n\r\n%txtumaku,JP\r\ndefault\r\n\r\n%txttikara,JP\r\ndefault\r\n\r\n%txt0free,JP\r\ndefault\r\n\r\n%txtokoto,JP\r\ndefault\r\n\r\n%txtsibui,JP\r\ndefault\r\n\r\n%txtnamaniku,JP\r\ndefault\r\n\r\n%txtkona,JP\r\ndefault\r\n\r\n%txtnamamen,JP\r\ndefault\r\n\r\n%txtheibon,JP\r\ndefault\r\n\r\n%txt1_2,JP\r\ndefault\r\n\r\n%txt3_4,JP\r\ndefault\r\n\r\n%txt5_6,JP\r\ndefault\r\n\r\n%txt7_8,JP\r\ndefault\r\n\r\n%txt9saiko,JP\r\ndefault\r\n\r\n%txtkaradake,JP\r\ndefault\r\n\r\n%txtyanwari,JP\r\ndefault\r\n\r\n%txtkunren,JP\r\ndefault\r\n\r\n%txtonaka,JP\r\ndefault\r\n\r\n%txthinsi,JP\r\ndefault\r\n\r\n%txtkodukuri,JP\r\ndefault\r\n\r\n%txtlayhand,JP\r\ndefault\r\n\r\n%endTxt";
        let result = parse_talk(text);
        assert!(result.is_some());
        let talk = result.unwrap();
        assert_eq!(talk.jp, "%txt_ucnpc_ev_b\r\n%txtevstart,JP\r\nCNPCの選択肢会話機能についての説明を聞きたいのか？{c}聞きたい%yes{ev},,,,,{c}余計な世話だ%no{ev},,,,,\r\n%txtyes,JP\r\n具体的に何について聞きたいんだ？{c}作り方%hensyu1{ev},,,,,{c}可能なこと%kano{ev},,,,,{c}注意%chui{ev},,,,,\r\n%txtno,JP\r\n…本当に聞かないつもりか？{c}聞く%yes{ev},,,,,{c}聞かない%joudan{ev},,,,,\r\n%txtjoudan,JP\r\nおいおい、冗談だろう？{c}聞く%yes{ev},,,,,{c}聞かない%no{ev},,,,,\r\n%txthensyu1,JP\r\n通常のCNPCのテキストに追加して記述することで可能になる。{n}bitOn.の下あたりから記述するとよいだろう。{n}ちなみにfixLv.を6にしないとこの機能は使えないぞ。{c}(続く)%hensyu2{ev},,,,,\r\n%txthensyu2,JP\r\n記述の順番は「会話の内容、選択肢、イベントアクション」だ。{n}ここで説明するには長すぎるから割愛させてもらおう。{n}「会話の内容」では\"you\"や\"me\"や\"n\"などが使えるが、{n}選択肢では使用できないから気を付けるといい。{c}(続く)%hensyu3{ev},,,,,\r\n%txthensyu3,JP\r\nなにはともあれ、説明のテキストを読んで{n}実際に作ってみるのが一番理解しやすいだろう。{n}陰ながら応援しているよ（ニヤリ）{c}他にも聞きたい%yes{ev},,,,,{c}さようなら%bye{ev},,,,,\r\n%txtkano,JP\r\nこのような選択肢による会話分岐のほかに{n}会話後にキャラを敵対させたり消滅させたりすることが可能だ。{n}他にもエモーションアイコンや効果音を出したり{n}流れている音楽を変えることもできる。{n}ただ、アイテムやキャラクターの生成は{n}ゲームバランスを考えて不可能にしている。{n}何か試してみるか？{c}敵対%tekitai{ev},,,,,{c}消滅%evstart{ev}vanq,,,,,{c}エモーションアイコン%emo{ev}emolove,,,,,{c}効果音%koukaon{ev}curse3.wav,,,,,{c}(次へ)%kano2{ev},,,,,\r\n%txttekitai,JP\r\n…本当に試してしまったのか？{c}えー%bye{ev}gohos,,,,,\r\n%txtemo,JP\r\nエモーションアイコンを出したぞ。{c}見えない%emo2{ev},,,,,\r\n%txtemo2,JP\r\nおいおい、会話ウィンドウに隠れて見えないのは{n}少し考えれば分かることだろう。{n}これでエモーションアイコンを出すときは{n}会話を終了するときにするべきだというのが理解できたな。{n}（ロミアスはにやりと笑った）{c}さようなら%bye{ev},,,,,\r\n%txtkoukaon,JP\r\nあなたのエーテル抗体のポーションは黒く輝いた。{c}(続く)%koukaon2{ev},,,,,\r\n%txtkoukaon2,JP\r\n…おっと、驚かせてしまったかな。{n}もちろん、本当に呪われたわけではないから安心するといい。{n}{n}さて、他に何か聞きたいことはあるか？{c}ある%yes{ev},,,,,{c}ない%bye{ev},,,,,\r\n%txtongaku,JP\r\nどうだ、少しは緊張感のある音楽になっただろう。{n}もっとも、他人と自分が同じBGMを使っているとは限らないから、{n}場合によっては大変間抜けなことになる可能性もあるがな。{n}{n}さて、他に何か聞きたいことはあるか？{c}ある%yes{ev},,,,,{c}ない%bye{ev},,,,,\r\n%txtchui,JP\r\nそうだな、まずテキストは間違いのないように記述することだ。{n}上手く動作しないという場合は必ずテキストを見なおすべきだ。{n}それと抜けられない無限ループを作るのはやめておいた方がいい。{n}Shiftで会話から抜けられるとはいえ、やはり印象は悪い。{n}あとは…そうだな。{n}無限ループを避けるために{n}選択肢は300回までしか使えないようになっている。{n}もっとも、そんなに大量の文章を書く変態はいないだろうがね。{n}（ロミアスはにやりと笑った）{c}(続く)%chui2{ev},,,,,\r\n%txtchui2,JP\r\nさて、他に何か聞きたいことはあるか？{c}ある%yes{ev},,,,,{c}ない%bye{ev},,,,,\r\n%txtbye,JP\r\n\r\n%txtkano2,JP\r\nこのような選択肢による会話分岐のほかに{n}会話後にキャラを敵対させたり消滅させたりすることが可能だ。{n}他にもエモーションアイコンや効果音を出したり{n}流れている音楽を変えることもできる。{n}ただ、アイテムやキャラクターの生成は{n}ゲームバランスを考えて不可能にしている。{n}何か試してみるか？{c}BGM変更%ongaku{ev}mcLastBoss,,,,,{c}気持ちいいことしない？%kimoti{ev},,,,,{c}(戻る)%kano{ev},,,,,\r\n%txtkimoti,JP\r\nおいおい、照れるじゃないか。{c}いやん♪%bye{ev}iikoto,,,,,\r\n%txt_ucnpc_ev_e");
        assert_eq!(talk.parsed_text, "%Elona Custom Npc\r\n\r\nauthor.		\"だれか\"\r\nname.		\"explomias,解説ロミアス\"\r\nrace.		\"elea\"\r\nclass.		\"archer\"\r\nfilter.		\"/man/cnpc/\"\r\nlevel.		\"100\"\r\nrelation.	\"0\"\r\nsex.		\"-1\"\r\nfixLv.		\"6\"\r\nrare.		\"100\"\r\nspawnType.	\"0\"\r\naiCalm.		\"4\"\r\naiMove.		\"100\"\r\naiDist.		\"1\"\r\naiHeal.		\"640\"\r\naiAct.		\"-1,-1,-2,651,0\"\r\naiActSubFreq.	\"20\"\r\naiActSub.	\"610,610,0,0,0\"\r\nmeleeElem.	\"61,200\"\r\nresist.		\"50,3,51,-5\"\r\nbitOn.		\"5,23\"\r\n\r\n\r\n\r\n\r\n\r\n\r\n%txt_talk_order\r\n%txtCalm,JP\r\n\r\n%txtAggro,JP\r\n\r\n%txtDead,JP\r\n\r\n%txtKilled,JP\r\n\r\n%txtWelcome,JP\r\n\r\n%txtDialog,JP\r\n\r\n%txtabuse,JP\r\ndefault\r\n\r\n%txtmarriage,JP\r\ndefault\r\n\r\n%txtanata,JP\r\ndefault\r\n\r\n%txtiyayo,JP\r\ndefault\r\n\r\n%txtnakanaka,JP\r\ndefault\r\n\r\n%txtikuyo,JP\r\ndefault\r\n\r\n%txtkiyameru,JP\r\ndefault\r\n\r\n%txtkuyasii,JP\r\ndefault\r\n\r\n%txtjigo,JP\r\ndefault\r\n%txtnoru,JP\r\ndefault\r\n\r\n%txtoriru,JP\r\ndefault\r\n\r\n%txtbiyaku,JP\r\ndefault\r\n\r\n%txttiti,JP\r\ndefault\r\n\r\n%txtsaite,JP\r\ndefault\r\n\r\n%txtsand,JP\r\ndefault\r\n\r\n%txtnikorose,JP\r\ndefault\r\n\r\n%txtkya,JP\r\ndefault\r\n\r\n%txttyohazusu,JP\r\ndefault\r\n\r\n%txtsibaru,JP\r\ndefault\r\n\r\n%txthodoku,JP\r\ndefault\r\n\r\n%txtturusu,JP\r\ndefault\r\n\r\n%txtsorosu,JP\r\ndefault\r\n\r\n%txtsnaguru,JP\r\ndefault\r\n\r\n%txtomiyage,JP\r\ndefault\r\n\r\n%txtyubikubi,JP\r\ndefault\r\n\r\n%txttoriage,JP\r\ndefault\r\n\r\n%txtpbou,JP\r\ndefault\r\n\r\n%txtexthank,JP\r\ndefault\r\n\r\n%txtexhiya,JP\r\ndefault\r\n\r\n%txtgoei,JP\r\ndefault\r\n\r\n%txtyatou,JP\r\ndefault\r\n\r\n%txthihiya,JP\r\ndefault\r\n\r\n%txtumaku,JP\r\ndefault\r\n\r\n%txttikara,JP\r\ndefault\r\n\r\n%txt0free,JP\r\ndefault\r\n\r\n%txtokoto,JP\r\ndefault\r\n\r\n%txtsibui,JP\r\ndefault\r\n\r\n%txtnamaniku,JP\r\ndefault\r\n\r\n%txtkona,JP\r\ndefault\r\n\r\n%txtnamamen,JP\r\ndefault\r\n\r\n%txtheibon,JP\r\ndefault\r\n\r\n%txt1_2,JP\r\ndefault\r\n\r\n%txt3_4,JP\r\ndefault\r\n\r\n%txt5_6,JP\r\ndefault\r\n\r\n%txt7_8,JP\r\ndefault\r\n\r\n%txt9saiko,JP\r\ndefault\r\n\r\n%txtkaradake,JP\r\ndefault\r\n\r\n%txtyanwari,JP\r\ndefault\r\n\r\n%txtkunren,JP\r\ndefault\r\n\r\n%txtonaka,JP\r\ndefault\r\n\r\n%txthinsi,JP\r\ndefault\r\n\r\n%txtkodukuri,JP\r\ndefault\r\n\r\n%txtlayhand,JP\r\ndefault\r\n\r\n%endTxt");
    }

    #[test]
    fn test_parse_text_talk_order_none() {
        let text = "%Elona Custom Npc\r\n\r\nauthor.		\"だれか\"\r\nname.		\"younger brother,おとおと\"\r\nrace.		\"norland\"\r\nclass.		\"predator\"\r\nfilter.		\"/man/cnpc/\"\r\nlevel.		\"1\"\r\nrelation.	\"-1\"\r\nsex.		\"-1\"\r\nfixLv.		\"4\"\r\nrare.		\"100\"\r\nspawnType.	\"0\"\r\naiCalm.		\"4\"\r\naiMove.		\"100\"\r\naiDist.		\"1\"\r\naiHeal.		\"640\"\r\naiAct.		\"-1,-1,-2,651,0\"\r\naiActSubFreq.	\"20\"\r\naiActSub.	\"610,610,0,0,0\"\r\nmeleeElem.	\"61,200\"\r\nresist.		\"50,3,51,-5\"\r\nbitOn.		\"5,23\"\r\n\r\n%txtCalm,JP\r\n「わーい」\r\n「おにいちゃん！」\r\n\r\n%txtCalm,EN\r\n\"Weee.\"\r\n\"Brother!\"\r\n\r\n%txtAggro,JP\r\n「てめー」\r\n\r\n%txtAggro,EN\r\n\"Scum!\"\r\n\r\n%txtDead,JP\r\n「ちんだ」\r\n\r\n%txtDead,EN\r\n\"I'm dead.\"\r\n\r\n%txtKilled,JP\r\n「ころしたよー」\r\n\r\n%txtKilled,EN\r\n\"I killed it.\"\r\n\r\n%txtWelcome,JP\r\n「おかえり」\r\n\r\n%txtWelcome,EN\r\n\"Welcome back.\"\r\n\r\n%txtDialog,JP\r\nなあに？\r\n（おとおとはあなたをじっとみている)\r\n%txtDialog,EN\r\nHi.\r\nWhat's up?\r\nDude...\r\n%endTxt";
        let result = parse_text_talk_order(text);
        assert!(result == false);
    }

    #[test]
    fn test_parse_text_talk_order_simple() {
        let text = "%Elona Custom Npc\r\n\r\nauthor.		\"だれか\"\r\nname.		\"explomias,解説ロミアス\"\r\nrace.		\"elea\"\r\nclass.		\"archer\"\r\nfilter.		\"/man/cnpc/\"\r\nlevel.		\"100\"\r\nrelation.	\"0\"\r\nsex.		\"-1\"\r\nfixLv.		\"6\"\r\nrare.		\"100\"\r\nspawnType.	\"0\"\r\naiCalm.		\"4\"\r\naiMove.		\"100\"\r\naiDist.		\"1\"\r\naiHeal.		\"640\"\r\naiAct.		\"-1,-1,-2,651,0\"\r\naiActSubFreq.	\"20\"\r\naiActSub.	\"610,610,0,0,0\"\r\nmeleeElem.	\"61,200\"\r\nresist.		\"50,3,51,-5\"\r\nbitOn.		\"5,23\"\r\n\r\n\r\n\r\n%txt_ucnpc_ev_b\r\n%txtevstart,JP\r\nCNPCの選択肢会話機能についての説明を聞きたいのか？{c}聞きたい%yes{ev},,,,,{c}余計な世話だ%no{ev},,,,,\r\n%txtyes,JP\r\n具体的に何について聞きたいんだ？{c}作り方%hensyu1{ev},,,,,{c}可能なこと%kano{ev},,,,,{c}注意%chui{ev},,,,,\r\n%txtno,JP\r\n…本当に聞かないつもりか？{c}聞く%yes{ev},,,,,{c}聞かない%joudan{ev},,,,,\r\n%txtjoudan,JP\r\nおいおい、冗談だろう？{c}聞く%yes{ev},,,,,{c}聞かない%no{ev},,,,,\r\n%txthensyu1,JP\r\n通常のCNPCのテキストに追加して記述することで可能になる。{n}bitOn.の下あたりから記述するとよいだろう。{n}ちなみにfixLv.を6にしないとこの機能は使えないぞ。{c}(続く)%hensyu2{ev},,,,,\r\n%txthensyu2,JP\r\n記述の順番は「会話の内容、選択肢、イベントアクション」だ。{n}ここで説明するには長すぎるから割愛させてもらおう。{n}「会話の内容」では\"you\"や\"me\"や\"n\"などが使えるが、{n}選択肢では使用できないから気を付けるといい。{c}(続く)%hensyu3{ev},,,,,\r\n%txthensyu3,JP\r\nなにはともあれ、説明のテキストを読んで{n}実際に作ってみるのが一番理解しやすいだろう。{n}陰ながら応援しているよ（ニヤリ）{c}他にも聞きたい%yes{ev},,,,,{c}さようなら%bye{ev},,,,,\r\n%txtkano,JP\r\nこのような選択肢による会話分岐のほかに{n}会話後にキャラを敵対させたり消滅させたりすることが可能だ。{n}他にもエモーションアイコンや効果音を出したり{n}流れている音楽を変えることもできる。{n}ただ、アイテムやキャラクターの生成は{n}ゲームバランスを考えて不可能にしている。{n}何か試してみるか？{c}敵対%tekitai{ev},,,,,{c}消滅%evstart{ev}vanq,,,,,{c}エモーションアイコン%emo{ev}emolove,,,,,{c}効果音%koukaon{ev}curse3.wav,,,,,{c}(次へ)%kano2{ev},,,,,\r\n%txttekitai,JP\r\n…本当に試してしまったのか？{c}えー%bye{ev}gohos,,,,,\r\n%txtemo,JP\r\nエモーションアイコンを出したぞ。{c}見えない%emo2{ev},,,,,\r\n%txtemo2,JP\r\nおいおい、会話ウィンドウに隠れて見えないのは{n}少し考えれば分かることだろう。{n}これでエモーションアイコンを出すときは{n}会話を終了するときにするべきだというのが理解できたな。{n}（ロミアスはにやりと笑った）{c}さようなら%bye{ev},,,,,\r\n%txtkoukaon,JP\r\nあなたのエーテル抗体のポーションは黒く輝いた。{c}(続く)%koukaon2{ev},,,,,\r\n%txtkoukaon2,JP\r\n…おっと、驚かせてしまったかな。{n}もちろん、本当に呪われたわけではないから安心するといい。{n}{n}さて、他に何か聞きたいことはあるか？{c}ある%yes{ev},,,,,{c}ない%bye{ev},,,,,\r\n%txtongaku,JP\r\nどうだ、少しは緊張感のある音楽になっただろう。{n}もっとも、他人と自分が同じBGMを使っているとは限らないから、{n}場合によっては大変間抜けなことになる可能性もあるがな。{n}{n}さて、他に何か聞きたいことはあるか？{c}ある%yes{ev},,,,,{c}ない%bye{ev},,,,,\r\n%txtchui,JP\r\nそうだな、まずテキストは間違いのないように記述することだ。{n}上手く動作しないという場合は必ずテキストを見なおすべきだ。{n}それと抜けられない無限ループを作るのはやめておいた方がいい。{n}Shiftで会話から抜けられるとはいえ、やはり印象は悪い。{n}あとは…そうだな。{n}無限ループを避けるために{n}選択肢は300回までしか使えないようになっている。{n}もっとも、そんなに大量の文章を書く変態はいないだろうがね。{n}（ロミアスはにやりと笑った）{c}(続く)%chui2{ev},,,,,\r\n%txtchui2,JP\r\nさて、他に何か聞きたいことはあるか？{c}ある%yes{ev},,,,,{c}ない%bye{ev},,,,,\r\n%txtbye,JP\r\n\r\n%txtkano2,JP\r\nこのような選択肢による会話分岐のほかに{n}会話後にキャラを敵対させたり消滅させたりすることが可能だ。{n}他にもエモーションアイコンや効果音を出したり{n}流れている音楽を変えることもできる。{n}ただ、アイテムやキャラクターの生成は{n}ゲームバランスを考えて不可能にしている。{n}何か試してみるか？{c}BGM変更%ongaku{ev}mcLastBoss,,,,,{c}気持ちいいことしない？%kimoti{ev},,,,,{c}(戻る)%kano{ev},,,,,\r\n%txtkimoti,JP\r\nおいおい、照れるじゃないか。{c}いやん♪%bye{ev}iikoto,,,,,\r\n%txt_ucnpc_ev_e\r\n\r\n\r\n%txt_talk_order\r\n%txtCalm,JP\r\n\r\n%txtAggro,JP\r\n\r\n%txtDead,JP\r\n\r\n%txtKilled,JP\r\n\r\n%txtWelcome,JP\r\n\r\n%txtDialog,JP\r\n\r\n%txtabuse,JP\r\ndefault\r\n\r\n%txtmarriage,JP\r\ndefault\r\n\r\n%txtanata,JP\r\ndefault\r\n\r\n%txtiyayo,JP\r\ndefault\r\n\r\n%txtnakanaka,JP\r\ndefault\r\n\r\n%txtikuyo,JP\r\ndefault\r\n\r\n%txtkiyameru,JP\r\ndefault\r\n\r\n%txtkuyasii,JP\r\ndefault\r\n\r\n%txtjigo,JP\r\ndefault\r\n%txtnoru,JP\r\ndefault\r\n\r\n%txtoriru,JP\r\ndefault\r\n\r\n%txtbiyaku,JP\r\ndefault\r\n\r\n%txttiti,JP\r\ndefault\r\n\r\n%txtsaite,JP\r\ndefault\r\n\r\n%txtsand,JP\r\ndefault\r\n\r\n%txtnikorose,JP\r\ndefault\r\n\r\n%txtkya,JP\r\ndefault\r\n\r\n%txttyohazusu,JP\r\ndefault\r\n\r\n%txtsibaru,JP\r\ndefault\r\n\r\n%txthodoku,JP\r\ndefault\r\n\r\n%txtturusu,JP\r\ndefault\r\n\r\n%txtsorosu,JP\r\ndefault\r\n\r\n%txtsnaguru,JP\r\ndefault\r\n\r\n%txtomiyage,JP\r\ndefault\r\n\r\n%txtyubikubi,JP\r\ndefault\r\n\r\n%txttoriage,JP\r\ndefault\r\n\r\n%txtpbou,JP\r\ndefault\r\n\r\n%txtexthank,JP\r\ndefault\r\n\r\n%txtexhiya,JP\r\ndefault\r\n\r\n%txtgoei,JP\r\ndefault\r\n\r\n%txtyatou,JP\r\ndefault\r\n\r\n%txthihiya,JP\r\ndefault\r\n\r\n%txtumaku,JP\r\ndefault\r\n\r\n%txttikara,JP\r\ndefault\r\n\r\n%txt0free,JP\r\ndefault\r\n\r\n%txtokoto,JP\r\ndefault\r\n\r\n%txtsibui,JP\r\ndefault\r\n\r\n%txtnamaniku,JP\r\ndefault\r\n\r\n%txtkona,JP\r\ndefault\r\n\r\n%txtnamamen,JP\r\ndefault\r\n\r\n%txtheibon,JP\r\ndefault\r\n\r\n%txt1_2,JP\r\ndefault\r\n\r\n%txt3_4,JP\r\ndefault\r\n\r\n%txt5_6,JP\r\ndefault\r\n\r\n%txt7_8,JP\r\ndefault\r\n\r\n%txt9saiko,JP\r\ndefault\r\n\r\n%txtkaradake,JP\r\ndefault\r\n\r\n%txtyanwari,JP\r\ndefault\r\n\r\n%txtkunren,JP\r\ndefault\r\n\r\n%txtonaka,JP\r\ndefault\r\n\r\n%txthinsi,JP\r\ndefault\r\n\r\n%txtkodukuri,JP\r\ndefault\r\n\r\n%txtlayhand,JP\r\ndefault\r\n\r\n%endTxt";
        let result = parse_text_talk_order(text);
        assert!(result == true);
    }

    #[test]
    fn test_parse_text_simple() {
        let text = "%Elona Custom Npc\r\n\r\nauthor.		\"だれか\"\r\nname.		\"younger brother,おとおと\"\r\nrace.		\"norland\"\r\nclass.		\"predator\"\r\nfilter.		\"/man/cnpc/\"\r\nlevel.		\"1\"\r\nrelation.	\"-1\"\r\nsex.		\"-1\"\r\nfixLv.		\"4\"\r\nrare.		\"100\"\r\nspawnType.	\"0\"\r\naiCalm.		\"4\"\r\naiMove.		\"100\"\r\naiDist.		\"1\"\r\naiHeal.		\"640\"\r\naiAct.		\"-1,-1,-2,651,0\"\r\naiActSubFreq.	\"20\"\r\naiActSub.	\"610,610,0,0,0\"\r\nmeleeElem.	\"61,200\"\r\nresist.		\"50,3,51,-5\"\r\nbitOn.		\"5,23\"\r\n\r\n%txtCalm,JP\r\n「わーい」\r\n「おにいちゃん！」\r\n\r\n%txtCalm,EN\r\n\"Weee.\"\r\n\"Brother!\"\r\n\r\n%txtAggro,JP\r\n「てめー」\r\n\r\n%txtAggro,EN\r\n\"Scum!\"\r\n\r\n%txtDead,JP\r\n「ちんだ」\r\n\r\n%txtDead,EN\r\n\"I'm dead.\"\r\n\r\n%txtKilled,JP\r\n「ころしたよー」\r\n\r\n%txtKilled,EN\r\n\"I killed it.\"\r\n\r\n%txtWelcome,JP\r\n「おかえり」\r\n\r\n%txtWelcome,EN\r\n\"Welcome back.\"\r\n\r\n%txtDialog,JP\r\nなあに？\r\n（おとおとはあなたをじっとみている)\r\n%txtDialog,EN\r\nHi.\r\nWhat's up?\r\nDude...\r\n%endTxt";
        let result = parse_text(text);
        assert!(result.is_some());
        let v = result.unwrap();
        assert_eq!(v.len(), 6);
        assert_eq!(v.get(0).unwrap().id, "%txtCalm");
        let bodies = &v.get(0).unwrap().bodies;
        assert_eq!(bodies.len(), 1);
        let jp = &bodies.get(0).unwrap().jp;
        assert_eq!(jp.len(), 3);
        assert_eq!(v.get(5).unwrap().bodies.get(0).unwrap().jp.len(), 2);

        let text2 = "%Elona Custom Npc\r\n\r\nauthor.		\"だれか\"\r\nname.		\"explomias,解説ロミアス\"\r\nrace.		\"elea\"\r\nclass.		\"archer\"\r\nfilter.		\"/man/cnpc/\"\r\nlevel.		\"100\"\r\nrelation.	\"0\"\r\nsex.		\"-1\"\r\nfixLv.		\"6\"\r\nrare.		\"100\"\r\nspawnType.	\"0\"\r\naiCalm.		\"4\"\r\naiMove.		\"100\"\r\naiDist.		\"1\"\r\naiHeal.		\"640\"\r\naiAct.		\"-1,-1,-2,651,0\"\r\naiActSubFreq.	\"20\"\r\naiActSub.	\"610,610,0,0,0\"\r\nmeleeElem.	\"61,200\"\r\nresist.		\"50,3,51,-5\"\r\nbitOn.		\"5,23\"\r\n\r\n%txt_talk_order\r\n%txtCalm,JP\r\n\r\n%txtAggro,JP\r\n\r\n%txtDead,JP\r\n\r\n%txtKilled,JP\r\n\r\n%txtWelcome,JP\r\n\r\n%txtDialog,JP\r\n\r\n%txtabuse,JP\r\ndefault\r\n\r\n%txtmarriage,JP\r\ndefault\r\n\r\n%txtanata,JP\r\ndefault\r\n\r\n%txtiyayo,JP\r\ndefault\r\n\r\n%txtnakanaka,JP\r\ndefault\r\n\r\n%txtikuyo,JP\r\ndefault\r\n\r\n%txtkiyameru,JP\r\ndefault\r\n\r\n%txtkuyasii,JP\r\ndefault\r\n\r\n%txtjigo,JP\r\ndefault\r\n%txtnoru,JP\r\ndefault\r\n\r\n%txtoriru,JP\r\ndefault\r\n\r\n%txtbiyaku,JP\r\ndefault\r\n\r\n%txttiti,JP\r\ndefault\r\n\r\n%txtsaite,JP\r\ndefault\r\n\r\n%txtsand,JP\r\ndefault\r\n\r\n%txtnikorose,JP\r\ndefault\r\n\r\n%txtkya,JP\r\ndefault\r\n\r\n%txttyohazusu,JP\r\ndefault\r\n\r\n%txtsibaru,JP\r\ndefault\r\n\r\n%txthodoku,JP\r\ndefault\r\n\r\n%txtturusu,JP\r\ndefault\r\n\r\n%txtsorosu,JP\r\ndefault\r\n\r\n%txtsnaguru,JP\r\ndefault\r\n\r\n%txtomiyage,JP\r\ndefault\r\n\r\n%txtyubikubi,JP\r\ndefault\r\n\r\n%txttoriage,JP\r\ndefault\r\n\r\n%txtpbou,JP\r\ndefault\r\n\r\n%txtexthank,JP\r\ndefault\r\n\r\n%txtexhiya,JP\r\ndefault\r\n\r\n%txtgoei,JP\r\ndefault\r\n\r\n%txtyatou,JP\r\ndefault\r\n\r\n%txthihiya,JP\r\ndefault\r\n\r\n%txtumaku,JP\r\ndefault\r\n\r\n%txttikara,JP\r\ndefault\r\n\r\n%txt0free,JP\r\ndefault\r\n\r\n%txtokoto,JP\r\ndefault\r\n\r\n%txtsibui,JP\r\ndefault\r\n\r\n%txtnamaniku,JP\r\ndefault\r\n\r\n%txtkona,JP\r\ndefault\r\n\r\n%txtnamamen,JP\r\ndefault\r\n\r\n%txtheibon,JP\r\ndefault\r\n\r\n%txt1_2,JP\r\ndefault\r\n\r\n%txt3_4,JP\r\ndefault\r\n\r\n%txt5_6,JP\r\ndefault\r\n\r\n%txt7_8,JP\r\ndefault\r\n\r\n%txt9saiko,JP\r\ndefault\r\n\r\n%txtkaradake,JP\r\ndefault\r\n\r\n%txtyanwari,JP\r\ndefault\r\n\r\n%txtkunren,JP\r\ndefault\r\n\r\n%txtonaka,JP\r\ndefault\r\n\r\n%txthinsi,JP\r\ndefault\r\n\r\n%txtkodukuri,JP\r\ndefault\r\n\r\n%txtlayhand,JP\r\ndefault\r\n\r\n%endTxt";
        let result2 = parse_text(text2);
        assert!(result2.is_some());
        let v2 = result2.unwrap();
        assert_eq!(v2.len(), 59);
        assert_eq!(v2.get(58).unwrap().bodies.get(0).unwrap().jp.len(), 2);
    }

    #[test]
    fn test_parse_text_in_comment() {
        let text = "%Elona Custom Npc\r\n\r\nauthor.		\"だれか\"\r\nname.		\"younger brother,おとおと\"\r\nrace.		\"norland\"\r\nclass.		\"predator\"\r\nfilter.		\"/man/cnpc/\"\r\nlevel.		\"1\"\r\nrelation.	\"-1\"\r\nsex.		\"-1\"\r\nfixLv.		\"4\"\r\nrare.		\"100\"\r\nspawnType.	\"0\"\r\naiCalm.		\"4\"\r\naiMove.		\"100\"\r\naiDist.		\"1\"\r\naiHeal.		\"640\"\r\naiAct.		\"-1,-1,-2,651,0\"\r\naiActSubFreq.	\"20\"\r\naiActSub.	\"610,610,0,0,0\"\r\nmeleeElem.	\"61,200\"\r\nresist.		\"50,3,51,-5\"\r\nbitOn.		\"5,23\"\r\n\r\n%txtCalm,JP// 待機中\r\n「わーい」\r\n「おにいちゃん！」\r\n\r\n%txtCalm,EN\r\n\"Weee.\"\r\n\"Brother!\"\r\n\r\n%txtAggro,JP\r\n「てめー」\r\n\r\n%txtAggro,EN\r\n\"Scum!\"\r\n\r\n%txtDead,JP\r\n「ちんだ」\r\n\r\n%txtDead,EN\r\n\"I'm dead.\"\r\n\r\n%txtKilled,JP\r\n「ころしたよー」\r\n\r\n%txtKilled,EN\r\n\"I killed it.\"\r\n\r\n%txtWelcome,JP\r\n「おかえり」\r\n\r\n%txtWelcome,EN\r\n\"Welcome back.\"\r\n\r\n%txtDialog,JP\r\n// コメント\r\nなあに？\r\n（おとおとはあなたをじっとみている)\r\n%txtDialog,EN\r\nHi.\r\nWhat's up?\r\nDude...\r\n%endTxt";
        let result = parse_text(text);
        assert!(result.is_some());
        let v = result.unwrap();
        assert_eq!(v.len(), 6);
        assert_eq!(v.get(0).unwrap().id, "%txtCalm");
        let bodies = &v.get(0).unwrap().bodies;
        assert_eq!(bodies.len(), 1);
        let jp = &bodies.get(0).unwrap().jp;
        assert_eq!(jp.len(), 3);
        assert_eq!(v.get(5).unwrap().bodies.get(0).unwrap().jp.len(), 2);
    }

    #[test]
    fn test_parse_text_addt() {
        let text = "%Elona Custom Npc\r\n\r\nauthor.		\"だれか\"\r\nname.		\"younger brother,おとおと\"\r\nrace.		\"norland\"\r\nclass.		\"predator\"\r\nfilter.		\"/man/cnpc/\"\r\nlevel.		\"1\"\r\nrelation.	\"-1\"\r\nsex.		\"-1\"\r\nfixLv.		\"4\"\r\nrare.		\"100\"\r\nspawnType.	\"0\"\r\naiCalm.		\"4\"\r\naiMove.		\"100\"\r\naiDist.		\"1\"\r\naiHeal.		\"640\"\r\naiAct.		\"-1,-1,-2,651,0\"\r\naiActSubFreq.	\"20\"\r\naiActSub.	\"610,610,0,0,0\"\r\nmeleeElem.	\"61,200\"\r\nresist.		\"50,3,51,-5\"\r\nbitOn.		\"5,23\"\r\n\r\n%txt_talk_order\r\n%txtCalm,JP\r\n「わーい」\r\n「おにいちゃん！」\r\n\r\n$weather Sun\r\n「日差しが気持ちいい～」\r\n「今日もいい天気だね！」\r\n$when year 520\r\n「今年は520年だね！」\r\n「もう3年も経ったんだね！」\r\n$when Morning\r\n「おはよう！」\r\n「朝だよ！」\r\n$when year - 1000\r\n「まだ1000年経ってないよ！」\r\n$when year 1000 -\r\n「1000年経ったよ！」\r\n$when date 20 - 31\r\n「今月も、もう終わりだね！」\r\n$when month 11 - 12\r\n「今年も、もうすぐ終わりだねー」\r\n$when time 00:00\r\n「日付が変わったよー」\r\n$when time 00:00 - 12:00\r\n「午前中だよー」\r\n$where Larna\r\n「ここはラーナだよ！」\r\n$where floor 20 - 24\r\n「随分と深くまで来たねー」\r\n$where floor 25\r\n「ここは最下層だよ！」\r\n$impression Love\r\n「愛してる！」\r\n$impression 200\r\n「大好き！」\r\n$impression 100 -\r\n「好きよー」\r\n$impression - 10\r\n「大嫌い！」\r\n$impression 10 - 20\r\n「嫌い！」\r\n$condition Sick\r\n「具合悪い……」\r\n$PCcondition sleepiness 0\r\n「元気！」\r\n$PCcondition sleepiness 50 -\r\n「もう寝なさーい！」\r\n$PCcondition sleepiness - 15\r\n「元気そう！」\r\n$PCcondition sleepiness 15 - 50\r\n「眠い？」\r\n$karma 0 - 20\r\n「いいこいいこー」\r\n$religion Mani\r\n「マニ様をあがめよー」\r\n$race cupid\r\n「実は天使なんだよー」\r\n$class wizard\r\n「魔法使いだよー」\r\n$comparison PCAge < Age\r\n「自分の方が年上だよー」\r\n$random 1\r\n「ぱんぱかぱーん！」\r\n$anorexia\r\n「ごはんたべたくない……」\r\n%txtCalm,EN\r\n\"Weee.\"\r\n\"Brother!\"\r\n\r\n%txtAggro,JP\r\n「てめー」\r\n\r\n%txtAggro,EN\r\n\"Scum!\"\r\n\r\n%txtDead,JP\r\n「ちんだ」\r\n\r\n%txtDead,EN\r\n\"I'm dead.\"\r\n\r\n%txtKilled,JP\r\n「ころしたよー」\r\n\r\n%txtKilled,EN\r\n\"I killed it.\"\r\n\r\n%txtWelcome,JP\r\n「おかえり」\r\n\r\n%txtWelcome,EN\r\n\"Welcome back.\"\r\n\r\n%txtDialog,JP\r\nなあに？\r\n（おとおとはあなたをじっとみている)\r\n%txtDialog,EN\r\nHi.\r\nWhat's up?\r\nDude...\r\n%endTxt";
        let result = parse_text(text);
        assert!(result.is_some());
        let v = result.unwrap();
        assert_eq!(v.len(), 6);
        assert_eq!(v.get(0).unwrap().id, "%txtCalm");
        let bodies = &v.get(0).unwrap().bodies;
        assert_eq!(bodies.len(), 30);
        let jp = &bodies.get(0).unwrap().jp;
        assert_eq!(jp.len(), 3);
    }
}