use clap::Args;
use colored::*;
use std::fs;

#[derive(Args)]
pub struct List {}

impl List {
    pub fn execute(&self) {
        let home_dir = match dirs::home_dir() {
            Some(path) => path,
            None => {
                eprintln!("{}: Could not find home directory.", "Error".red());
                return;
            }
        };
        let templates_dir = home_dir.join(".progenitor").join("templates");

        println!("\nAvailable templates:\n");

        if let Ok(entries) = fs::read_dir(&templates_dir) {
            let mut found_templates = false;
            for entry in entries {
                if let Ok(entry) = entry {
                    if entry.file_type().map_or(false, |ft| ft.is_dir()) {
                        if let Some(template_name) = entry.file_name().to_str() {
                            println!("  • {}", template_name.bright_green());
                            found_templates = true;
                        }
                    }
                }
            }
            if !found_templates {
                println!("  {}", "No templates found. Use `pgen add` to add new templates.".italic().dimmed());
            }
        } else {
            println!("  {}", "Could not read templates directory. Run `pgen add` to create it.".italic().dimmed());
        }

        println!("\nUse them with: pgen create -t <template-name> -n <project-name> <path>\n");
    }
}
