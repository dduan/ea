use assert_cmd::prelude::CommandCargoExt;
use std::path::PathBuf;
use std::process;
use std::io::Read;
use pty::fork::Fork;

fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|window| window == needle)
}

#[test]
// Run `ea run` in a terminal with a sample app. Check output is reasonably modified by `ea`.
fn invoke_subprocess_via_pty() -> Result<(), Box<dyn std::error::Error>> {
    let mut ea = process::Command::cargo_bin("ea")?;
    let printer_cmd = process::Command::cargo_bin("printfile")?;
    let printer = printer_cmd.get_program();
    let input_path: PathBuf = [
        env!("CARGO_MANIFEST_DIR"),
        "resources",
        "fixtures",
        "parsers",
        "grouped.in.txt"
    ].iter().collect();

    let fork = Fork::from_ptmx().unwrap();
    let mut output = Vec::new();
    if let Ok(mut parent) = fork.is_parent() {
        _ = parent.read_to_end(&mut output);
    } else {
        ea
            .args(["run", "grouped"])
            .arg(printer)
            .arg("--")
            .arg(input_path)
            .status()
            .expect("execution fails");
        process::exit(0)
    }
    assert!(find_subsequence(&output, b"[0m[[31m1[0m] [0m[32m20[0m:").is_some());
    Ok(())
}

#[test]
fn subprocess_command_error() -> Result<(), Box<dyn std::error::Error>> {
    let mut ea = process::Command::cargo_bin("ea")?;
    let fork = Fork::from_ptmx().unwrap();
    let mut output = Vec::new();
    if let Ok(mut parent) = fork.is_parent() {
        _ = parent.read_to_end(&mut output);
    } else {
        ea
            .args(["run", "rust", "cargo", "--", "clippppy"])
            .status()
            .expect("execution fails");
        process::exit(0)
    }
    assert!(find_subsequence(&output, b"[ea]: cargo encountered an error").is_some());
    Ok(())
}

#[test]
fn subprocess_execution_fail() -> Result<(), Box<dyn std::error::Error>> {
    let mut ea = process::Command::cargo_bin("ea")?;
    let fork = Fork::from_ptmx().unwrap();
    let mut output = Vec::new();
    if let Ok(mut parent) = fork.is_parent() {
        _ = parent.read_to_end(&mut output);
    } else {
        ea
            .args(["run", "rust", "/lmao/i/do/not/exist"])
            .status()
            .expect("execution fails");
        process::exit(0)
    }
    assert!(find_subsequence(&output, b"[ea]: could not execute /lmao/i/do/not/exist").is_some());
    Ok(())
}
