use clap::Args;
use std::fs;
use std::path::Path;

use crate::templates::{fastapi, fiber};

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
        let templates = vec!["fastapi", "fiber"];
        
        if !templates.contains(&self.template.as_str()) {
            println!("Template '{}' not found!", self.template);
            println!("Available templates:");
            for template in templates {
                println!("- {}", template);
            }
            return;
        }

        let project_path = Path::new(&self.path).join(&self.name);
        fs::create_dir(&project_path).unwrap();

        match self.template.as_str() {
            "fastapi" => fastapi::create_files(&project_path),
            "fiber" => fiber::create_files(&project_path, &self.name),
            _ => unreachable!()
        }
        println!("✨ Project {} created successfully using {} template!", self.name, self.template);
    }
}