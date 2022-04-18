use crate::location::Location;
use lazy_static::lazy_static;
use regex::Regex;
use std::option::Option;

lazy_static! {
    static ref RE_ANSI_CODE: Regex = Regex::new(r#"\x1b\[[0-9;]*m"#).unwrap();
}

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
        println!("{}", &striped);
        if let Some(line_match) = RE_LINE.captures(&striped) {
            if let Ok(line_number) = line_match.get(1).unwrap().as_str().parse::<u64>() {
                if let Some(current_file) = &file {
                    let column: Option<u64> = line_match.get(2).and_then(|x| x.as_str().parse::<u64>().ok());
                    let new_location = Location {
                        path: current_file.to_string(),
                        line: Some(line_number),
                        column: column,
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
        } else if !striped.is_empty() {
            file = Some(striped);
        }
        output = format!("{}{}\n", output, line);
    }

    let output_data: Vec<u8> = output.as_bytes().to_owned();
    (output_data, locations)
}

pub fn linear(input: &[u8]) -> (Vec<u8>, Vec<Location>) {
    _ = std::fs::write("/tmp/debug", input);
    let mut output = String::new();
    let mut locations: Vec<Location> = Vec::new();
    let input_str = std::str::from_utf8(input).unwrap();
    for line in input_str.lines() {
        if line.is_empty() {
            continue
        }

        let striped = RE_ANSI_CODE.replace_all(&line, "");
        locations.push(Location {
            path: striped.to_string(),
            line: None,
            column: None
        });

        output = format!(
            "{}[\x1b[0m\x1b[31m{}\x1b[0m] {}\n",
            output,
            locations.len(),
            line
        );
    }
    let output_data: Vec<u8> = output.as_bytes().to_owned();
    _ = std::fs::write("/tmp/debug-output", &output_data);
    (output_data, locations)
}
