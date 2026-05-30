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
  Devora combines the best aspects of Cargo, Create-React-App, and Cookiecutter into a single, extensible tool that scaffolds projects across languages and frameworks through a plugin-based architecture.
</p>

## Quick Start

### Installation

Devora ships prebuilt binaries — you do **not** need a Rust toolchain to use it.

macOS / Linux:

```bash
curl -LsSf https://github.com/Nathandona/Devora/releases/latest/download/devora-installer.sh | sh
```

Windows (PowerShell):

```powershell
irm https://github.com/Nathandona/Devora/releases/latest/download/devora-installer.ps1 | iex
```

These installers (built by [dist](https://github.com/axodotdev/cargo-dist) on
each tagged release) download the right binary for your platform and put it on
your PATH.

<details>
<summary>From source (requires Rust)</summary>

```bash
cargo install --git https://github.com/Nathandona/Devora
```

</details>

### Your First Project

```bash
devora new my-project rust
cd my-project
cargo run
```

Output:

```
Resolving plugin: rust@0.1.0
Rendering templates …
my-project
├─ .gitignore
├─ Cargo.toml
├─ README.md
└─ src
   └─ main.rs
created my-project in 0.42s
```

## Features

- **Plugin-based architecture**: each language is a self-contained plugin (a manifest plus templates); the engine stays generic.
- **Template engine**: Tera-based templating with variable substitution and inheritance.
- **Interactive mode**: prompts for required configuration, or run fully non-interactive.
- **Hook system**: pre/post-generation commands (git init, cargo fmt, …), skippable with `--no-hooks`.
- **Self-contained binary**: plugins are baked in at build time via `include_dir!` — no runtime plugin directory required.
- **Agent-friendly**: `--json` output and stable exit codes for scripting and CI.

## Usage

### List languages

```bash
devora list
```

```
stable     rust          1 template
paused     c++           templates being rethought
wishlist   go            open to contributions
wishlist   python        open to contributions
wishlist   typescript    open to contributions
wishlist   zig           open to contributions
```

### List frameworks for a language

```bash
devora list rust
```

```
base
```

### Create a new project

```bash
# Interactive mode (prompts for missing required info)
devora new my-app rust

# Non-interactive mode
devora new my-app rust --non-interactive

# Specify framework
devora new my-app rust --framework=base

# Custom variables
devora new my-app rust --var="description=My awesome project" --var="license=Apache-2.0"

# Dry run (preview without creating files)
devora new my-app rust --dry-run

# Skip pre/post hooks (formatters, git init)
devora new my-app rust --non-interactive --no-hooks
```

### Get information

```bash
# Language information
devora info rust

# Framework information
devora info rust base
```

## Agent / scripting mode

Every command accepts `--json` for machine-readable output and stable exit
codes (`0` on success, non-zero on error with a `{"error": "..."}` envelope on
stderr). Combined with `--non-interactive` and `--no-hooks`, this lets agents
and CI drive Devora deterministically.

```bash
devora list --json
devora info rust --json
devora new my-app rust --json --non-interactive --no-hooks
```

```json
{
  "name": "my-app",
  "language": "rust",
  "framework": "base",
  "path": "my-app",
  "files": [".gitignore", "Cargo.toml", "README.md", "src/main.rs"],
  "dry_run": false,
  "hooks_skipped": true,
  "elapsed_ms": 4
}
```

`--json` implies `--non-interactive` (machine mode never prompts).

## Architecture

Devora uses a three-layer modularity system:

```
LANGUAGE -> FRAMEWORK -> TEMPLATE
```

### Language plugins

Located at `plugins/{language}/manifest.toml`:

```toml
[language]
id = "rust"
name = "Rust"
version = "0.1.0"
description = "The Rust Programming Language"

default_framework = "base"
frameworks = ["base"]
```

### Framework plugins

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
include_tests = { description = "Include tests", default = true, required = false }

[[post_hooks]]
command = "cargo fmt"
description = "Format code"
```

### Template files

Located at `plugins/{language}/frameworks/{framework}/templates/`:

- **Tera templates**: files ending in `.tera` are processed with variable substitution.
- **Binary files**: other files are copied as-is.
- **Nested directories**: the full directory structure is preserved.
- Files named `base.*` and anything under `partials/` are used only for template inheritance/includes and are not emitted.

## Template variables

### Built-in variables

- `project_name`: the project name (from the command line)
- `project_slug`: sanitized project name for file paths
- `author`: author name from git config
- `email`: author email from git config
- `date`: current date (YYYY-MM-DD)
- `year`: current year
- `framework`: selected framework name

### Custom variables

Defined in framework manifests and provided via:

- Interactive prompts
- Command-line arguments (`--var="key=value"`)
- Defaults declared in the manifest

Boolean-looking values (`true`/`false`) passed via `--var` are coerced to real
booleans so template conditionals behave correctly.

### Template example

`src/main.rs.tera`:

```rust
fn main() {
    println!("Hello from {{ project_name }}!");
}
{% if include_tests %}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
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
```

## Adding new plugins

Plugins are embedded into the binary at build time, so adding a language or
framework means editing the `plugins/` tree and rebuilding (`cargo build`).

### Add a new language

1. Create the language directory: `plugins/python/`
2. Add the language manifest: `plugins/python/manifest.toml`
3. Create at least one framework: `plugins/python/frameworks/base/`
4. Add the framework manifest and templates
5. Rebuild

### Add a new framework

1. Create the framework directory: `plugins/rust/frameworks/<name>/`
2. Add the framework manifest: `plugins/rust/frameworks/<name>/manifest.toml`
3. Create the templates directory and files
4. Define framework-specific variables and hooks as needed
5. Rebuild

## Development

### Building from source

```bash
git clone https://github.com/Nathandona/Devora.git
cd Devora
cargo build --release
```

> Plugins under `plugins/` are embedded into the binary at build time via
> `include_dir!`, so the compiled `devora` is fully self-contained.

### Running tests

```bash
cargo test
```

### Project structure

```
devora/
├── src/
│   ├── main.rs              # CLI entry point
│   ├── lib.rs               # Library interface
│   ├── cli.rs               # Command-line interface definitions
│   ├── core/                # Core functionality
│   │   ├── registry.rs      # Plugin discovery (from embedded tree)
│   │   ├── generator.rs     # Template rendering engine + hooks
│   │   ├── context.rs       # Template variable builder
│   │   ├── embedded.rs      # Plugins baked into the binary (include_dir!)
│   │   └── roadmap.rs       # Language status board for `devora list`
│   ├── models/              # Data structures
│   ├── commands/            # CLI command implementations (new/list/info)
│   ├── error.rs             # Custom error types
│   └── utils/               # Utility functions
├── plugins/                 # Plugin directory (embedded at build time)
│   └── rust/
│       ├── manifest.toml
│       └── frameworks/base/
│           ├── manifest.toml
│           └── templates/
├── Cargo.toml
└── README.md
```

## Contributing

Contributions are welcome. Please feel free to open an issue or submit a pull
request.

Good places to start:

- **New language plugins**: add support for Go, Python, TypeScript, Zig, etc.
- **Framework templates**: create templates for popular frameworks.
- **Documentation**: improve this README and add guides.
- **Tests**: extend the unit and integration test suite.

## Roadmap

Devora ships Rust today. C++ is paused while its templates are reworked. Other
languages are on the wishlist and open to contributions — there is no fixed
calendar. See `devora list` for the current status board.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- **Tera** - template engine
- **Clap** - command-line argument parsing
- **Serde** - serialization framework
- **Dialoguer** - interactive prompts
- Inspired by **Cargo**, **Create-React-App**, and **Cookiecutter**

## Links

- [Repository](https://github.com/Nathandona/Devora)
- [Issues](https://github.com/Nathandona/Devora/issues)

---

<p align="center">
  <strong>Devora - scaffold any project, any language, any framework.</strong>
</p>
