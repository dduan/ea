use crate::parsers::{RE_ANSI_CODE, append_line};
use ea_command::Location;
use lazy_static::lazy_static;
use regex::Regex;

pub fn grouped(input: &[u8]) -> (Vec<u8>, Vec<Location>) {
    lazy_static! {
        static ref RE_LINE: Regex = Regex::new(r#"^(\d+):(?:(\d+):)?.+?"#).unwrap();
    }

    let mut output = String::new();
    let input_str = std::str::from_utf8(input).unwrap();

    let mut locations: Vec<Location> = Vec::new();
    let mut file: Option<String> = None;
    let mut striped: String;
    for line in input_str.lines() {
        striped = RE_ANSI_CODE.replace_all(line, "").to_string();
        if let Some(line_match) = RE_LINE.captures(&striped) {
            if let Ok(line_number) = line_match.get(1).unwrap().as_str().parse::<u64>() {
                if let Some(current_file) = &file {
                    let column: Option<u64> = line_match
                        .get(2)
                        .and_then(|x| x.as_str().parse::<u64>().ok());
                    let new_location = Location {
                        path: current_file.to_string(),
                        line: Some(line_number),
                        column,
                    };

                    locations.push(new_location);
                }

                append_line(&mut output, locations.len(), line);
            }
            continue;
        } else if !striped.is_empty() {
            file = Some(striped);
        }
        output = format!("{}{}\n", output, line);
    }

    let output_data: Vec<u8> = output.as_bytes().to_owned();
    (output_data, locations)
}
