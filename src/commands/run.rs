use crate::archive;
use crate::interface::Style;
use crate::location::Location;
use crate::parsers;
use atty;
use pty::fork::Fork;
use std;
use std::io::{self, Read, Write};
use std::process::Command;

pub fn run(style: &Style, executable: &str, arguments: &Vec<String>) {
    process(&style, &execute(&executable, &arguments));
}

fn execute(executable: &str, arguments: &Vec<String>) -> Vec<u8> {
    if atty::is(atty::Stream::Stdout) {
        let fork = Fork::from_ptmx().unwrap();

        let mut output = Vec::new();
        if let Some(mut parent) = fork.is_parent().ok() {
            _ = parent.read(&mut output);
            print!("[{}]", std::str::from_utf8(&output).unwrap());
        } else {
            let e = Command::new(executable)
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

fn process(style: &Style, output: &[u8]) {
    let locations: Vec<Location>;
    let display: Vec<u8>;
    match style {
        Style::Ripgrep => {
            (display, locations) = parsers::ripgrep(&output);
        }
    }
    _ = io::stdout().write(&display);
    _ = archive::write(&locations);
}
