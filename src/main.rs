mod command;
mod config;
mod nutshell;

use clap::Parser;
use command::config::*;
use command::start::*;
use config::{get_config_path, read_config};
use env_logger;
use log::{debug, error, info, trace, warn};
use nutshell::cli::{Cli, Commands};
use signal_hook::{consts::SIGINT, iterator::Signals};
use std::env;
use std::sync::mpsc::{self, Receiver, Sender};
use std::{error::Error, thread, time::Duration};

fn main() -> Result<(), Box<dyn Error>> {
    let mut signals = Signals::new([SIGINT])?;

    thread::spawn(move || {
        for sig in signals.forever() {
            debug!("Received signal {:?}", sig);

            let exit_code = match sig {
                signal_hook::consts::SIGINT => 130,  // SIGINT (Ctrl+C)
                signal_hook::consts::SIGTERM => 143, // SIGTERM
                _ => 1,
            };

            std::process::exit(exit_code);
        }
    });

    match env::var("RUST_LOG") {
        Ok(value) => println!("The value of RUST_LOG is: {}", value),
        Err(_) => {
            env::set_var("RUST_LOG", "debug");
        }
    }

    env_logger::init();

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
        Commands::Start => {
            let key_mappings = read_config(&config_path)?;

            let (sender, receiver): (Sender<String>, Receiver<String>) = mpsc::channel();
            start_hotkey_listener(sender);

            loop {
                for key in receiver.iter() {
                    debug!("Key detected: {}", key);
                    handle_keypress(&key, &key_mappings);
                }

                thread::sleep(Duration::from_millis(100));
            }
        }
    }

    Ok(())
}
