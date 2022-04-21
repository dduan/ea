use clap::{ArgEnum, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Interface {
    #[clap(subcommand)]
    pub command: Option<Commands>,
}

#[derive(ArgEnum, Clone, Debug)]
pub enum Style {
    Grouped,
    Linear,
    Search,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Run {
        #[clap(arg_enum)]
        style: Style,
        executable: String,
        #[clap(last = true)]
        arguments: Vec<String>,
        #[clap(long, value_name = "debug_files_base_name")]
        debug: Option<String>,
    },

    List,
}
