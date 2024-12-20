use crate::mods::extract;
use regex::Regex;
use std::fs::File;
use std::io::{self, Read};

const FIRST_QUERY: &str = "北海道";
const FILE_PATH: &str = "./db/0.html";

pub fn title40() -> bool {
    let title = extract::extract_element("title", FILE_PATH);
    let title = title[0].to_string();
    title.chars().count() <= 40
}

pub fn is_ssl_enabled(file_path: &str) -> io::Result<bool> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let re = Regex::new(r#"https://"#).unwrap();
    Ok(re.is_match(&contents))
}
