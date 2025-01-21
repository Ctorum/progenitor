# 🌱 Progenitor v0.2-beta

**Progenitor** is a fast and efficient Rust CLI tool designed to generate project templates for a variety of frameworks and application types. With Progenitor, you can quickly bootstrap projects for frameworks like **FastAPI**, **React**, **React Native**, **Fiber**, **microservices**, and many more with a simple command. 🚀

## ✨ Features

- 📦 Generate ready-to-use project templates for multiple frameworks
- 🏗️ Quickly bootstrap microservices architectures
- 💻 Easy-to-use CLI interface
- 🎨 Customizable template options

## 🛠️ Installation

To install **Progenitor** globally, you'll need to build the project from source:

```bash
cargo build --release
```

After building, you can find the binary in target/release/. To run it:

```bash
./target/release/progenitor
```

## 🎯 Supported Templates

Currently supported templates:

- FastAPI: Python web framework for building APIs
- Fiber: Go web framework focused on performance

## 📋 Usage

Create a new project using a template using `progenitor` or the shorthand `pgen`:

```bash
./target/release/progenitor create -t <template> -n <project_name> <path>
```

or

```bash
./target/release/pgen create -t <template> -n <project_name> <path>
```

List all available templates:

```bash
./target/release/progenitor list
```

or

```bash
./target/release/pgen list
```
