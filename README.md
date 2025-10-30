# Devora

<p align="center">
  <img src="https://img.shields.io/badge/rust-1.70+-orange.svg" alt="Rust Version">
  <img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="License">
  <img src="https://img.shields.io/badge/build-passing-brightgreen.svg" alt="Build Status">
</p>

<p align="center">
  <strong>A universal, modular project scaffolding framework</strong>
</p>

<p align="center">
  Devora combines the best aspects of Cargo, Create-React-App, and Cookiecutter into a single, extensible tool that can scaffold projects across any language and framework through a plugin-based architecture.
</p>

## 🚀 Quick Start

### Installation

```bash
cargo install devora
```

### Your First Project

Create a new Rust project in seconds:

```bash
devora new my-project rust
cd my-project
cargo run
```

Output:
```
🚀 Creating new rust project: my-project
📁 Framework: base
Executing hook: cargo fmt
✅ Project 'my-project' created successfully at: my-project

Next steps:
  cd my-project
  cargo run
  cargo build
```

## 📋 Features

- **🔌 Plugin-Based Architecture**: Easy to add new languages and frameworks
- **🎨 Template Engine**: Powerful Tera-based templating with variable substitution
- **⚡ Interactive Mode**: Smart prompts for required configuration
- **🔧 Hook System**: Pre/post-generation commands (git init, cargo fmt, etc.)
- **📦 Hot-Reloading**: No recompilation needed for new plugins
- **🌍 Multi-Language Support**: Extensible to any programming language
- **📁 Smart File Generation**: Handles nested directories and binary files

## 🎯 Usage

### List Available Languages

```bash
devora list
```

```
📋 Available Languages and Frameworks
=====================================
Available languages:
  rust - The Rust Programming Language
```

### List Frameworks for a Language

```bash
devora list rust
```

```
📋 Available Languages and Frameworks
=====================================
Frameworks for language: rust
  base
```

### Create a New Project

```bash
# Interactive mode (prompts for missing info)
devora new my-app rust

# Non-interactive mode
devora new my-app rust --non-interactive

# Specify framework
devora new my-app rust --framework=base

# Custom variables
devora new my-app rust --var="description=My awesome project" --var="license=Apache-2.0"

# Dry run (preview without creating files)
devora new my-app rust --dry-run
```

### Get Information

```bash
# Language information
devora info rust

# Framework information
devora info rust base
```

## 🏗️ Architecture

Devora uses a three-layer modularity system:

```
LANGUAGE → FRAMEWORK → TEMPLATE
```

### Language Plugins
Located at `plugins/{language}/manifest.toml`:

```toml
[language]
id = "rust"
name = "Rust"
version = "1.0.0"
description = "The Rust Programming Language"

default_framework = "base"
frameworks = ["base", "clap", "axum"]
```

### Framework Plugins
Located at `plugins/{language}/frameworks/{framework}/manifest.toml`:

```toml
[framework]
id = "base"
name = "Base"
version = "1.0.0"
description = "Basic Rust project structure"

[variables]
description = { description = "Project description", required = false }
license = { description = "Project license", default = "MIT", required = false }

[[post_hooks]]
command = "cargo fmt"
description = "Format code"

init_commands = [
    "cargo run",
    "cargo build"
]
```

### Template Files
Located at `plugins/{language}/frameworks/{framework}/templates/`:

- **Tera Templates**: Files ending in `.tera` are processed with variable substitution
- **Binary Files**: Other files are copied as-is
- **Nested Directories**: Full directory structure is preserved

## 🎨 Template Variables

### Built-in Variables
- `project_name`: The project name (from command line)
- `project_slug`: Sanitized project name for file paths
- `author`: Author name from git config
- `email`: Author email from git config
- `date`: Current date (YYYY-MM-DD)
- `year`: Current year
- `framework`: Selected framework name

### Custom Variables
Defined in framework manifests and can be provided via:
- Interactive prompts
- Command line arguments (`--var="key=value"`)
- Environment variables
- Template references

### Template Example

`src/main.rs.tera`:
```rust
{% if framework == "clap" %}
use clap::Parser;

#[derive(Parser)]
#[command(name = "{{ project_slug }}")]
#[command(about = "{{ project_name }}")]
{% if description %}
#[command(long_about = "{{ description }}")]
{% endif %}
struct Cli {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,
}

fn main() {
    let cli = Cli::parse();
    println!("Hello, {}!", cli.name);
}
{% else %}
fn main() {
    println!("Hello, world!");
}
{% endif %}
```

