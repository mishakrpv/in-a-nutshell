use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::PathBuf;

pub(crate) fn remove_mapping(lhs: &str, config_path: &PathBuf) -> io::Result<()> {
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

pub(crate) fn add_mapping(lhs: &str, rhs: &str, config_path: &PathBuf) -> io::Result<()> {
    let mapping = format!("map({}) to ({})\n", lhs, rhs);

    let mut file = OpenOptions::new().append(true).open(&config_path)?;
    file.write_all(mapping.as_bytes())?;

    return Ok(());
}
