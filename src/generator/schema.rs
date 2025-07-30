use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct GeneratorSchema {
    pub description: Option<String>,
    #[serde(default)]
    pub variables: Vec<Variable>,
    pub structure: Vec<StructureItem>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Variable {
    pub name: String,
    #[serde(rename = "type")]
    pub var_type: String, // e.g., "string", "number", "boolean"
    pub default: Option<String>,
    pub prompt: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum StructureItem {
    #[serde(rename = "directory")]
    Directory {
        name: String,
        #[serde(default)]
        content: Vec<StructureItem>,
    },
    #[serde(rename = "file")]
    File { name: String, source: String },
}
