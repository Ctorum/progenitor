use std::fs;
use std::path::Path;

pub fn create_files(project_path: &Path, project_name: &str) {
    fs::create_dir(project_path.join("src")).unwrap();
    fs::create_dir(project_path.join("build")).unwrap();
    
    fs::write(
        project_path.join("package.json"),
        format!(r#"{{
  "name": "{project_name}",
  "version": "1.0.0",
  "description": "Express TypeScript Microservice",
  "main": "dist/index.js",
  "scripts": {{
    "build": "tsc",
    "start": "node dist/index.js",
    "dev": "ts-node-dev src/index.ts"
  }},
  "dependencies": {{
    "express": "^4.18.2",
    "dotenv": "^16.3.1"
  }},
  "devDependencies": {{
    "@types/express": "^4.17.21",
    "@types/node": "^20.9.0",
    "ts-node-dev": "^2.0.0",
    "typescript": "^5.2.2"
  }}
}}"#)
    ).unwrap();

    fs::write(
        project_path.join("tsconfig.json"),
        r#"{
  "compilerOptions": {
    "target": "es2020",
    "module": "commonjs",
    "outDir": "./dist",
    "rootDir": "./src",
    "strict": true,
    "esModuleInterop": true,
    "skipLibCheck": true,
    "forceConsistentCasingInFileNames": true
  },
  "include": ["src/**/*"],
  "exclude": ["node_modules"]
}"#
    ).unwrap();

    fs::write(
        project_path.join("src/index.ts"),
        r#"import express, { Request, Response } from 'express';
import dotenv from 'dotenv';

dotenv.config();

const app = express();
const port = process.env.PORT || 3000;

app.use(express.json());

app.get('/', (req: Request, res: Response) => {
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

# Build
dist/
build/

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
