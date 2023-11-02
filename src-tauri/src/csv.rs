use std::error::Error;
use std::fs::File;
use std::io::Read;
use csv::{ByteRecord, StringRecord, Reader};

use crate::{MasterData, Race, Class};

type CsvResult<T> = Result<T, Box<dyn Error>>;

fn read_csv<T>(path: &str) -> CsvResult<Vec<T>> where T: MasterData {
    let mut data: Vec<T> = Vec::new();
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let (contents, _, _) = encoding_rs::SHIFT_JIS.decode(&buffer);
    let mut rdr = Reader::from_reader(contents.as_bytes());
    let mut record = ByteRecord::new();
    while rdr.read_byte_record(&mut record)? {
        let str_record = StringRecord::from_byte_record(record.clone())?;
        let name = &str_record[0];
        if name.len() == 0 { continue };
        let id = &str_record[1];
        let playable = &str_record[3];
        data.push(T::new(name, id, playable == "1"));
    }
    Ok(data)
}

pub(crate) fn read_race(dir_path: &str) -> CsvResult<Vec<Race>> {
    let mut races: Vec<Race> = Vec::new();
    let result = read_csv(format!("{}\\data\\o_race.csv", dir_path).as_str());
    if let Ok(_d) = result {
        races = _d
    }
    Ok(races)
}

pub(crate) fn read_class(dir_path: &str) -> CsvResult<Vec<Class>> {
    let mut classes: Vec<Class> = Vec::new();
    let result = read_csv(format!("{}\\data\\oo_class.csv", dir_path).as_str());
    if let Ok(_d) = result {
        classes = _d
    }
    Ok(classes)
}
