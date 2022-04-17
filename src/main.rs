use clap::Parser;
use interface::Commands;

mod archive;
mod commands;
mod interface;
mod location;
mod parsers;

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
