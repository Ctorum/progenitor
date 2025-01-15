use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "progenitor")]
#[command(author = "Scolion <scolion@protonmail.com>")]
#[command(version = "0.1.0")]
#[command(about = "A CLI tool for your project", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    Create {
        #[arg(short, long)]
        template: String,

        #[arg(short, long)]
        name: String,
    }
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Create {
            template,
            name,
        } => {
            println!("Creating project {} from template {}", name, template);
        }
    }
}
