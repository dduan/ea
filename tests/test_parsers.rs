use ea::location::Location;
use ea::parsers;
use std::fs;
use std::str;

fn read_locations(path: &str) -> Vec<Location> {
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

#[test]
fn test_grouped_output() {
    let input = fs::read("tests/fixtures/grouped.in.txt").expect("input file");
    let expected_output = fs::read("tests/fixtures/grouped.out.txt").expect("output file");
    let output = parsers::grouped(&input);
    assert_eq!(output.0, expected_output);
}

#[test]
fn test_grouped_locations() {
    let input = fs::read("tests/fixtures/grouped.in.txt").expect("input file");
    let expected_locations: Vec<Location> = read_locations("tests/fixtures/grouped_locations.csv");
    let output = parsers::grouped(&input);
    assert_eq!(output.1, expected_locations);
}

#[test]
fn test_grouped_output2() {
    let input = fs::read("tests/fixtures/grouped2.in.txt").expect("input file");
    let expected_output = fs::read("tests/fixtures/grouped2.out.txt").expect("output file");
    let output = parsers::grouped(&input);
    assert_eq!(output.0, expected_output);
}

#[test]
fn test_grouped_locations2() {
    let input = fs::read("tests/fixtures/grouped2.in.txt").expect("input file");
    let expected_locations: Vec<Location> = read_locations("tests/fixtures/grouped2_locations.csv");
    let output = parsers::grouped(&input);
    assert_eq!(output.1, expected_locations);
}

#[test]
fn test_grouped_output3() {
    let input = fs::read("tests/fixtures/grouped3.in.txt").expect("input file");
    let expected_output = fs::read("tests/fixtures/grouped3.out.txt").expect("output file");
    let output = parsers::grouped(&input).0;
    _ = std::fs::write("/tmp/debug", &output);
    assert_eq!(output, expected_output);
}

#[test]
fn test_grouped_locations3() {
    let input = fs::read("tests/fixtures/grouped3.in.txt").expect("input file");
    let expected_locations: Vec<Location> = read_locations("tests/fixtures/grouped3_locations.csv");
    let output = parsers::grouped(&input);
    assert_eq!(output.1, expected_locations);
}

#[test]
fn test_linear_colored_output() {
    let input = fs::read("tests/fixtures/linear_colored.in.txt").expect("input file");
    let expected_output = fs::read("tests/fixtures/linear_colored.out.txt").expect("output file");
    let output = parsers::linear(&input);
    assert_eq!(output.0, expected_output);
}

#[test]
fn test_linear_colored_locations() {
    let input = fs::read("tests/fixtures/linear_colored.in.txt").expect("input file");
    let expected_locations: Vec<Location> = read_locations("tests/fixtures/linear_colored_locations.csv");
    let output = parsers::linear(&input);
    assert_eq!(output.1, expected_locations);
}

#[test]
fn test_linear_output() {
    let input = fs::read("tests/fixtures/linear.in.txt").expect("input file");
    let expected_output = fs::read("tests/fixtures/linear.out.txt").expect("output file");
    let output = parsers::linear(&input);
    assert_eq!(output.0, expected_output);
}

#[test]
fn test_linear_locations() {
    let input = fs::read("tests/fixtures/linear.in.txt").expect("input file");
    let expected_locations: Vec<Location> = read_locations("tests/fixtures/linear_locations.csv");
    let output = parsers::linear(&input);
    assert_eq!(output.1, expected_locations);
}
