use clap::Args;
use colored::*;
use std::fs;

#[derive(Args)]
pub struct List {}

impl List {
    pub fn execute(&self) {
        let current_dir = std::env::current_dir().unwrap();
        let templates_dir = current_dir.join("src/templates");
        
        println!("\nAvailable templates:\n");
        
        if let Ok(entries) = fs::read_dir(&templates_dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Ok(file_name) = entry.file_name().into_string() {
                        if file_name != "mod.rs" && file_name.ends_with(".rs") {
                            let template_name = file_name.trim_end_matches(".rs");
                            println!("  • {}", template_name.bright_green());
                        }
                    }
                }
            }
        } else {
            println!("  No templates found in {} directory", templates_dir.display());
        }
        
        println!("\nUse them with: pgen create <template-name>\n");
    }
}