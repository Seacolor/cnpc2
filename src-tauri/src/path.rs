use std::fs;

use ini::Ini;

pub(crate) fn read_path() -> Option<String> {
     let config_dir = directories::ProjectDirs::from(
            "",
            "",
            "Custom NPC Editor 2"
        )
        .map(|dirs| dirs.config_dir().to_path_buf())
        .unwrap_or_else(|| std::env::current_dir().expect("Cannot access the current directory"));
    let conf_file_path = config_dir.join("conf.ini");
    let config_exists = std::fs::metadata(&conf_file_path).is_ok();
    if !config_exists {
        return None
    }
    let contents = fs::read_to_string(conf_file_path).unwrap();
    if let Ok(conf) = Ini::load_from_str(&contents) {
        let section = conf.section(Some("Elona")).unwrap();
        let dir_path = section.get("path").unwrap();
        return Some(dir_path.to_string())
    }
    return None
}

pub(crate) fn is_file_exists() -> bool {
    if let Some(dir_path) = read_path() {
        let dir_exists = std::fs::metadata(&dir_path).is_ok();
        if !dir_exists {
            return false
        }
        let race_path = format!("{}\\data\\o_race.csv", &dir_path);
        let race_exists = std::fs::metadata(race_path).is_ok();
        if !race_exists {
            return false
        }
        let class_path = format!("{}\\data\\oo_class.csv", &dir_path);
        let class_exists = std::fs::metadata(class_path).is_ok();
        if !class_exists {
            return false
        }
        let skill_path = format!("{}\\data\\o_skill.db", &dir_path);
        let skill_exists = std::fs::metadata(skill_path).is_ok();
        if !skill_exists {
            return false
        }
        return true
    }
    return false
}
