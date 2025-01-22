use std::fs;
use std::path::Path;

pub fn create_files(project_path: &Path, project_name: &str) {
    fs::create_dir(project_path.join("build")).unwrap();
    
    fs::write(
        project_path.join("index.js"),
        r#"exports.main = (req, res) => {
    res.send('Hello World!');
};"#
    ).unwrap();

    fs::write(
        project_path.join("package.json"),
        format!(r#"{{
    "name": "{project_name}",
    "version": "1.0.0",
    "description": "JavaScript Cloud Function",
    "main": "index.js",
    "engines": {{
        "node": ">=18.0.0"
    }},
    "scripts": {{
        "start": "functions-framework --target=main"
    }},
    "dependencies": {{
        "@google-cloud/functions-framework": "^3.0.0"
    }}
}}"#)
    ).unwrap();

    fs::write(
        project_path.join("main.tf"),
        format!(r#"locals {{
    project = "{project_name}"
}}

data "archive_file" "function_source" {{
    type = "zip"
    source_dir = "./build"
    output_path = "./build/main.zip"
}}

provider "google" {{
    credentials = file("key.json")
    project     = var.project
    region      = var.region
}}

resource "google_storage_bucket" "{project_name}-bkt" {{
    name     = "${{local.project}}-gcf-source"
    location = var.region
    force_destroy = true
    uniform_bucket_level_access = true
}}

resource "google_storage_bucket_object" "{project_name}-obj" {{
    name    = "main-${{data.archive_file.function_source.output_md5}}.zip"
    bucket  = google_storage_bucket.{project_name}-bkt.name
    source  = "build/main.zip"
}}

resource "google_cloudfunctions2_function" "{project_name}" {{
    name = "{project_name}"
    location = var.region
    description = "JavaScript microservice function"

    build_config {{
        runtime     = "nodejs18"
        entry_point = "main"
        source {{
            storage_source {{
                bucket = google_storage_bucket.{project_name}-bkt.name
                object = google_storage_bucket_object.{project_name}-obj.name
            }}
        }}
    }}

    service_config {{
        max_instance_count  = 1
        available_memory    = "256M"
        timeout_seconds     = 60
    }}
}}

resource "google_cloudfunctions2_function_iam_member" "invoker" {{
    project     = var.project
    location    = var.region
    cloud_function = google_cloudfunctions2_function.{project_name}.name

    role    = "roles/cloudfunctions.invoker"
    member  = "allUsers"
}}

resource "google_cloud_run_service_iam_member" "invoker" {{
    project     = var.project
    location    = var.region
    service     = google_cloudfunctions2_function.{project_name}.name

    role    = "roles/run.invoker"
    member  = "allUsers"
}}"#)
    ).unwrap();

    fs::write(
        project_path.join(".gitignore"),
        r#"# Dependencies
node_modules/
package-lock.json

# Build
build/
dist/

# Logs
logs
*.log
npm-debug.log*

# Runtime data
pids
*.pid
*.seed

# IDE
.idea/
.vscode/

# Terraform
terraform.tfstate
terraform.tfstate.*
.terraform/
*.tfvars

# Environment
key.json
.env"#
    ).unwrap();

    fs::write(
        project_path.join("variables.tf"),
        r#"variable "project" {
    type = string
    description = "GCP Project ID"
}

variable "region" {
    type = string
    description = "GCP Region"
    default = "us-central1"
}"#
    ).unwrap();
}
