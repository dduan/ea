use crate::archive;
use crate::interface::Style;
use crate::parsers;
use atty;
use pty::fork::Fork;
use std;
use std::error;
use std::fs;
use std::io::{self, Read, Write};
use std::process;

fn format_error(is_tty: bool, error: Box<dyn error::Error>) -> String {
    if is_tty {
        format!("\n\x1b[0m\x1b[31m[ea]: {}\x1b[0m\n", error)
    } else {
        format!("\n[ea]: {}\n", error)
    }
}

pub fn run(style: &Style, executable: &str, arguments: &[String], debug: Option<String>) {
    let is_tty = atty::is(atty::Stream::Stdout);
    let output = execute(is_tty, executable, arguments);
    _ = fs::write("/tmp/out", &output);
    let parsed = match style {
        Style::Grouped => parsers::grouped::grouped,
        Style::Linear => parsers::linear::linear,
        Style::Search => parsers::search::search,
    }(&output);

    let (display, locations) = match parsed {
        Ok(result) => result,
        Err(error) => {
            _ = io::stdout().write(&output);
            _ = io::stderr().write(format_error(is_tty, Box::new(error)).as_bytes());
            return;
        }
    };

    _ = io::stdout().write(&display);
    _ = archive::write(&locations);
    if let Some(debug_path) = debug {
        _ = fs::write(
            format!("{}.args", debug_path),
            format!("{:?}\n{}\n{:?}", style, executable, arguments),
        );
        _ = fs::write(format!("{}.in", debug_path), output);
        _ = fs::write(format!("{}.out", debug_path), &display);
    }
}

fn execute(is_tty: bool, executable: &str, arguments: &[String]) -> Vec<u8> {
    if is_tty {
        let fork = Fork::from_ptmx().unwrap();
        let mut output = Vec::new();
        if let Ok(mut parent) = fork.is_parent() {
            _ = parent.read_to_end(&mut output);
        } else {
            process::Command::new(executable)
                .args(arguments)
                .status()
                .expect(concat!("could not execute", stringify!(executable)));
            // TODO: should the exit code ever not be 0?
            process::exit(0)
        }
        output
    } else {
        let output = process::Command::new(executable)
            .args(arguments)
            .output()
            .expect(concat!("could not execute", stringify!(executable)));
        output.stdout
    }
}
