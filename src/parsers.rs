use lazy_static::lazy_static;
use regex::Regex;

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

#[cfg(test)]
mod tests {
    use crate::location::Location;
    use super::*;
    use std::fs;
    use std::str;
    use std::path::PathBuf;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref FIXTURES: PathBuf = [
            env!("CARGO_MANIFEST_DIR"),
            "resources",
            "fixtures",
        ].iter().collect();
    }

    fn read_locations(path: PathBuf) -> Vec<Location> {
        let mut locations = Vec::new();
        for line in str::from_utf8(&fs::read(path).unwrap())
            .unwrap()
                .split('\n')
                {
                    let parts: Vec<&str> = line.split(',').collect();
                    let length = parts.len();
                    let location: Location;
                    if length == 1 {
                        location = Location {
                            path: parts[0].to_string(),
                            line: None,
                            column: None,
                        }
                    } else if length == 2 {
                        let line = parts[1].parse::<u64>().unwrap();
                        location = Location {
                            path: parts[0].to_string(),
                            line: Some(line),
                            column: None,
                        }
                    } else {
                        let line = parts[1].parse::<u64>().unwrap();
                        let column = parts[2].parse::<u64>().unwrap();
                        location = Location {
                            path: parts[0].to_string(),
                            line: Some(line),
                            column: Some(column),
                        }
                    }

                    if !location.path.is_empty() {
                        locations.push(location);
                    }
                }

        locations
    }

    fn fixture(path: &str) -> PathBuf {
        let mut result = FIXTURES.clone();
        result.push(path);
        result
    }

#[test]
    fn test_grouped_output() {
        let input = fs::read(fixture("grouped.in.txt")).expect("input file");
        let expected_output = fs::read(fixture("grouped.out.txt")).expect("output file");
        let output = grouped::grouped(&input);
        assert_eq!(output.0, expected_output);
    }

#[test]
    fn test_grouped_locations() {
        let input = fs::read(fixture("grouped.in.txt")).expect("input file");
        let expected_locations: Vec<Location> = read_locations(fixture("grouped_locations.csv"));
        let output = grouped::grouped(&input);
        assert_eq!(output.1, expected_locations);
    }

#[test]
    fn test_grouped_output2() {
        let input = fs::read(fixture("grouped2.in.txt")).expect("input file");
        let expected_output = fs::read(fixture("grouped2.out.txt")).expect("output file");
        let output = grouped::grouped(&input);
        assert_eq!(output.0, expected_output);
    }

#[test]
    fn test_grouped_locations2() {
        let input = fs::read(fixture("grouped2.in.txt")).expect("input file");
        let expected_locations: Vec<Location> = read_locations(fixture("grouped2_locations.csv"));
        let output = grouped::grouped(&input);
        assert_eq!(output.1, expected_locations);
    }

#[test]
    fn test_grouped_output3() {
        let input = fs::read(fixture("grouped3.in.txt")).expect("input file");
        let expected_output = fs::read(fixture("grouped3.out.txt")).expect("output file");
        let output = grouped::grouped(&input).0;
        _ = std::fs::write("/tmp/debug", &output);
        assert_eq!(output, expected_output);
    }

#[test]
    fn test_grouped_locations3() {
        let input = fs::read(fixture("grouped3.in.txt")).expect("input file");
        let expected_locations: Vec<Location> = read_locations(fixture("grouped3_locations.csv"));
        let output = grouped::grouped(&input);
        assert_eq!(output.1, expected_locations);
    }

#[test]
    fn test_linear_colored_output() {
        let input = fs::read(fixture("linear_colored.in.txt")).expect("input file");
        let expected_output = fs::read(fixture("linear_colored.out.txt")).expect("output file");
        let output = linear::linear(&input);
        assert_eq!(output.0, expected_output);
    }

#[test]
    fn test_linear_colored_locations() {
        let input = fs::read(fixture("linear_colored.in.txt")).expect("input file");
        let expected_locations: Vec<Location> = read_locations(fixture("linear_colored_locations.csv"));
        let output = linear::linear(&input);
        assert_eq!(output.1, expected_locations);
    }

#[test]
    fn test_linear_output() {
        let input = fs::read(fixture("linear.in.txt")).expect("input file");
        let expected_output = fs::read(fixture("linear.out.txt")).expect("output file");
        let output = linear::linear(&input);
        assert_eq!(output.0, expected_output);
    }

#[test]
    fn test_linear_locations() {
        let input = fs::read(fixture("linear.in.txt")).expect("input file");
        let expected_locations: Vec<Location> = read_locations(fixture("linear_locations.csv"));
        let output = linear::linear(&input);
        assert_eq!(output.1, expected_locations);
    }

#[test]
    fn test_search_output() {
        let input = fs::read(fixture("search.in.txt")).expect("input file");
        let expected_output = fs::read(fixture("search.out.txt")).expect("output file");
        let output = search::search(&input);
        assert_eq!(output.0, expected_output);
    }

#[test]
    fn test_search_locations() {
        let input = fs::read(fixture("search.in.txt")).expect("input file");
        let expected_locations: Vec<Location> = read_locations(fixture("search_locations.csv"));
        let output = search::search(&input);
        assert_eq!(output.1, expected_locations);
    }
}
