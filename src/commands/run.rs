use crate::archive;
use crate::interface::Style;
use crate::parsers;
use atty;
use pty::fork::Fork;
use std;
use std::error;
use std::fmt;
use std::fs;
use std::io::{self, Read, Write};
use std::process;

#[derive(Debug)]
enum RunError {
    CouldNotExecuteCommand(String),
}

impl fmt::Display for RunError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RunError::CouldNotExecuteCommand(command) => write!(f, "could not execute {}", command),
        }
    }
}

impl error::Error for RunError {}

static ERROR_SIGNAL: [u8; 4] = [0xde, 0xad, 0xbe, 0xef];

fn format_error(is_tty: bool, error: Box<dyn error::Error>) -> String {
    if is_tty {
        format!("\x1b[0m\x1b[31m[ea]: {}\x1b[0m\n", error)
    } else {
        format!("[ea]: {}\n", error)
    }
}

pub fn run(style: &Style, executable: &str, arguments: &[String], debug: Option<String>) {
    let is_tty = atty::is(atty::Stream::Stdout);
    let output = execute(is_tty, executable, arguments);
    if output[..4] == ERROR_SIGNAL {
        _ = io::stderr().write(
            format_error(
                is_tty,
                Box::new(RunError::CouldNotExecuteCommand(executable.to_string())),
            )
            .as_bytes(),
        );
        _ = io::stderr().write(&output[5..]);
        eprintln!("");
        process::exit(output[4] as i32);
    }

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

fn execute_simple(executable: &str, arguments: &[String], output: &mut Vec<u8>) -> i32 {
    let execution_result = process::Command::new(executable).args(arguments).status();
    match execution_result {
        Ok(exit_status) => {
            if let Some(exit_code) = exit_status.code() {
                if exit_code != 0 {
                    output.extend_from_slice(&ERROR_SIGNAL);
                    output.push(exit_code as u8);
                    // TODO: better explanation?
                    output.extend_from_slice(b"an unknown error happend during execution");
                }
                return exit_code;
            } // TODO: else ??????
        }
        Err(error) => {
            // We are in a child process, whose entire output will be read by the parent.
            // To signal to the parent that something went wrong, we print out a special
            // sequence at the start of the output, followed by an error code.
            // Any error produced by the external command is attached after the error
            // signal sequence.
            output.extend_from_slice(&ERROR_SIGNAL);
            output.push(error.raw_os_error().unwrap_or(1) as u8);
            output.extend_from_slice(error.to_string().as_bytes());
            return 1;
        }
    }

    0
}

fn execute(is_tty: bool, executable: &str, arguments: &[String]) -> Vec<u8> {
    let mut output = Vec::new();
    if is_tty {
        let fork = Fork::from_ptmx().unwrap();
        if let Ok(mut parent) = fork.is_parent() {
            _ = parent.read_to_end(&mut output);
        } else {
            let code = execute_simple(executable, arguments, &mut output);
            if code != 0 {
                _ = io::stderr().write(&output);
            }
            process::exit(code);
        }
    } else {
        _ = execute_simple(executable, arguments, &mut output);
    }
    output
}
