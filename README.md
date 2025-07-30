# 🌱 Progenitor v0.3.0 (beta)

Now you can create and install your own templates.

**Progenitor** is a fast and efficient Rust CLI tool designed to generate project templates for a variety of frameworks and application types. With Progenitor, you can quickly bootstrap projects for frameworks like **FastAPI**, **Express**, **Fiber**, **microservices**, and many more with a simple command. 🚀

## ✨ Features

- 📦 Generate ready-to-use project templates for multiple frameworks
- 🏗️ Quickly bootstrap microservices architectures
- 💻 Easy-to-use CLI interface
- 🎨 Customizable template options

## 🛠️ Installation

To install **Progenitor** using Cargo:

```bash
cargo install progenitor-cli
```

To install **Progenitor** using Homebrew:

```bash
brew tap ctorum/progenitor
brew install progenitor
```

## 🚀 Basic Usage

Use the `progenitor` command in your terminal.

```bash
progenitor create -t <template> -n <project_name> <path>
```

or

```bash
pgen create -t <template> -n <project_name> <path>
```

## ✨ CLI Features

Progenitor v0.3.0 introduces significant enhancements to its CLI, making template management more flexible and powerful.

### New Commands:
- `add`: This new command allows you to add custom templates directly from a local path or a Git repository. This means you can now easily integrate your own project structures or those shared by others into your Progenitor workflow. All your custom templates will be stored in your home directory at `~/.progenitor/templates`.

- `remove`: The `remove` command allows you to remove the custom template from your local collection. This ensures that your Progenitor setup remains clean and organized. Also you can use the flag --all to remove all custom templates at once.

### Changes to Existing Commands:

#### `create` command:
The `create` command now supports using both built-in and custom templates. When specifying a template, Progenitor will first look for a matching custom template before falling back to its built-in collection. This provides a seamless experience for using your own templates.

#### `list` command:
The `list` command has been updated to display both built-in and custom templates, providing a comprehensive overview of all available templates on your system. Custom templates are clearly marked to distinguish them from the default ones.

## 🔮 Upcoming Features

We are continuously working to expand Progenitor's capabilities. Here's a sneak peek at what's coming next:

- `--recursive` flag for `add`: Soon, the `add` command will support a `--recursive` flag, allowing you to add multiple projects at once inside a unique folder. This will streamline the process of setting up complex multi-project repositories. Additionally, the `add` command will gain the ability to automatically import templates directly from `--uri` (for URLs) or `--git` (for Git repositories), simplifying the process of adding remote templates.
- `integrate` command: A powerful new `integrate` command is on the horizon. This command will enable you to change the architecture of existing projects (e.g., applying Clean Architecture, Onion Architecture, Domain-Driven Design (DDD), or SOLID principles). Additionally, it will facilitate changing project settings and dependency managers, such as switching from `pip` to `uv` or `poetry`, or from `npm` to `yarn` or `pnpm`.

