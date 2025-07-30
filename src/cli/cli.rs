use crate::commands::add::Add;
use crate::commands::create::Create;
use crate::commands::list::List;
use crate::commands::remove::Remove;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "progenitor")]
#[command(author = "Ctorum <dev.rezendevitor@gmail.com>")]
#[command(version = "0.3.0")]
#[command(about = "Every creation has your own creator", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Create(Create),
    List(List),
    Add(Add),
    Remove(Remove),
}
