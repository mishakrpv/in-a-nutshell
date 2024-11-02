mod config;
mod nutshell;

use clap::Parser;
use config::get_config_path;
use nutshell::cli::{Cli, Commands};
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::PathBuf;

fn remove_mapping(lhs: &str, config_path: &PathBuf) -> io::Result<()> {
    let contents = fs::read_to_string(config_path)?;
    let mut mappings: Vec<String> = contents.lines().map(String::from).collect();

    mappings.retain(|mapping| !mapping.contains(lhs));

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(config_path)?;
    for mapping in mappings {
        writeln!(file, "{}", mapping)?;
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let args: Cli = Cli::parse();

    match args.command {
        Commands::ConfigAdd { lhs, rhs } => {
            let mapping = format!("map({}) to ({})\n", lhs, rhs);
            let config_path = get_config_path()?;

            let mut file = OpenOptions::new().append(true).open(&config_path)?;
            file.write_all(mapping.as_bytes())?;
            println!("Mapping has been added: {} -> {}", lhs, rhs);
        }
        Commands::ConfigRemove { lhs } => {
            let config_path = get_config_path()?;

            match remove_mapping(&lhs, &config_path) {
                Ok(_) => println!("Mapping has been removed for: {}", lhs),
                Err(e) => eprintln!("An error occured while removing mapping: {}", e),
            }
        }
    }

    Ok(())
}
