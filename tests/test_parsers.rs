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
    let input = fs::read("tests/fixtures/grouped.in").expect("input file");
    let expected_output = fs::read("tests/fixtures/grouped.out").expect("output file");
    let output = parsers::grouped(&input);
    assert_eq!(output.0, expected_output);
}

#[test]
fn test_grouped_locations() {
    let input = fs::read("tests/fixtures/grouped.in").expect("input file");
    let output = parsers::grouped(&input);
    let expected_locations: Vec<Location> = read_locations("tests/fixtures/grouped_locations.csv");
    assert_eq!(output.1, expected_locations);
}

#[test]
fn test_grouped_output2() {
    let input = fs::read("tests/fixtures/grouped2.in").expect("input file");
    let expected_output = fs::read("tests/fixtures/grouped2.out").expect("output file");
    let output = parsers::grouped(&input);
    assert_eq!(output.0, expected_output);
}

#[test]
fn test_grouped_locations2() {
    let input = fs::read("tests/fixtures/grouped2.in").expect("input file");
    let output = parsers::grouped(&input);
    let expected_locations: Vec<Location> = read_locations("tests/fixtures/grouped2_locations.csv");
    assert_eq!(output.1, expected_locations);
}
