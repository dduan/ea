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

    let mut output: Vec<String> = vec![];
    let input_str = std::str::from_utf8(input).unwrap();

    let mut locations: Vec<Location> = Vec::new();
    let mut file: Option<&str> = None;
    for line in input_str.split('\n') {
        if let Some(line_match) = RE_PATH.captures(line) {
            file = Some(line_match.get(1).unwrap().as_str());
        } else if let Some(line_match) = RE_LINE.captures(line) {
            if let Ok(line_number) = line_match.get(1).unwrap().as_str().parse::<u64>() {
                if let Some(current_file) = file {
                    locations.push(Location {
                        path: current_file.to_string(),
                        line: Some(line_number),
                        column: None,
                    })
                }

                output.push(format!(
                    "[\x1B[0m\x1B[31m{}\x1B[0m] {}",
                    locations.len(),
                    line
                ));
            }
            continue;
        }
        output.push(line.to_string());
    }

    let output_data: Vec<u8> = output.join("\n").as_bytes().to_owned();
    _ = std::fs::write("/tmp/test_output", &output_data);
    (output_data, locations)
}
