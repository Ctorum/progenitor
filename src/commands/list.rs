use clap::Args;
use colored::*;

#[derive(Args)]
pub struct List {}

impl List {
    pub fn execute(&self) {
        println!("\nAvailable templates:\n");
        
        println!("  • {}", "fastapi".bright_green());
        println!("  • {}", "fiber".bright_green());
        println!("  • {}", "gcp_terra_go".bright_green());
        
        println!("\nUse them with: pgen create <template-name>\n");
    }
}