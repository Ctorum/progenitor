use clap::Args;
use std::fs;
use std::path::Path;
use colored::*;

use crate::templates::TEMPLATES;

#[derive(Args)]
pub struct Create {
    #[arg(short, long)]
    pub template: String,

    #[arg(short, long)]
    pub name: String,

    pub path: String,
}

impl Create {
    pub fn execute(&self) {        
        let template = TEMPLATES.iter().find(|t| t.name == self.template);
        
        if template.is_none() {
            println!("{}", format!("Template '{}' not found!", self.template).red());
            println!("{}", "Available templates:".yellow());
            for template in TEMPLATES {
                println!("{} {}", "-".yellow(), template.name.green());
            }
            return;
        }

        let project_path = Path::new(&self.path).join(&self.name);
        fs::create_dir(&project_path).unwrap();

        (template.unwrap().create_fn)(&project_path, &self.name);
        
        println!("{} Project {} created successfully using {} template!", "✨".bright_yellow(), self.name.green(), self.template.green());
    }}