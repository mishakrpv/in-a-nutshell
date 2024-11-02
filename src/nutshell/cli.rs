use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "nutshell")]
#[command(
    about = "Map and execute terminal commands via hotkeys",
    version = "0.1.0"
)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub(crate) enum Commands {
    /// Adds a new hotkey mapping to the config
    #[command(name = "config-add")]
    ConfigAdd {
        /// The left-hand-side pattern to map (e.g., <C-r>)
        lhs: String,
        /// The right-hand-side command to execute when the key is pressed
        rhs: String,
    },
    #[command(name = "config-remove")]
    ConfigRemove {
        /// The left-hand-side pattern to remove
        lhs: String,
    },
}
