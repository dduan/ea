use crate::parsers::{append_line, RE_ANSI_CODE};
use ea_command::Location;
use lazy_static::lazy_static;
use regex::Regex;

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
                    let column: Option<u64> = line_match
                        .get(2)
                        .and_then(|x| x.as_str().parse::<u64>().ok());
                    let new_location = Location {
                        path: current_file.to_string(),
                        line: Some(line_number),
                        column,
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

#[cfg(test)]
mod tests {
    use super::grouped;
    use crate::archive::read_from;
    use crate::parsers::tests::fixture;
    use ea_command::Location;
    use std::fs;

    #[test]
    fn test_grouped_output() {
        let input = fs::read(fixture("grouped.in.txt")).expect("input file");
        let expected_output = fs::read(fixture("grouped.out.txt")).expect("output file");
        let output = grouped(&input);
        assert_eq!(output.0, expected_output);
    }

    #[test]
    fn test_grouped_locations() {
        let input = fs::read(fixture("grouped.in.txt")).expect("input file");
        let expected_locations: Vec<Location> = read_from(&fixture("grouped_locations.bin"));
        let output = grouped(&input);
        assert_eq!(output.1, expected_locations);
    }

    #[test]
    fn test_grouped_output2() {
        let input = fs::read(fixture("grouped2.in.txt")).expect("input file");
        let expected_output = fs::read(fixture("grouped2.out.txt")).expect("output file");
        let output = grouped(&input);
        assert_eq!(output.0, expected_output);
    }

    #[test]
    fn test_grouped_locations2() {
        let input = fs::read(fixture("grouped2.in.txt")).expect("input file");
        let expected_locations: Vec<Location> = read_from(&fixture("grouped2_locations.bin"));
        let output = grouped(&input);
        assert_eq!(output.1, expected_locations);
    }

    #[test]
    fn test_grouped_output3() {
        let input = fs::read(fixture("grouped3.in.txt")).expect("input file");
        let expected_output = fs::read(fixture("grouped3.out.txt")).expect("output file");
        let output = grouped(&input).0;
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_grouped_locations3() {
        let input = fs::read(fixture("grouped3.in.txt")).expect("input file");
        let expected_locations: Vec<Location> = read_from(&fixture("grouped3_locations.bin"));
        let output = grouped(&input);
        assert_eq!(output.1, expected_locations);
    }
}