`Cargo.toml.tera`:
```toml
[package]
name = "{{ project_slug }}"
version = "0.1.0"
edition = "2021"
{% if author %}
authors = ["{{ author }}{% if email %} <{{ email }}>{% endif %}"]
{% endif %}
{% if description %}
description = "{{ description }}"
{% endif %}
{% if license %}
license = "{{ license }}"
{% endif %}

[dependencies]
{% if framework == "clap" %}
clap = { version = "4.4", features = ["derive"] }
{% endif %}
```

## 🔌 Adding New Plugins

### Adding a New Language

1. Create language directory: `plugins/python/`
2. Add language manifest: `plugins/python/manifest.toml`
3. Create at least one framework: `plugins/python/frameworks/base/`
4. Add framework manifest and templates

### Adding a New Framework

1. Create framework directory: `plugins/rust/frameworks/axum/`
2. Add framework manifest: `plugins/rust/frameworks/axum/manifest.toml`
3. Create templates directory and files
4. Define framework-specific variables
5. Add post-generation hooks if needed

### Example: Python Flask Framework

`plugins/python/frameworks/flask/manifest.toml`:
```toml
[framework]
id = "flask"
name = "Flask"
version = "1.0.0"
description = "Flask web framework"

[variables]
app_name = { description = "Flask app name", default = "app", required = false }
database = { description = "Database type", default = "sqlite", required = false }

[[post_hooks]]
command = "python -m venv venv"
description = "Create virtual environment"

[[post_hooks]]
command = "source venv/bin/activate && pip install flask"
description = "Install Flask"
```

## 🛠️ Development

### Building from Source

```bash
git clone https://github.com/yourusername/devora.git
cd devora
cargo build --release
```

### Running Tests

```bash
cargo test
```

### Project Structure

```
devora/
├── src/
│   ├── main.rs              # CLI entry point
│   ├── lib.rs               # Library interface
│   ├── cli.rs               # Command-line interface definitions
│   ├── core/                # Core functionality
│   │   ├── registry.rs      # Plugin discovery and management
│   │   ├── generator.rs     # Template rendering engine
│   │   └── context.rs       # Template variable builder
│   ├── models/              # Data structures
│   │   ├── language.rs      # Language manifest types
│   │   ├── framework.rs     # Framework manifest types
│   │   └── template.rs      # Template metadata types
│   ├── commands/            # CLI command implementations
│   │   ├── new.rs           # Project creation
│   │   ├── list.rs          # List languages/frameworks
│   │   └── info.rs          # Show information
│   ├── error.rs             # Custom error types
│   └── utils/               # Utility functions
├── plugins/                 # Plugin directory
│   └── rust/                # Rust language plugin
│       ├── manifest.toml    # Language manifest
│       └── frameworks/      # Framework implementations
│           └── base/        # Base framework
│               ├── manifest.toml
│               └── templates/
├── Cargo.toml
└── README.md
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### How to Contribute

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Areas for Contribution

- **New Language Plugins**: Add support for Python, JavaScript, Go, etc.
- **Framework Templates**: Create templates for popular frameworks
- **Documentation**: Improve this README and add guides
- **Tests**: Add unit and integration tests
- **Features**: Implement advanced features from the roadmap

## 🗺️ Roadmap

### Phase 1: Core Features ✅
- [x] Basic plugin loading
- [x] Template rendering with Tera
- [x] Simple manifest system
- [x] File generation
- [x] Basic error handling

### Phase 2: Enhanced UX (In Progress)
- [x] Interactive variable prompts
- [x] Better error messages with suggestions
- [x] List and info commands
- [x] Dry-run mode
- [ ] Progress indicators
- [ ] Colored output

### Phase 3: Advanced Features
- [ ] Pre-generation hooks (validation, checks)
- [ ] Conditional file inclusion
- [ ] Template inheritance
- [ ] Variable validation
- [ ] Git integration

### Phase 4: Remote Capabilities
- [ ] Remote template registry
- [ ] GitHub/GitLab template sources
- [ ] Template versioning
- [ ] Automatic updates
- [ ] Community template marketplace

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- **Tera** - Powerful template engine
- **Clap** - Command line argument parsing
- **Serde** - Serialization framework
- **Dialoguer** - Interactive prompts
- Inspired by **Cargo**, **Create-React-App**, and **Cookiecutter**

## 🔗 Links

- [Repository](https://github.com/yourusername/devora)
- [Crates.io](https://crates.io/crates/devora)
- [Documentation](https://docs.rs/devora)
- [Issues](https://github.com/yourusername/devora/issues)

---

<p align="center">
  <strong>Devora - Scaffold any project, any language, any framework. 🚀</strong>
</p>