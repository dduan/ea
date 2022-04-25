use clap::CommandFactory;
use clap_complete::{generate_to, Shell};
use std::fs;
use std::path::PathBuf;
use Shell::*;

include!("src/interface.rs");

fn main() {
    let outdir: PathBuf = [
        env!("CARGO_MANIFEST_DIR"),
        "resources",
        "completion_scripts",
    ]
    .iter()
    .collect();
    fs::create_dir_all(&outdir).unwrap();
    let mut cmd = Interface::command();
    for shell in [Bash, PowerShell, Fish, Elvish, Zsh] {
        generate_to(shell, &mut cmd, "ea", &outdir).unwrap();
    }
}
