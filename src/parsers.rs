use crate::location::Location;
use lazy_static::lazy_static;
use regex::Regex;
use std::option::Option;

lazy_static! {
    static ref RE_ANSI_CODE: Regex = Regex::new(r#"\x1b\[[0-9;K]*m?"#).unwrap();
}

fn append_line(output: &mut String, location_number: usize, line: &str) {
    *output = format!(
        //"{}\x1b[0m[\x1b[0m\x1b[31m{}\x1b[0m] {}\n",
        "{}[\x1b[0m\x1b[31m{}\x1b[0m] {}\n",
        output,
        location_number,
        line
    );
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

pub fn linear(input: &[u8]) -> (Vec<u8>, Vec<Location>) {
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

        append_line(&mut output, locations.len(), line);
    }
    let output_data: Vec<u8> = output.as_bytes().to_owned();
    (output_data, locations)
}

pub fn search(input: &[u8]) -> (Vec<u8>, Vec<Location>) {
    lazy_static! {
        static ref RE_LINE: Regex = Regex::new(r#"(\r|\n)(\x1b\[[0-9;]*m?)*([^:\n\r]+):(\d+)(?::(\d+))?"#)
            .unwrap();
    }

    let mut output = String::new();
    let mut start: usize = 0;
    let mut locations: Vec<Location> = Vec::new();
    let input_str = std::str::from_utf8(input).unwrap();
    for captures in RE_LINE.captures_iter(&input_str) {
        let path_match = captures.get(3).unwrap();
        let line = captures.get(4).unwrap().as_str().parse::<u64>().unwrap();
        let column = captures.get(5).and_then(|x| x.as_str().parse::<u64>().ok());
        locations.push(Location {
            path: path_match.as_str().to_string(),
            line: Some(line),
            column: column,
        });
        output = format!(
            "{}{}[\x1b[0m\x1b[31m{}\x1b[0m] ",
            output,
            &input_str[start..path_match.start()],
            locations.len()
        );
        start = path_match.start();
    }

    output = format!("{}{}", output, &input_str[start..]);

    let output_data: Vec<u8> = output.as_bytes().to_owned();
    (output_data, locations)
}
