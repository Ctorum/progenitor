use std::fs;
use std::path::Path;

pub fn create_files(project_path: &Path, project_name: &str) {
    fs::create_dir(project_path.join("src")).unwrap();
    
    fs::write(
        project_path.join("package.json"),
        format!(r#"{{
  "name": "{project_name}",
  "version": "1.0.0",
  "description": "Express JavaScript Microservice",
  "main": "src/index.js",
  "scripts": {{
    "start": "node src/index.js",
    "dev": "nodemon src/index.js"
  }},
  "dependencies": {{
    "express": "^4.18.2",
    "dotenv": "^16.3.1"
  }},
  "devDependencies": {{
    "nodemon": "^3.0.1"
  }}
}}"#)
    ).unwrap();

    fs::write(
        project_path.join("src/index.js"),
        r#"const express = require('express');
const dotenv = require('dotenv');

dotenv.config();

const app = express();
const port = process.env.PORT || 3000;

app.use(express.json());

app.get('/', (req, res) => {
  res.json({ message: 'Hello World!' });
});

app.listen(port, () => {
  console.log(`Server running at http://localhost:${port}`);
});"#
    ).unwrap();

    fs::write(
        project_path.join(".gitignore"),
        r#"# Dependencies
node_modules/

# Environment
.env

# Logs
*.log

# IDE
.vscode/
.idea/

# System Files
.DS_Store
Thumbs.db"#
    ).unwrap();
}
