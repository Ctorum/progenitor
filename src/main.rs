mod cli;
mod commands;
mod generator;

use crate::cli::cli::{Cli, Commands};
use clap::Parser;

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Create(cmd) => cmd.execute(),
        Commands::List(cmd) => cmd.execute(),
        Commands::Add(cmd) => cmd.execute(),
        Commands::Remove(cmd) => cmd.execute(),
    }
}
