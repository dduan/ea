use lazy_static::lazy_static;
use regex::Regex;
use std::{fmt, error, fs};
use guard::guard;
use crate::Location;

pub mod grouped;
pub mod linear;
pub mod search;
pub mod rust;

lazy_static! {
    static ref RE_ANSI_CODE: Regex = Regex::new(r#"(\x1b\[[0-9;]*m|\x1b\[[0-9;]*K)"#).unwrap();
}


pub fn search_pattern(pattern: &Regex, input: &[u8]) -> Result<(Vec<u8>, Vec<Location>), ParseError> {
    let mut output = String::new();
    let mut start: usize = 0;
    let mut locations: Vec<Location> = Vec::new();
    guard!(let Ok(input_str) = std::str::from_utf8(input) else {
        return Result::Err(ParseError::FailedEncoding);
    });
    for captures in pattern.captures_iter(input_str) {
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

    _ = fs::write("/tmp/out", &output);

    let output_data: Vec<u8> = output.as_bytes().to_owned();
    Ok((output_data, locations))
}

fn append_line(output: &mut String, location_number: usize, line: &str) {
    *output = format!(
        "{}[\x1b[0m\x1b[31m{}\x1b[0m] {}\n",
        output, location_number, line
    );
}

#[derive(Debug)]
pub enum ParseError {
    FailedEncoding,
}

impl error::Error for ParseError {}
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not decode input as UTF-8 string")
    }
}

#[cfg(test)]
pub mod tests {
    use lazy_static::lazy_static;
    use std::path::PathBuf;
    use std::str;

    lazy_static! {
        static ref FIXTURES: PathBuf = [
            env!("CARGO_MANIFEST_DIR"),
            "resources",
            "fixtures",
            "parsers"
        ]
        .iter()
        .collect();
    }

    pub fn fixture(path: &str) -> PathBuf {
        let mut result = FIXTURES.clone();
        result.push(path);
        result
    }
}
