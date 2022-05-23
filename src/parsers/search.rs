use crate::Location;
use lazy_static::lazy_static;
use regex::Regex;
use crate::parsers::{ParseError, search_pattern};

lazy_static! {
    static ref RE_LINE: Regex =
        Regex::new(r#"(\r|\n)(\x1b\[[0-9;]*m?)*([^:\n\r]+):(\d+)(?::(\d+))?"#).unwrap();
}

pub fn search(input: &[u8]) -> Result<(Vec<u8>, Vec<Location>), ParseError> {
    search_pattern(&RE_LINE, input)
}

#[cfg(test)]
mod tests {
    use super::search;
    use crate::archive::read_from;
    use crate::parsers::tests::fixture;
    use crate::Location;
    use std::fs;

    #[test]
    fn test_search_output() {
        let input = fs::read(fixture("search.in.txt")).expect("input file");
        let expected_output = fs::read(fixture("search.out.txt")).expect("output file");
        let output = search(&input);
        assert_eq!(output.expect("search output").0, expected_output);
    }

    #[test]
    fn test_search_locations() {
        let input = fs::read(fixture("search.in.txt")).expect("input file");
        let expected_locations: Vec<Location> = read_from(&fixture("search_locations.bin"));
        let output = search(&input);
        assert_eq!(output.expect("search output").1, expected_locations);
    }
}
