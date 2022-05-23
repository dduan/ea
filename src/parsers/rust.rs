use crate::Location;
use lazy_static::lazy_static;
use regex::Regex;
use crate::parsers::{ParseError, search_pattern};

lazy_static! {
    static ref RE_LINE: Regex =
        Regex::new(r#"  (\x1b\[[0-9;]*m?)*--> (\x1b\[[0-9;]*m?)*([^:\n\r]+):(\d+)(?::(\d+))?"#).unwrap();
}

pub fn rust(input: &[u8]) -> Result<(Vec<u8>, Vec<Location>), ParseError> {
    search_pattern(&RE_LINE, input)
}

#[cfg(test)]
mod tests {
    use super::rust;
    use crate::archive::read_from;
    use crate::parsers::tests::fixture;
    use crate::Location;
    use std::fs;

    #[test]
    fn test_rust_output() {
        let input = fs::read(fixture("rust.in.txt")).expect("input file");
        let expected_output = fs::read(fixture("rust.out.txt")).expect("output file");
        let output = rust(&input);
        assert_eq!(output.expect("rust output").0, expected_output);
    }

    #[test]
    fn test_rust_locations() {
        let input = fs::read(fixture("rust.in.txt")).expect("input file");
        let expected_locations: Vec<Location> = read_from(&fixture("rust_locations.bin"));
        let output = rust(&input);
        assert_eq!(output.expect("search output").1, expected_locations);
    }
}
