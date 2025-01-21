use std::fs;
use std::path::Path;

pub fn create_files(project_path: &Path, project_name: &str) {
    fs::create_dir(project_path.join("build")).unwrap();
    
    fs::write(
        project_path.join("main.go"),
        r#"package main

import (
    "fmt"
    "net/http"
)

func Main(w http.ResponseWriter, r *http.Request) {
    fmt.Fprintf(w, "Hello World!")
}
"#
    ).unwrap();

    fs::write(
        project_path.join("go.mod"),
        "module example.com/function\n\ngo 1.21"
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
    description = "Go microservice for scheduling and managing contacts"

    build_config {{
        runtime     = "go121"
        entry_point = "Main"
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
        r#"# Created by https://www.toptal.com/developers/gitignore/api/go
# Edit at https://www.toptal.com/developers/gitignore?templates=go

### Go ###
# If you prefer the allow list template instead of the deny list, see community template:
# https://github.com/github/gitignore/blob/main/community/Golang/Go.AllowList.gitignore
#
# Binaries for programs and plugins
*.exe
*.exe~
*.dll
*.so
*.dylib

# Test binary, built with `go test -c`
*.test

# Output of the go coverage tool, specifically when used with LiteIDE
*.out

# Dependency directories (remove the comment below to include it)
# vendor/

# Go workspace file
go.work

# End of https://www.toptal.com/developers/gitignore/api/go

/build
terraform.tfstate
terraform.tfstate.*
.terraform/

#env
key.json
variables.tf
.env"#
    ).unwrap();
}
