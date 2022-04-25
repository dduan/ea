use clap::{ArgEnum, Parser, Subcommand, ValueHint};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Interface {
    #[clap(subcommand)]
    pub subcommand: Option<Commands>,
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
        #[clap(value_hint = ValueHint::CommandName)]
        executable: String,
        #[clap(last = true)]
        arguments: Vec<String>,
        #[clap(long, value_name = "debug_files_base_name", value_hint = ValueHint::FilePath)]
        debug: Option<String>,
    },

    List,

    #[clap(alias("p"))]
    Print {
        #[clap(required = true)]
        number: usize,
        #[clap(default_value = "{path}")]
        format: String,
    },
}
