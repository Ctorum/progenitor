use clap::{Parser, Subcommand};
use crate::commands::create::Create;
use crate::commands::list::List;

#[derive(Parser)]
#[command(name = "progenitor")]
#[command(author = "Scolion <scolion@protonmail.com>")]
#[command(version = "0.2.5")]
#[command(about = "A CLI tool for your project", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands
}

#[derive(Subcommand)]
pub enum Commands {
    Create(Create),
    List(List)
}
