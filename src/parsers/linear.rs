use crate::parsers::{RE_ANSI_CODE, append_line};
use crate::location::Location;

pub fn linear(input: &[u8]) -> (Vec<u8>, Vec<Location>) {
    let mut output = String::new();
    let mut locations: Vec<Location> = Vec::new();
    let input_str = std::str::from_utf8(input).unwrap();
    for line in input_str.lines() {
        if line.is_empty() {
            continue;
        }

        let striped = RE_ANSI_CODE.replace_all(line, "");
        let parts: Vec<&str> = striped.split(':').collect();
        let path = parts[0].to_string();
        let line_number = if parts.len() > 1 {
            parts[1].parse::<u64>().ok()
        } else {
            None
        };
        let column_number = if parts.len() > 2 {
            parts[2].parse::<u64>().ok()
        } else {
            None
        };

        locations.push(Location {
            path,
            line: line_number,
            column: column_number,
        });

        append_line(&mut output, locations.len(), line);
    }
    let output_data: Vec<u8> = output.as_bytes().to_owned();
    (output_data, locations)
}
