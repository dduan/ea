use crate::parsers::ParseError;
use crate::Location;
use guard::guard;
use lazy_static::lazy_static;
use regex::Regex;

pub fn search(input: &[u8]) -> Result<(Vec<u8>, Vec<Location>), ParseError> {
    lazy_static! {
        static ref RE_LINE: Regex =
            Regex::new(r#"(\r|\n)(\x1b\[[0-9;]*m?)*([^:\n\r]+):(\d+)(?::(\d+))?"#).unwrap();
    }

    let mut output = String::new();
    let mut start: usize = 0;
    let mut locations: Vec<Location> = Vec::new();
    guard!(let Ok(input_str) = std::str::from_utf8(input) else {
        return Result::Err(ParseError::FailedEncoding);
    });
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
    Ok((output_data, locations))
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
