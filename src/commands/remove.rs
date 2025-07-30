use clap::Args;
use colored::Colorize;
use std::path::PathBuf;
use std::fs;

#[derive(Args, Debug)]
pub struct Remove {
    /// Name of the template to remove
    #[arg(short, long)]
    pub name: Option<String>,
    /// Remove all templates
    #[arg(long)]
    pub all: bool,
}

impl Remove {
    pub fn execute(&self) {
        let home_dir = match dirs::home_dir() {
            Some(path) => path,
            None => {
                eprintln!("{}: Could not find home directory.", "Error".red());
                return;
            }
        };
        let progenitor_templates_dir = home_dir.join(".progenitor").join("templates");

        if self.all {
            self.remove_all_templates(&progenitor_templates_dir);
        } else if let Some(name) = &self.name {
            self.remove_single_template(&progenitor_templates_dir, name);
        } else {
            eprintln!("{}: Please specify a template name to remove or use the --all flag.", "Error".red());
        }
    }

    fn remove_all_templates(&self, templates_dir: &PathBuf) {
        if !templates_dir.exists() {
            println!("{}: No templates directory found at {}. Nothing to remove.", "Info".blue(), templates_dir.display());
            return;
        }

        match fs::remove_dir_all(templates_dir) {
            Ok(_) => println!("{}: All templates removed successfully from {}!", "Success".green(), templates_dir.display()),
            Err(e) => eprintln!("{}: Failed to remove all templates from {}: {}", "Error".red(), templates_dir.display(), e),
        }
    }

    fn remove_single_template(&self, templates_dir: &PathBuf, name: &str) {
        let template_path = templates_dir.join(name);

        if !template_path.exists() {
            eprintln!("{}: Template '{}' not found at {}.", "Error".red(), name, template_path.display());
            return;
        }

        match fs::remove_dir_all(&template_path) {
            Ok(_) => println!("{}: Template '{}' removed successfully!", "Success".green(), name),
            Err(e) => eprintln!("{}: Failed to remove template '{}': {}", "Error".red(), name, e),
        }
    }
}
