use crate::generator::schema::{GeneratorSchema, StructureItem};
use clap::Args;
use colored::Colorize;
use std::collections::HashMap;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

#[derive(Args, Debug)]
pub struct Add {
    #[arg(short, long)]
    pub path: String,
}

impl Add {
    pub fn execute(&self) {
        let template_path = PathBuf::from(&self.path);

        if !template_path.exists() {
            eprintln!("{}: The specified path does not exist.", "Error".red());
            return;
        }

        if !template_path.is_dir() {
            eprintln!("{}: The specified path is not a directory.", "Error".red());
            return;
        }

        let generator_yml_path = template_path.join("generator.yml");
        if !generator_yml_path.exists() {
            eprintln!(
                "{}: `generator.yml` not found in the template directory.",
                "Error".red()
            );
            return;
        }

        let (template_name, generator_schema) = match self.parse_generator_yml(&generator_yml_path)
        {
            Ok(data) => data,
            Err(e) => {
                eprintln!("{}: Failed to parse `generator.yml`: {}", "Error".red(), e);
                return;
            }
        };

        if let Err(e) = self.validate_gen_files(&template_path, &generator_schema) {
            eprintln!("{}: Template validation failed: {}", "Error".red(), e);
            return;
        }

        match self.copy_template(&template_path, &template_name, &generator_schema) {
            Ok(_) => println!(
                "{}: Template '{}' added successfully!",
                "Success".green(),
                template_name
            ),
            Err(e) => eprintln!("{}: Failed to add template: {}", "Error".red(), e),
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

    fn validate_gen_files(
        &self,
        template_root: &Path,
        schema: &GeneratorSchema,
    ) -> Result<(), String> {
        self.check_structure_items(template_root, &schema.structure, template_root)
    }

    fn check_structure_items(
        &self,
        current_path_in_template: &Path,
        items: &[StructureItem],
        template_root_source: &Path,
    ) -> Result<(), String> {
        for item in items {
            match item {
                StructureItem::Directory { name, content } => {
                    let dir_path = current_path_in_template.join(name);
                    self.check_structure_items(&dir_path, content, template_root_source)?;
                }
                StructureItem::File { name: _, source } => {
                    let gen_file_path = template_root_source.join(source);
                    if !gen_file_path.exists() {
                        return Err(format!(
                            "Referenced .gen file not found: {}",
                            gen_file_path.display()
                        ));
                    }
                    if !gen_file_path.is_file() {
                        return Err(format!(
                            "Referenced .gen path is not a file: {}",
                            gen_file_path.display()
                        ));
                    }
                }
            }
        }
        Ok(())
    }

    fn copy_template(
        &self,
        source_path: &Path,
        template_name: &str,
        schema: &GeneratorSchema,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let home_dir = dirs::home_dir().ok_or("Could not find home directory")?;
        let progenitor_dir = home_dir.join(".progenitor").join("templates");
        let destination_path = progenitor_dir.join(template_name);

        if destination_path.exists() {
            return Err(format!(
                "Template '{}' already exists at {}. Please remove it first.",
                template_name,
                destination_path.display()
            )
            .into());
        }

        fs::create_dir_all(&destination_path)?;

        fs::copy(
            source_path.join("generator.yml"),
            destination_path.join("generator.yml"),
        )?;

        self.copy_structure_items(
            source_path,
            &destination_path,
            &schema.structure,
            source_path,
            &destination_path,
        )?;

        Ok(())
    }

    fn copy_structure_items(
        &self,
        current_source_dir: &Path,
        current_dest_dir: &Path,
        items: &[StructureItem],
        template_root_source: &Path,
        template_root_destination: &Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for item in items {
            match item {
                StructureItem::Directory { name, content } => {
                    let new_source_dir = current_source_dir.join(name);
                    let new_dest_dir = current_dest_dir.join(name);
                    fs::create_dir_all(&new_dest_dir)?;
                    self.copy_structure_items(
                        &new_source_dir,
                        &new_dest_dir,
                        content,
                        template_root_source,
                        template_root_destination,
                    )?;
                }
                StructureItem::File { name: _, source } => {
                    let relative_path_from_template_root = PathBuf::from(source);
                    let full_src_path =
                        template_root_source.join(&relative_path_from_template_root);
                    let full_dest_path =
                        template_root_destination.join(&relative_path_from_template_root);

                    if let Some(parent) = full_dest_path.parent() {
                        fs::create_dir_all(parent)?;
                    }
                    fs::copy(&full_src_path, &full_dest_path)?;
                }
            }
        }
        Ok(())
    }
}
