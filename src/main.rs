use clap::Parser;

mod interface;
mod location;
mod commands;

fn main() {
    let args = interface::Interface::parse();
    match args.command {
        interface::Commands::Run { executable, arguments } => {
            commands::run::run(&executable, &arguments);
        }
    }
}
