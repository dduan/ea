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
    CommandEncounteredError(String),
    CommandWasInterrupted(String),
}

impl RunError {
    fn new(code: u8, command: &str) -> Option<Self> {
        match code {
            0 => Some(Self::CouldNotExecuteCommand(command.to_string())),
            1 => Some(Self::CommandEncounteredError(command.to_string())),
            2 => Some(Self::CommandWasInterrupted(command.to_string())),
            _ => None,
        }
    }

    fn could_not_execute_command() -> u8 { 0 }
    fn command_encountered_error() -> u8 { 1 }
    fn command_was_interrupted() -> u8 { 2 }
}

impl fmt::Display for RunError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RunError::CouldNotExecuteCommand(command) => write!(f, "could not execute {}", command),
            RunError::CommandEncounteredError(command) => write!(f, "{} encountered an error", command),
            RunError::CommandWasInterrupted(command) => write!(f, "{} was interrupted by signal", command),
        }
    }
}

impl error::Error for RunError {}

// deliberately mis-spelled
static ERROR_SIGNAL: [u8; 4] = [0xde, 0xad, 0xbe, 0xaf];

fn format_error(is_tty: bool, error: Box<dyn error::Error>) -> String {
    if is_tty {
        format!("\x1b[0m\x1b[31m[ea]: {}\x1b[0m\n", error)
    } else {
        format!("[ea]: {}\n", error)
    }
}

pub fn run(style: &Style, executable: &str, arguments: &[String], debug: Option<String>) {
    let is_tty = atty::is(atty::Stream::Stdout);
    let mut output = execute(is_tty, executable, arguments);
    let output_len = output.len();
    let mut error_exit_code: Option<i32> = None;
    if output_len >= 4 && output[(output_len - 4)..] == ERROR_SIGNAL {
        _ = io::stderr().write(&output);
        let error = RunError::new(output[output_len - 5], executable).expect("Error synthesized");
        _ = io::stderr().write(format_error(is_tty, Box::new(error)).as_bytes());
        error_exit_code = Some(output[output_len - 6] as i32);
        output = output[0..(output_len - 6)].to_vec();
    }

    let parsed = match style {
        Style::Grouped => parsers::grouped::grouped,
        Style::Linear => parsers::linear::linear,
        Style::Search => parsers::search::search,
        Style::Rust => parsers::rust::rust,
        Style::Python => parsers::python::python,
    }(&output);

    let (display, locations) = match parsed {
        Ok(result) => result,
        Err(error) => {
            _ = io::stdout().write(&output);
            _ = io::stderr().write(format_error(is_tty, Box::new(error)).as_bytes());
            return;
        }
    };

    if error_exit_code.is_none() || !&locations.is_empty() {
        _ = io::stdout().write(&display);
    } // else we would already let the executable print its errors, and we have nothing to add

    _ = archive::write(&locations);
    if let Some(debug_path) = debug {
        _ = fs::write(
            format!("{}.args", debug_path),
            format!("{:?}\n{}\n{:?}", style, executable, arguments),
        );
        _ = fs::write(format!("{}.in", debug_path), output);
        _ = fs::write(format!("{}.out", debug_path), &display);
    }

    if let Some(code) = error_exit_code {
        process::exit(code)
    }
}

fn execute_simple(executable: &str, arguments: &[String], output: &mut Vec<u8>) -> i32 {
    // We must run with .status() as opposed to .output() because we might be in a pty.
    // Running .output() would convince the process it's not in a pty!
    let execution_result = process::Command::new(executable).args(arguments).status();

    let error_code: u8;
    let exit: u8;
    match execution_result {
        Ok(exit_status) => {
            if let Some(exit_code) = exit_status.code() {
                exit = exit_code as u8;
                // if exit is 0, this following error won't really be used later.
                error_code = RunError::command_encountered_error();
            } else {
                exit = 1;
                error_code = RunError::command_was_interrupted();
            }
        }
        Err(error) => {
            output.extend_from_slice(error.to_string().as_bytes());
            exit = error.raw_os_error().unwrap_or(1) as u8;
            error_code = RunError::could_not_execute_command();
        }
    }

    // We are in a child process, whose entire output will be read by the parent.  To signal to the
    // parent that something went wrong, we print out a special sequence at the end of the output,
    // proceeded by an error code, proceeded by an exit status.  Any error produced by the external
    // command is attached after the error signal sequence.
    if exit != 0 {
        output.push(exit);
        output.push(error_code);
        output.extend_from_slice(&ERROR_SIGNAL);
    }

    exit as i32
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
