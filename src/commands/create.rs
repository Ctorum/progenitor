use crate::generator::schema::{GeneratorSchema, StructureItem, Variable};
use clap::Args;
use colored::*;
use std::collections::HashMap;
use std::fs;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

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
        let home_dir = match dirs::home_dir() {
            Some(path) => path,
            None => {
                eprintln!("{}: Could not find home directory.", "Error".red());
                return;
            }
        };
        let templates_dir = home_dir.join(".progenitor").join("templates");
        let template_path = templates_dir.join(&self.template);
        let generator_yml_path = template_path.join("generator.yml");

        if !template_path.exists() || !template_path.is_dir() {
            eprintln!(
                "{}: Template '{}' not found. Please ensure it's added using `pgen add`.",
                "Error".red(),
                self.template
            );
            self.list_available_templates(&templates_dir);
            return;
        }

        let (template_name, generator_schema) = match self.parse_generator_yml(&generator_yml_path)
        {
            Ok(data) => data,
            Err(e) => {
                eprintln!(
                    "{}: Failed to parse `generator.yml` for template '{}': {}",
                    "Error".red(),
                    self.template,
                    e
                );
                return;
            }
        };

        let mut variables = HashMap::new();
        variables.insert("project_name".to_string(), self.name.clone()); // Always include project_name

        if let Err(e) = self.collect_variables(&generator_schema.variables, &mut variables) {
            eprintln!("{}: Failed to collect variables: {}", "Error".red(), e);
            return;
        }

        let project_root_path =
            PathBuf::from(&self.path).join(self.replace_variables(&self.name, &variables));

        if project_root_path.exists() {
            eprintln!(
                "{}: Project directory '{}' already exists.",
                "Error".red(),
                project_root_path.display()
            );
            return;
        }

        match self.generate_project(
            &template_path,
            &project_root_path,
            &generator_schema.structure,
            &variables,
        ) {
            Ok(_) => println!(
                "{} Project {} created successfully using {} template!",
                "✨".bright_yellow(),
                self.name.green(),
                template_name.green()
            ),
            Err(e) => eprintln!("{}: Failed to create project: {}", "Error".red(), e),
        }
    }

    fn list_available_templates(&self, templates_dir: &Path) {
        println!("{}", "Available templates:".yellow());
        if let Ok(entries) = fs::read_dir(templates_dir) {
            let mut found_templates = false;
            for entry in entries {
                if let Ok(entry) = entry {
                    if entry.file_type().map_or(false, |ft| ft.is_dir()) {
                        if let Some(template_name) = entry.file_name().to_str() {
                            println!("{} {}", "-".yellow(), template_name.green());
                            found_templates = true;
                        }
                    }
                }
            }
            if !found_templates {
                println!(
                    "  {}",
                    "No templates found. Use `pgen add` to add new templates."
                        .italic()
                        .dimmed()
                );
            }
        } else {
            println!(
                "  {}",
                "Could not read templates directory.".italic().dimmed()
            );
        }
    }

    fn parse_generator_yml(
        &self,
        path: &PathBuf,
    ) -> Result<(String, GeneratorSchema), Box<dyn std::error::Error>> {
        let mut file = fs::File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let schema_map: HashMap<String, GeneratorSchema> = serde_yaml::from_str(&contents)?;

        let (template_name, schema) = schema_map
            .into_iter()
            .next()
            .ok_or("Empty generator.yml or missing top-level template name")?;
        Ok((template_name, schema))
    }

    fn collect_variables(
        &self,
        defined_vars: &[Variable],
        collected_vars: &mut HashMap<String, String>,
    ) -> Result<(), io::Error> {
        for var in defined_vars {
            if collected_vars.contains_key(&var.name) {
                continue;
            }

            let value = if let Some(prompt) = &var.prompt {
                println!("{}", prompt.cyan());
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                input.trim().to_string()
            } else if let Some(default_val) = &var.default {
                default_val.clone()
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!(
                        "Required variable '{}' has no default and no prompt.",
                        var.name
                    ),
                ));
            };
            collected_vars.insert(var.name.clone(), value);
        }
        Ok(())
    }

    fn generate_project(
        &self,
        template_base_path: &Path,
        current_project_path: &Path,
        structure_items: &[StructureItem],
        variables: &HashMap<String, String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        fs::create_dir_all(current_project_path)?;

        for item in structure_items {
            match item {
                StructureItem::Directory { name, content } => {
                    let new_dir_name = self.replace_variables(name, variables);
                    let new_dir_path = current_project_path.join(&new_dir_name);
                    self.generate_project(template_base_path, &new_dir_path, content, variables)?;
                }
                StructureItem::File { name, source } => {
                    let source_file_path = template_base_path.join(source);
                    let mut file = fs::File::open(&source_file_path)?;
                    let mut contents = String::new();
                    file.read_to_string(&mut contents)?;

                    let processed_contents = self.replace_variables(&contents, variables);
                    let new_file_name = self.replace_variables(name, variables);
                    let new_file_path = current_project_path.join(&new_file_name);

                    let mut output_file = fs::File::create(&new_file_path)?;
                    output_file.write_all(processed_contents.as_bytes())?;
                }
            }
        }
        Ok(())
    }

    fn replace_variables(&self, text: &str, variables: &HashMap<String, String>) -> String {
        let mut result = text.to_string();
        for (key, value) in variables {
            let placeholder = format!("${{{}}}", key);
            result = result.replace(&placeholder, value);
        }
        result
    }
}
