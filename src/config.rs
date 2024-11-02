use dirs::{config_dir, home_dir};
use std::fs::{self, OpenOptions};
use std::io::{self};
use std::path::PathBuf;

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