If you have any ideas for new features, improvements, or encounter any issues, feel free to discuss them in the [Discussions tab](https://github.com/cttorum/progenitor/discussions) of the Progenitor repository!

## 📝 Custom Template Example

Progenitor now empowers you to create and install your own project templates. Here's a simple example using a `my-test-template`:

### Understanding Template Structure

Before adding your custom template, it's crucial to understand its expected structure and the role of special files:

-   **Template Root Directory**: This is the main folder for your template (e.g., `my-test-template/`). It should contain all the files and directories that will be part of your generated project.
-   **`generator.yml`**: This file, placed at the root of your template, allows you to define variables that Progenitor will prompt the user for during project creation. These variables can then be used within your template files. For example, for `my-test-template`:

    ```yaml
    # my-test-template/generator.yml
    folder-app:
      description: "A simple Python application template with a subdirectory."
      variables:
        - name: author
          type: string
          default: "Roo"
          prompt: "Enter the authors name:"
      structure:
        - type: directory
          name: "src"
          content:
            - type: file
              name: "main.py"
              source: "src/main.py.gen"
        - type: file
          name: "README.md"
          source: "README.py.gen"
    ```
    This `generator.yml` defines an `author` variable and specifies the project structure, including a `src` directory and a `README.md` file sourced from `README.py.gen`.

    Let's break down the fields within `generator.yml`:
    -   **`folder-app`**: This is the unique identifier (template name) for your template. When a user runs `progenitor create -t <template_name>`, this is the name they will use.
    -   **`description`**: A brief explanation of what this template does or what kind of project it generates. This is useful for users to understand the template's purpose.
    -   **`variables`**: An array of objects, where each object defines a variable that Progenitor will prompt the user for.
        -   **`name`**: The name of the variable (e.g., `author`). This name will be used as a placeholder in your `.gen` files (e.g., `${author}`).
        -   **`type`**: The data type of the variable (e.g., `string`).
        -   **`default`**: (Optional) A default value for the variable. If the user doesn't provide input, this value will be used. This is useful for common values or to provide a fallback.
        -   **`prompt`**: (Optional) The message displayed to the user when prompting for this variable's value. This is crucial for guiding the user. **For sensitive information (e.g., API keys, passwords), you can use a `prompt` without a `default` value to ensure the user explicitly enters the information, preventing it from being stored in the template itself.**
    -   **`structure`**: An array defining the file and directory structure of the generated project.
        -   **`type`**: Can be `directory` or `file`.
        -   **`name`**: The name of the directory or file. For directories, you can use variable placeholders like `{{project_name}}` which will be replaced by the actual project name.
        -   **`content`**: (Only for `directory` type) An array of nested `type` and `name` definitions for files and subdirectories within this directory.
        -   **`source`**: (Only for `file` type) The path to the source `.gen` file within your template directory. This file's content will be processed and copied to the generated project.

-   **`gen` files**: Any file or directory within your template that ends with `.gen` will be processed by Progenitor's templating engine. This means you can embed variables (e.g., `${project_name}`, `${author}`) directly into your file names or content. The `.gen` extension will be removed after processing. For example, `README.py.gen` would become `README.md` with variables replaced.
    Consider the `my-test-template/README.py.gen` file:

    ```
    # ${project_name}

    This is a simple Python application generated by Progenitor, with a subdirectory.

    Author: ${author}
    ```
    Here, `${project_name}` and `${author}` are placeholders that Progenitor will replace with the actual project name and the author's name (prompted from `generator.yml`).

-   **Variable Placeholders**: Use `${variable_name}` to embed variables defined in `generator.yml` or built-in variables (like `${project_name}` which is automatically provided by the `create` command) within your `.gen` files.

### 1. Create your template structure:

Organize your template files and directories as you would any project. For instance, the `my-test-template` structure looks like this:

```
my-test-template/
├── README.py.gen
├── generator.yml
└── src/
```

In this example:
- `generator.yml` defines the `author` variable and the project's file structure.
- `README.py.gen` contains placeholders like `${project_name}` and `${author}` that Progenitor replaces during generation.
- The `src/` directory is created as specified in `generator.yml`.

### 2. Add your template to Progenitor:

Once your template is ready, use the `add` command to make it available:

```bash
progenitor add --path /path/to/your/my-test-template
```
or if it's a git repository:
```bash
progenitor add --git https://github.com/your-user/my-test-template.git
```

### 3. Use your custom template:

Now you can create new projects using your custom template just like any built-in one:

```bash
progenitor create -t folder-app -n my-new-project ./my-new-project
```
Note that `-t folder-app` refers to the `folder-app` key defined in `generator.yml`.

This flexibility allows you to standardize your team's project setups or quickly bootstrap personal projects with your preferred configurations.

## 📋 Contributing

If you want to contribute to **Progenitor**, please follow these steps:

1. Fork the repository.

2. Create a new branch for your feature or bug fix.

```bash
git checkout -b feature/your-feature
```

3. Build **Progenitor** from source:

```bash
cargo build --release
```

4. After building, you can find the binary in target/release/. To run it:

```bash
./target/release/progenitor
```

5. Commit your changes and push them to your forked repository.

```bash
git add .
git commit -m "Add your commit message here"
git push origin feature/your-feature
```

6. Create a pull request to the main repository.
