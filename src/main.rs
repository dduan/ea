use clap::Parser;
use ea::commands;
use ea::interface::{self, Commands};

fn main() {
    let args = interface::Interface::parse();
    match args.command {
        Some(Commands::Run {
            style,
            executable,
            arguments,
            debug,
        }) => {
            commands::run::run(&style, &executable, &arguments, debug);
        }
        Some(Commands::List) | None => {
            commands::list::list();
        },
    }
}
