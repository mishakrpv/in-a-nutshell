mod config;

use std::env;
use std::fs::OpenOptions;
use std::io::{self, Write};
use config::config_path;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Usage: nutshell config <hotkey> <command>");
        return Ok(());
    }

    let key = &args[2];
    let command = &args[3];

    let mapping = format!("map({}) to ({})\n", key, command);

    let config_path = config_path()?;

    let mut file = OpenOptions::new().append(true).open(&config_path)?;
    file.write_all(mapping.as_bytes())?;

    println!("Mapping added: map({}) to ({})", key, command);

    Ok(())
}