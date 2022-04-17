use clap::{Parser, Subcommand, ArgEnum};

#[derive(Debug, Parser)]
pub struct Interface {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(ArgEnum, Clone, Debug)]
pub enum Style {
    Ripgrep
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Run {
        #[clap(arg_enum)]
        style: Style,
        executable: String,
        arguments: Vec<String>,
    },

    List,
}
