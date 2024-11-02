use log::{debug, info};
use rdev::{listen, EventType};
use std::collections::HashMap;
use std::process::{Command, Output, Stdio};
use std::sync::mpsc::Sender;
use std::thread;

pub(crate) fn start_hotkey_listener(sender: Sender<String>) {
    thread::spawn(move || {
        if let Err(error) = listen(move |event| match event.event_type {
            EventType::KeyPress(key) => {
                let key_str = format!("{:?}", key);
                sender.send(key_str).unwrap();
            }
            _ => {}
        }) {
            eprintln!("Error: {:?}", error);
        }
    });
}

pub(crate) fn handle_keypress(key: &str, key_mappings: &HashMap<String, String>) {
    if let Some(command) = key_mappings.get(key) {
        info!("Executing command for key {}: {}", key, command);
        match execute_command(command) {
            Ok(output) => {
                info!("Command has been executed successfully.");
                debug!("{}", String::from_utf8_lossy(&output.stdout));
            }
            Err(e) => eprintln!("Failed to execute command: {}", e),
        }
    }
}

fn execute_command(command: &str) -> std::io::Result<Output> {
    // Split the command into its main part and arguments
    let parts: Vec<&str> = command.split_whitespace().collect();

    // Check if there is a command to execute
    if parts.is_empty() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Empty command",
        ));
    }

    // Separate the command from its arguments
    let (command, args) = parts.split_first().unwrap();

    Command::new(command)
        .args(args)
        .stdout(Stdio::inherit()) // Inherit stdout to display output in the terminal
        .stderr(Stdio::inherit()) // Inherit stderr to display errors in the terminal
        .output() // Execute the command and wait for it to finish
}
