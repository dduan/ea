use crate::location::Location;
use lazy_static::lazy_static;
use regex::Regex;
use std::option::Option;

pub fn ripgrep(input: &[u8]) -> (Vec<u8>, Vec<Location>) {
    lazy_static! {
        static ref RE_PATH: Regex =
            Regex::new(r#"^(?:\x1b\[[0-9;]*m)*(.+?)(?:\x1b\[[0-9;]*m)+?$"#).unwrap();
        static ref RE_LINE: Regex =
            Regex::new(r#"^(?:\x1b\[[0-9;]*m)*(\d+?)(?:\x1b\[[0-9;]*m)+?:.+?"#).unwrap();
    }

    let mut output = String::new();
    _ = std::fs::write("/tmp/debug", input);
    let input_str = std::str::from_utf8(input).unwrap();

    let mut locations: Vec<Location> = Vec::new();
    let mut file: Option<&str> = None;
    for line in input_str.lines() {
        if let Some(line_match) = RE_PATH.captures(line) {
            file = Some(line_match.get(1).unwrap().as_str());
        } else if let Some(line_match) = RE_LINE.captures(line) {
            if let Ok(line_number) = line_match.get(1).unwrap().as_str().parse::<u64>() {
                if let Some(current_file) = file {
                    let new_location = Location {
                        path: current_file.to_string(),
                        line: Some(line_number),
                        column: None,
                    };

                    locations.push(new_location);
                }

                output = format!(
                    "{}[\x1b[0m\x1b[31m{}\x1b[0m] {}\n",
                    output,
                    locations.len(),
                    line
                );
            }
            continue;
        }
        output = format!("{}{}\n", output, line);
    }

    let output_data: Vec<u8> = output.as_bytes().to_owned();
    (output_data, locations)
}
