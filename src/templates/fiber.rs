use std::fs;
use std::path::Path;

pub fn create_files(project_path: &Path, project_name: &str) {
    fs::write(
        project_path.join("main.go"),
        "package main\n\nimport \"github.com/gofiber/fiber/v2\"\n\nfunc main() {\n    app := fiber.New()\n\n    app.Get(\"/\", func(c *fiber.Ctx) error {\n        return c.JSON(fiber.Map{\n            \"message\": \"Hello World\",\n        })\n    })\n\n    app.Listen(\":3000\")\n}"
    ).unwrap();
    fs::write(
        project_path.join("go.mod"),
        format!("module {}\n\ngo 1.20", project_name)
    ).unwrap();
}
