use crate::Location;
use lazy_static::lazy_static;
use regex::Regex;
use crate::parsers::{ParseError, search_pattern};

lazy_static! {
    static ref RE_LINE: Regex =
        Regex::new(r#"(\x1b\[[0-9;]*m?)?  File (\x1b\[[0-9;]*m?)*"(?P<path>[^:\n\r]+)"(\x1b\[[0-9;]*m?)*, line (?P<line>\x1b\[[0-9;]*m?)*(\d+)"#).unwrap();
}

pub fn python(input: &[u8]) -> Result<(Vec<u8>, Vec<Location>), ParseError> {
    search_pattern(&RE_LINE, input)
}

#[cfg(test)]
mod tests {
    use super::python;
    use crate::archive::read_from;
    use crate::{parsers::tests::fixture, Location};
    use std::fs;

    #[test]
    fn test_python_output() {
        let input = fs::read(fixture("python3.in.txt")).expect("input file");
        let expected_output = fs::read(fixture("python3.out.txt")).expect("output file");
        let output = python(&input);
        assert_eq!(output.expect("python3 output").0, expected_output);
    }

    #[test]
    fn test_python_locations() {
        let input = fs::read(fixture("python3.in.txt")).expect("input file");
        let expected_locations: Vec<Location> = read_from(&fixture("python3_locations.bin"));
        let output = python(&input);
        assert_eq!(output.expect("python3 output").1, expected_locations);
    }

    #[test]
    fn test_ipython_output() {
        let input = fs::read(fixture("ipython3.in.txt")).expect("input file");
        let expected_output = fs::read(fixture("ipython3.out.txt")).expect("output file");
        let output = python(&input);
        assert_eq!(output.expect("python output").0, expected_output);
    }

    #[test]
    fn test_ipython_locations() {
        let input = fs::read(fixture("ipython3.in.txt")).expect("input file");
        let expected_locations: Vec<Location> = read_from(&fixture("ipython3_locations.bin"));
        let output = python(&input);
        assert_eq!(output.expect("python output").1, expected_locations);
    }
}
