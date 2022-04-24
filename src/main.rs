mod commands;
mod interface;
mod archive;
mod location;
mod parsers;

use clap::Parser;
use crate::interface::Commands;

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
        Some(Commands::Print { number, format }) => {
            commands::print::print(number, &format);
        }
    }
}
