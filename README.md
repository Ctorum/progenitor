# 🌱 Progenitor v0.2.4 (beta)

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

## 🎯 Supported Templates

Currently supported templates:

- FastAPI: Python web framework for building APIs
- Fiber: Go web framework focused on performance
- Express: Node.js web application framework
- Terraform: Infrastructure as Code tool for building and managing infrastructure
- New ones: Coming soon...

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
