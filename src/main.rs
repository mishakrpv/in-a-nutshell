mod command;
mod config;
mod nutshell;

use clap::Parser;
use command::config::*;
use config::get_config_path;
use nutshell::cli::{Cli, Commands};
use std::io::{self};

fn main() -> io::Result<()> {
    let args: Cli = Cli::parse();
    let config_path = get_config_path()?;

    match args.command {
        Commands::ConfigAdd { lhs, rhs } => {
            _ = add_mapping(&lhs, &rhs, &config_path);

            println!("Mapping has been added: {} -> {}", lhs, rhs);
        }
        Commands::ConfigRemove { lhs } => match remove_mapping(&lhs, &config_path) {
            Ok(_) => println!("Mapping has been removed for: {}", lhs),
            Err(e) => eprintln!("An error occured while removing mapping: {}", e),
        },
    }

    Ok(())
}
