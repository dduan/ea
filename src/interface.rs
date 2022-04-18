use clap::{ArgEnum, Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct Interface {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(ArgEnum, Clone, Debug)]
pub enum Style {
    Ripgrep,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Run {
        #[clap(arg_enum)]
        style: Style,
        executable: String,
        #[clap(last = true)]
        arguments: Vec<String>,
    },

    List,
}
