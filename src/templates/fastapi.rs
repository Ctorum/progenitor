use std::fs;
use std::path::Path;

pub fn create_files(project_path: &Path) {
    fs::create_dir(project_path.join("app")).unwrap();
    fs::write(
        project_path.join("main.py"),
        "from fastapi import FastAPI\n\napp = FastAPI()\n\n@app.get('/')\ndef root():\n    return {'message': 'Hello World'}"
    ).unwrap();
    fs::write(
        project_path.join("requirements.txt"),
        "fastapi\nuvicorn"
    ).unwrap();
}
