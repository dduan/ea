use clap::Parser;
use interface::Commands;

mod archive;
mod commands;
mod interface;
mod location;

fn main() {
    let args = interface::Interface::parse();
    match args.command {
        Commands::Run {
            executable,
            arguments,
        } => {
            commands::run::run(&executable, &arguments);
        }
        Commands::List => {
            commands::list::list();
        }
    }
}
