use crate::location::Location;
use lazy_static::lazy_static;
use regex::Regex;

pub fn search(input: &[u8]) -> (Vec<u8>, Vec<Location>) {
    lazy_static! {
        static ref RE_LINE: Regex =
            Regex::new(r#"(\r|\n)(\x1b\[[0-9;]*m?)*([^:\n\r]+):(\d+)(?::(\d+))?"#).unwrap();
    }

    let mut output = String::new();
    let mut start: usize = 0;
    let mut locations: Vec<Location> = Vec::new();
    let input_str = std::str::from_utf8(input).unwrap();
    for captures in RE_LINE.captures_iter(input_str) {
        let path_match = captures.get(3).unwrap();
        let line = captures.get(4).unwrap().as_str().parse::<u64>().unwrap();
        let column = captures.get(5).and_then(|x| x.as_str().parse::<u64>().ok());
        locations.push(Location {
            path: path_match.as_str().to_string(),
            line: Some(line),
            column,
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
