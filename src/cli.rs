use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::ipc::{Command, client};

#[derive(Parser)]
#[command(name = "zlaunch")]
#[command(about = "A fast application launcher for Linux")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Show the launcher window
    Show,
    /// Hide the launcher window
    Hide,
    /// Toggle the launcher window visibility
    Toggle,
    /// Quit the daemon
    Quit,
}

impl Commands {
    /// Convert to IPC command.
    pub fn to_ipc_command(&self) -> Command {
        match self {
            Commands::Show => Command::Show,
            Commands::Hide => Command::Hide,
            Commands::Toggle => Command::Toggle,
            Commands::Quit => Command::Quit,
        }
    }
}

/// Handle a client command by sending it to the running daemon.
pub fn handle_client_command(cmd: Commands) -> Result<()> {
    if !client::is_daemon_running() {
        eprintln!("Error: zlaunch daemon is not running");
        eprintln!("Start the daemon first by running: zlaunch");
        std::process::exit(1);
    }

    client::send_command(cmd.to_ipc_command())?;
    Ok(())
}
