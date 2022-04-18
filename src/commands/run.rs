use crate::archive;
use crate::interface::Style;
use crate::parsers;
use atty;
use pty::fork::Fork;
use std;
use std::io::{self, Read, Write};
use std::process::Command;

pub fn run(style: &Style, executable: &str, arguments: &Vec<String>) {
    let output = execute(&executable, &arguments);
    let (display, locations) = match style {
        Style::Grouped => parsers::grouped,
        Style::Linear => parsers::linear
    }(&output);
    _ = io::stdout().write(&display);
    _ = archive::write(&locations);
}

fn execute(executable: &str, arguments: &Vec<String>) -> Vec<u8> {
    if atty::is(atty::Stream::Stdout) {
        let fork = Fork::from_ptmx().unwrap();
        let mut output = Vec::new();
        if let Some(mut parent) = fork.is_parent().ok() {
            _ = parent.read_to_end(&mut output);
        } else {
            Command::new(executable)
                .args(arguments)
                .status()
                .expect(concat!("could not execute", stringify!(executable)));
        }
        return output;
    } else {
        let output = Command::new(executable)
            .args(arguments)
            .output()
            .expect(concat!("could not execute", stringify!(executable)));
        return output.stdout;
    }
}
