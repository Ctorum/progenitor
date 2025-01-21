mod cli;
mod commands;
mod templates;

use clap::Parser;
use crate::cli::cli::{Cli, Commands};

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Create(cmd) => cmd.execute(),
        Commands::List(cmd) => cmd.execute(),
    }
}