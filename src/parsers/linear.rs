use crate::parsers::{append_line, ParseError, RE_ANSI_CODE};
use crate::Location;
use guard::guard;

pub fn linear(input: &[u8]) -> Result<(Vec<u8>, Vec<Location>), ParseError> {
    let mut output = String::new();
    let mut locations: Vec<Location> = Vec::new();
    guard!(let Ok(input_str) = std::str::from_utf8(input) else {
        return Result::Err(ParseError::FailedEncoding);
    });
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
    Ok((output_data, locations))
}

#[cfg(test)]
mod tests {
    use super::linear;
    use crate::archive::read_from;
    use crate::parsers::tests::fixture;
    use crate::Location;
    use std::fs;

    #[test]
    fn test_linear_colored_output() {
        let input = fs::read(fixture("linear_colored.in.txt")).expect("input file");
        let expected_output = fs::read(fixture("linear_colored.out.txt")).expect("output file");
        let output = linear(&input);
        assert_eq!(output.expect("linear output").0, expected_output);
    }

    #[test]
    fn test_linear_colored_locations() {
        let input = fs::read(fixture("linear_colored.in.txt")).expect("input file");
        let expected_locations: Vec<Location> = read_from(&fixture("linear_colored_locations.bin"));
        let output = linear(&input);
        assert_eq!(output.expect("linear output").1, expected_locations);
    }

    #[test]
    fn test_linear_output() {
        let input = fs::read(fixture("linear.in.txt")).expect("input file");
        let expected_output = fs::read(fixture("linear.out.txt")).expect("output file");
        let output = linear(&input);
        assert_eq!(output.expect("linear output").0, expected_output);
    }

    #[test]
    fn test_linear_locations() {
        let input = fs::read(fixture("linear.in.txt")).expect("input file");
        let expected_locations: Vec<Location> = read_from(&fixture("linear_locations.bin"));
        let output = linear(&input);
        assert_eq!(output.expect("linear output").1, expected_locations);
    }
}
