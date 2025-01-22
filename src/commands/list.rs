use clap::Args;
use colored::*;

use crate::templates::TEMPLATES;

#[derive(Args)]
pub struct List {}

impl List {
    pub fn execute(&self) {
        println!("\nAvailable templates:\n");
        
        for template in TEMPLATES {
            println!("  • {}", template.name.bright_green());
        }
        
        println!("\nUse them with: pgen create <template-name>\n");
    }
}