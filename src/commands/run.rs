use crate::archive;
use crate::interface::Style;
use crate::parsers;
use atty;
use pty::fork::Fork;
use std;
use std::fs;
use std::io::{self, Read, Write};
use std::process::Command;

pub fn run(style: &Style, executable: &str, arguments: &[String], debug: Option<String>) {
    let output = execute(executable, arguments);
    let (display, locations) = match style {
        Style::Grouped => parsers::grouped,
        Style::Linear => parsers::linear,
        Style::Search => parsers::search,
    }(&output);
    _ = io::stdout().write(&display);
    _ = archive::write(&locations);
    if let Some(debug_path) = debug {
        _ = fs::write(format!("{}.args", debug_path), format!("{:?}\n{}\n{:?}", style, executable, arguments));
        _ = fs::write(format!("{}.in", debug_path), output);
        _ = fs::write(format!("{}.out", debug_path), &display);
    }
}

fn execute(executable: &str, arguments: &[String]) -> Vec<u8> {
    if atty::is(atty::Stream::Stdout) {
        let fork = Fork::from_ptmx().unwrap();
        let mut output = Vec::new();
        if let Ok(mut parent) = fork.is_parent() {
            _ = parent.read_to_end(&mut output);
        } else {
            Command::new(executable)
                .args(arguments)
                .status()
                .expect(concat!("could not execute", stringify!(executable)));
        }
        output
    } else {
        let output = Command::new(executable)
            .args(arguments)
            .output()
            .expect(concat!("could not execute", stringify!(executable)));
        output.stdout
    }
}
