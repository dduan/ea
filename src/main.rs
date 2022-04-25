mod archive;
mod commands;
mod interface;
mod parsers;

use crate::interface::Commands;
use clap::Parser;

fn main() {
    let args = interface::Interface::parse();
    match args.subcommand {
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
        }
        Some(Commands::Print { number, format }) => {
            commands::print::print(number, &format);
        }
    }
}
