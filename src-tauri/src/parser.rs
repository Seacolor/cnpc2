use regex::Regex;

use crate::{Character, UserResist, UserRace, UserSkill, UserTrait, Figure, UserClass, InitEquip, UserText, UserTextBody, UserTalk};
struct Equip {
    id: String,
    custom_item_id: String,
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
    let regex = Regex::new("(?sR)%txt_ucnpc_ev_b\r\n(.*?)\r\n%txt_ucnpc_ev_e").unwrap();
    if let Some(mat) = regex.find(text) {
        let parsed_text = regex.replace(text, "").to_string();
        return Some(
            Talk {
                jp: mat.as_str().to_string(),
                parsed_text: parsed_text,
            }
        );
    }
    return None;
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
    let regex1 = Regex::new(r"(?im)%.*?,jp").unwrap();
    let tags: Vec<&str> = regex1.find_iter(text).map(|m| m.as_str()).collect();
    for tag in &tags {
        let re = format!("(?sR){}\r\n(.*?)\r\n%", tag);
        let regex2 = Regex::new(&re).unwrap();
        if let Some(caps1) = regex2.captures(text) {
            let mut bodies: Vec<UserTextBody> = Vec::new();
            let v = caps1[1]
                .split("\r\n")
                .collect::<Vec<_>>();
            let mut body = UserTextBody{
                case_value: "".into(),
                values: Vec::new(),
                jp: Vec::new(),
            };
            for s in v {
                let regex3 = Regex::new(r"(?m)^%?(\$.*?) (.*?)$").unwrap();
                if let Some(caps2) = regex3.captures(s) {
                    bodies.push(body);
                    let mut case_value = "".into();
                    let mut values: Vec<String> = Vec::new();
                    match &caps2[1] {
                        "$when" => {
                            let vv = caps2[2]
                                .split(" ")
                                .collect::<Vec<_>>();
                            match vv[0] {
                                "time" => {
                                    match vv.len() {
                                        2 => {
                                            let vvv = vv[1]
                                                .split(":")
                                                .collect::<Vec<_>>();
                                            case_value = format!("{} {} {}:{}", &caps2[1], vv[0], "{0}", "{1}");
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
                                            case_value = format!("{} {} {}:{} {} {}:{}", &caps2[1], vv[0], "{0}", "{1}", vv[2], "{2}", "{3}");
                                            values.push(vvv1[0].to_string());
                                            values.push(vvv1[1].to_string());
                                            values.push(vvv2[0].to_string());
                                            values.push(vvv2[1].to_string());
                                        }
                                        _ => {
                                            case_value = format!("{} {}", &caps2[1], vv[0]);
                                        }
                                    }
                                }
                                _ => {
                                    match vv.len() {
                                        2 => {
                                            case_value = format!("{} {} {}", &caps2[1], vv[0], "{0}");
                                            values.push(vv[1].to_string());
                                        }
                                        3 => {
                                            if vv[1] == "-" {
                                                case_value = format!("{} {} {} {}", &caps2[1], vv[0], vv[1], "{0}");
                                                values.push(vv[2].to_string());
                                            } else {
                                                case_value = format!("{} {} {} {}", &caps2[1], vv[0], "{0}", vv[2]);
                                                values.push(vv[1].to_string());
                                            }
                                        }
                                        4 => {
                                            case_value = format!("{} {} {} {} {}", &caps2[1], vv[0], "{0}", vv[2], "{1}");
                                            values.push(vv[1].to_string());
                                            values.push(vv[3].to_string());
                                        }
                                        5 => {
                                            case_value = format!("{} {} {} {} {}", &caps2[1], vv[0], "{0}", vv[2], "{1}");
                                            values.push(vv[1].to_string());
                                            values.push(vv[3].to_string());
                                        }
                                        _ => {
                                            case_value = format!("{} {}", &caps2[1], vv[0]);
                                        }
                                    }
                                }
                            }
                        }
                        "$where" => {
                            let vv = caps2[2]
                                .split(" ")
                                .collect::<Vec<_>>();
                            match vv[0] {
                                "floor" => {
                                    match vv.len() {
                                        2 => {
                                            case_value = format!("{} {} {}", &caps2[1], vv[0], "{0}");
                                            values.push(vv[1].to_string());
                                        }
                                        3 => {
                                            if vv[1] == "-" {
                                                case_value = format!("{} {} {} {}", &caps2[1], vv[0], vv[1], "{0}");
                                                values.push(vv[2].to_string());
                                            } else {
                                                case_value = format!("{} {} {} {}", &caps2[1], vv[0], "{0}", vv[2]);
                                                values.push(vv[1].to_string());
                                            }
                                        }
                                        4 => {
                                            case_value = format!("{} {} {} {} {}", &caps2[1], vv[0], "{0}", vv[2], "{1}");
                                            values.push(vv[1].to_string());
                                            values.push(vv[3].to_string());
                                        }
                                        _ => {
                                            case_value = format!("{} {}", &caps2[1], vv[0]);
                                        }
                                    }
                                }
                                _ => {
                                    case_value = format!("{} {}", &caps2[1], vv[0]);
                                }
                            }
                        }
                        "$weather" => {
                            case_value = format!("{} {}", &caps2[1], &caps2[2]);
                        }
                        "$impression" => {
                            let vv = caps2[2]
                                .split(" ")
                                .collect::<Vec<_>>();
                            match vv.len() {
                                2 => {
                                    if vv[0] == "-" {
                                        case_value = format!("{} {} {}", &caps2[1], vv[0], "{0}");
                                        values.push(vv[1].to_string());
                                    } else {
                                        case_value = format!("{} {} {}", &caps2[1], "{0}", vv[1]);
                                        values.push(vv[0].to_string());
                                    }
                                }
                                3 => {
                                    case_value = format!("{} {} {} {}", &caps2[1], "{0}", vv[1], "{1}");
                                    values.push(vv[0].to_string());
                                    values.push(vv[2].to_string());
                                }
                                _ => {
                                    let regex4 = Regex::new(r"(?m)^(\d+)$").unwrap();
                                    if regex4.is_match(vv[0]) {
                                        case_value = format!("{} {}", &caps2[1], "{0}");
                                        values.push(vv[0].to_string());
                                    } else {
                                        case_value = format!("{} {}", &caps2[1], vv[0]);
                                    }
                                }
                            }
                        }
                        "$condition" => {
                            case_value = format!("{} {}", &caps2[1], &caps2[2]);
                        }
                        "$PCcondition" => {
                            let vv = caps2[2]
                                .split(" ")
                                .collect::<Vec<_>>();
                            match vv[0] {
                                "sleepiness" => {
                                    match vv.len() {
                                        2 => {
                                            case_value = format!("{} {} {}", &caps2[1], vv[0], "{0}");
                                            values.push(vv[1].to_string());
                                        }
                                        3 => {
                                            if vv[1] == "-" {
                                                case_value = format!("{} {} {} {}", &caps2[1], vv[0], vv[1], "{0}");
                                                values.push(vv[2].to_string());
                                            } else {
                                                case_value = format!("{} {} {} {}", &caps2[1], vv[0], "{0}", vv[2]);
                                                values.push(vv[1].to_string());
                                            }
                                        }
                                        4 => {
                                            case_value = format!("{} {} {} {} {}", &caps2[1], vv[0], "{0}", vv[2], "{1}");
                                            values.push(vv[1].to_string());
                                            values.push(vv[3].to_string());
                                        }
                                        _ => {
                                            case_value = format!("{} {}", &caps2[1], vv[0]);
                                        }
                                    }
                                }
                                _ => {
                                    case_value = format!("{} {}", &caps2[1], vv[0]);
                                }
                            }
                        }
                        "$karma" => {
                            let vv = caps2[2]
                                .split(" ")
                                .collect::<Vec<_>>();
                            match vv.len() {
                                2 => {
                                    if vv[0] == "-" {
                                        case_value = format!("{} {} {}", &caps2[1], vv[0], "{0}");
                                        values.push(vv[1].to_string());
                                    } else {
                                        case_value = format!("{} {} {}", &caps2[1], "{0}", vv[1]);
                                        values.push(vv[0].to_string());
                                    }
                                }
                                3 => {
                                    case_value = format!("{} {} {} {}", &caps2[1], "{0}", vv[1], "{1}");
                                    values.push(vv[0].to_string());
                                    values.push(vv[2].to_string());
                                }
                                _ => {
                                    case_value = format!("{} {}", &caps2[1], "{0}");
                                    values.push(vv[0].to_string());
                                }
                            }
                        }
                        "$cash" => {
                            let vv = caps2[2]
                                .split(" ")
                                .collect::<Vec<_>>();
                            match vv.len() {
                                2 => {
                                    if vv[0] == "-" {
                                        case_value = format!("{} {} {}", &caps2[1], vv[0], "{0}");
                                        values.push(vv[1].to_string());
                                    } else {
                                        case_value = format!("{} {} {}", &caps2[1], "{0}", vv[1]);
                                        values.push(vv[0].to_string());
                                    }
                                }
                                3 => {
                                    case_value = format!("{} {} {} {}", &caps2[1], "{0}", vv[1], "{1}");
                                    values.push(vv[0].to_string());
                                    values.push(vv[2].to_string());
                                }
                                _ => {
                                    case_value = format!("{} {}", &caps2[1], "{0}");
                                    values.push(vv[0].to_string());
                                }
                            }
                        }
                        "$PCcash" => {
                            let vv = caps2[2]
                                .split(" ")
                                .collect::<Vec<_>>();
                            match vv.len() {
                                2 => {
                                    if vv[0] == "-" {
                                        case_value = format!("{} {} {}", &caps2[1], vv[0], "{0}");
                                        values.push(vv[1].to_string());
                                    } else {
                                        case_value = format!("{} {} {}", &caps2[1], "{0}", vv[1]);
                                        values.push(vv[0].to_string());
                                    }
                                }
                                3 => {
                                    case_value = format!("{} {} {} {}", &caps2[1], "{0}", vv[1], "{1}");
                                    values.push(vv[0].to_string());
                                    values.push(vv[2].to_string());
                                }
                                _ => {
                                    case_value = format!("{} {}", &caps2[1], "{0}");
                                    values.push(vv[0].to_string());
                                }
                            }
                        }
                        "$fame" => {
                            let vv = caps2[2]
                                .split(" ")
                                .collect::<Vec<_>>();
                            match vv.len() {
                                2 => {
                                    if vv[0] == "-" {
                                        case_value = format!("{} {} {}", &caps2[1], vv[0], "{0}");
                                        values.push(vv[1].to_string());
                                    } else {
                                        case_value = format!("{} {} {}", &caps2[1], "{0}", vv[1]);
                                        values.push(vv[0].to_string());
                                    }
                                }
                                3 => {
                                    case_value = format!("{} {} {} {}", &caps2[1], "{0}", vv[1], "{1}");
                                    values.push(vv[0].to_string());
                                    values.push(vv[2].to_string());
                                }
                                _ => {
                                    case_value = format!("{} {}", &caps2[1], "{0}");
                                    values.push(vv[0].to_string());
                                }
                            }
                        }
                        "$PCfame" => {
                            let vv = caps2[2]
                                .split(" ")
                                .collect::<Vec<_>>();
                            match vv.len() {
                                2 => {
                                    if vv[0] == "-" {
                                        case_value = format!("{} {} {}", &caps2[1], vv[0], "{0}");
                                        values.push(vv[1].to_string());
                                    } else {
                                        case_value = format!("{} {} {}", &caps2[1], "{0}", vv[1]);
                                        values.push(vv[0].to_string());
                                    }
                                }
                                3 => {
                                    case_value = format!("{} {} {} {}", &caps2[1], "{0}", vv[1], "{1}");
                                    values.push(vv[0].to_string());
                                    values.push(vv[2].to_string());
                                }
                                _ => {
                                    case_value = format!("{} {}", &caps2[1], "{0}");
                                    values.push(vv[0].to_string());
                                }
                            }
                        }
                        "$religion" => {
                            case_value = format!("{} {}", &caps2[1], &caps2[2]);
                        }
                        "$PCreligion" => {
                            case_value = format!("{} {}", &caps2[1], &caps2[2]);
                        }
                        "$action" => {
                            case_value = format!("{} {}", &caps2[1], &caps2[2]);
                        }
                        "$PCaction" => {
                            case_value = format!("{} {}", &caps2[1], &caps2[2]);
                        }
                        "$sex" => {
                            case_value = format!("{} {}", &caps2[1], &caps2[2]);
                        }
                        "$PCsex" => {
                            case_value = format!("{} {}", &caps2[1], &caps2[2]);
                        }
                        "$race" => {
                            case_value = format!("{} {}", &caps2[1], "{0}");
                            values.push(caps2[2].to_string());
                        }
                        "$PCrace" => {
                            case_value = format!("{} {}", &caps2[1], "{0}");
                            values.push(caps2[2].to_string());
                        }
                        "$class" => {
                            case_value = format!("{} {}", &caps2[1], "{0}");
                            values.push(caps2[2].to_string());
                        }
                        "$PCclass" => {
                            case_value = format!("{} {}", &caps2[1], "{0}");
                            values.push(caps2[2].to_string());
                        }
                        "$comparison" => {
                            case_value = format!("{} {}", &caps2[1], "{0}");
                            values.push(caps2[2].to_string());
                        }
                        "$random" => {
                            case_value = format!("{} {}", &caps2[1], "{0}");
                            values.push(caps2[2].to_string());
                        }
                        _ => {}
                    }
                    body = UserTextBody {
                        case_value: case_value,
                        values: values,
                        jp: Vec::new(),
                    };
                } else {
                    let regex4 = Regex::new(r"(?m)^%?(\$.*?)$").unwrap();
                    if let Some(caps3) = regex4.captures(s) {
                        bodies.push(body);
                        let mut case_value = "".into();
                        match &caps3[1] {
                            "$pet" => {
                                case_value = format!("{}", &caps3[1]);
                            }
                            "$married" => {
                                case_value = format!("{}", &caps3[1]);
                            }
                            "$stethoscope" => {
                                case_value = format!("{}", &caps3[1]);
                            }
                            "$tied" => {
                                case_value = format!("{}", &caps3[1]);
                            }
                            "$ridden" => {
                                case_value = format!("{}", &caps3[1]);
                            }
                            "$agreement" => {
                                case_value = format!("{}", &caps3[1]);
                            }
                            "$anorexia" => {
                                case_value = format!("{}", &caps3[1]);
                            }
                            "$layhand" => {
                                case_value = format!("{}", &caps3[1]);
                            }
                            "$incognito" => {
                                case_value = format!("{}", &caps3[1]);
                            }
                            _ => {}
                        }
                        body = UserTextBody {
                            case_value: case_value,
                            values: Vec::new(),
                            jp: Vec::new(),
                        };
                    } else {
                        body.jp.push(s.to_string());
                    }
                }
            }
            bodies.push(body);
            let id = parse_id(tag);
            txt.push(
                UserText {
                    id: id.id,
                    value: id.value,
                    bodies: bodies,
                }
            )
        }
    }
    Some(txt)
}

fn parse_text_talk_order(text: &str) -> bool {
    let regex = Regex::new(r"(?mR)^%txt_talk_order$").unwrap();
    return regex.is_match(text);
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
