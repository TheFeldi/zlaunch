use anyhow::Result;
use clap::Parser;
use zlaunch::cli::{Cli, handle_client_command};
use zlaunch::daemon;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(cmd) => handle_client_command(cmd),
        None => daemon::run(),
    }
}
