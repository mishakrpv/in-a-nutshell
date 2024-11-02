use dirs::{config_dir, home_dir};
use log::debug;
use std::fs::{self, OpenOptions};
use std::io::{self, BufRead};
use std::path::PathBuf;
use std::fs::File;
use std::collections::HashMap;

pub(crate) fn read_config<P: AsRef<std::path::Path>>(path: P) -> io::Result<HashMap<String, String>> {
    let mut key_mappings = HashMap::new();
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        if line.trim().starts_with("map(") {
            // Parse lines like: map(<C-r>) to (echo 'Control + R pressed')
            if let Some((lhs, rhs)) = parse_mapping_line(&line) {
                debug!("lhs: {}, rhs: {}", lhs, rhs);
                key_mappings.insert(lhs, rhs);
            }
        }
    }
    Ok(key_mappings)
}

fn parse_mapping_line(line: &str) -> Option<(String, String)> {
    let map_parts: Vec<&str> = line.split(") to (").collect();
    if map_parts.len() == 2 {
        let lhs = map_parts[0].trim_start_matches("map(").to_string();
        let rhs = map_parts[1].trim_end_matches(')').to_string();
        Some((lhs, rhs))
    } else {
        None
    }
}

pub(crate) fn get_config_path() -> io::Result<PathBuf> {
    let potential_locations = [
        config_dir().map(|p| p.join("nutshell/config.nutshell")),
        home_dir().map(|p| p.join(".nutshell/config.nutshell")),
        home_dir().map(|p| p.join("config.nutshell")),
    ];

    for location in &potential_locations {
        if let Some(path) = location {
            if path.exists() {
                return Ok(path.clone());
            }
        }
    }

    let default_location = config_dir()
        .unwrap_or_else(|| home_dir().expect("Home directory not found"))
        .join("nutshell/config.nutshell");

    if let Some(parent) = default_location.parent() {
        fs::create_dir_all(parent)?;
    }

    OpenOptions::new()
        .create(true)
        .append(true)
        .open(&default_location)?;

    Ok(default_location)
}
