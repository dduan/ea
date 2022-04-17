use crate::location::Location;
use lazy_static::lazy_static;
use regex::Regex;
use std::option::Option;

pub fn ripgrep(input: &[u8]) -> (Vec<u8>, Vec<Location>) {
    lazy_static! {
        static ref RE_PATH: Regex = Regex::new(r#"^\x1B\[0m\x1B\[\d+?m(.+?)\x1B\[0m$"#).unwrap();
        static ref RE_LINE: Regex =
            Regex::new(r#"^\x1B\[0m\x1B\[\d+?m(\d+?)\x1B\[0m:.+?"#).unwrap();
    }

    let input_str = std::str::from_utf8(input).unwrap();

    let mut locations: Vec<Location> = Vec::new();
    let mut file: Option<&str> = None;
    for line in input_str.split('\n') {
        if let Some(line_match) = RE_PATH.captures(line) {
            file = Some(line_match.get(1).unwrap().as_str());
            continue;
        }

        if let Some(line_match) = RE_LINE.captures(line) {
            if let Ok(line_number) = line_match.get(1).unwrap().as_str().parse::<u64>() {
                if let Some(current_file) = file {
                    locations.push(Location {
                        path: current_file.to_string(),
                        line: Some(line_number),
                        column: None,
                    })
                }
            }
            continue;
        }
    }

    (vec![], locations)
}
