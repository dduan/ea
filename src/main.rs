use clap::Parser;
use ea::commands;
use ea::interface::{self, Commands};

fn main() {
    let args = interface::Interface::parse();
    match args.command {
        Commands::Run {
            style,
            executable,
            arguments,
        } => {
            commands::run::run(&style, &executable, &arguments);
        }
        Commands::List => {
            commands::list::list();
        }
    }
}
