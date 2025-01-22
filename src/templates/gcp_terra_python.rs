use std::fs;
use std::path::Path;

pub fn create_files(project_path: &Path, project_name: &str) {
    fs::create_dir(project_path.join("build")).unwrap();
    
    fs::write(
        project_path.join("main.py"),
        r#"import functions_framework

@functions_framework.http
def main(request):
    return 'Hello World!'
"#
    ).unwrap();

    fs::write(
        project_path.join("requirements.txt"),
        "functions-framework==3.*"
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
    description = "Python microservice function"

    build_config {{
        runtime     = "python311"
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

    fs::write(
        project_path.join(".gitignore"),
        r#"# Byte-compiled / optimized / DLL files
__pycache__/
*.py[cod]
*$py.class

# Distribution / packaging
dist/
build/
*.egg-info/

# Virtual environments
venv/
env/
ENV/

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
}