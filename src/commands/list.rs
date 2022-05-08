use atty;
use crate::archive;
use crate::Location;

fn colored_location(index: &usize, location: &Location) -> String {
    let mut output = format!("[\x1b[0m\x1b[31m{}\x1b[0m] {}", index + 1, location.path);

    if let Some(line_number) = location.line {
        output = format!("{}\x1b[2m:\x1b[0m\x1b[32m{}\x1b[0m", output, &line_number);
        if let Some(column_number) = location.column {
            output = format!("{}\x1b[2m:{}\x1b[0m", output, column_number);
        }
    }

    output
}

fn uncolored_location(index: &usize, location: &Location) -> String {
    let mut output = format!("[{}] {}", index + 1, location.path);
    if let Some(line_number) = location.line {
        output = format!("{}:{}", output, &line_number);
        if let Some(column_number) = location.column {
            output = format!("{}:{}", output, column_number);
        }
    }

    output
}

pub fn list() {
    for (idx, location) in archive::read().iter().enumerate() {
        if atty::is(atty::Stream::Stdout) {
            println!("{}", colored_location(&idx, location));
        } else {
            println!("{}", uncolored_location(&idx, location));
        }
    }
}
