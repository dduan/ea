use clap::Parser;

mod archive;
mod commands;
mod interface;
mod location;

fn main() {
    let args = interface::Interface::parse();
    match args.command {
        interface::Commands::Run {
            executable,
            arguments,
        } => {
            commands::run::run(&executable, &arguments);
        }
    }
}
