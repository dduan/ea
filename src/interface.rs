use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct Interface {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Run {
        executable: String,
        arguments: Vec<String>,
    }
}
