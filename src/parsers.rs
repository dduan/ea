use lazy_static::lazy_static;
use regex::Regex;
use std::error;
use std::fmt;

pub mod grouped;
pub mod linear;
pub mod search;

lazy_static! {
    static ref RE_ANSI_CODE: Regex = Regex::new(r#"(\x1b\[[0-9;]*m|\x1b\[[0-9;]*K)"#).unwrap();
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
        write!(f, "{}", "Could not decode input as UTF-8 string")
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
