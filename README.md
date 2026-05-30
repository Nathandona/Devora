<p align="center">
  <img src="assets/banner.svg" alt="Devora" width="100%">
</p>

<p align="center">
  <a href="https://github.com/Nathandona/Devora/releases/latest"><img src="https://img.shields.io/github/v/release/Nathandona/Devora?style=flat&label=release&labelColor=121214&color=3b3b42" alt="Release"></a>
  <a href="https://github.com/Nathandona/Devora/actions/workflows/ci.yml"><img src="https://img.shields.io/github/actions/workflow/status/Nathandona/Devora/ci.yml?branch=main&style=flat&label=ci&labelColor=121214" alt="CI"></a>
  <a href="https://github.com/Nathandona/Devora/releases"><img src="https://img.shields.io/github/downloads/Nathandona/Devora/total?style=flat&label=downloads&labelColor=121214&color=3b3b42" alt="Downloads"></a>
  <img src="https://img.shields.io/badge/platforms-macOS%20%7C%20Linux%20%7C%20Windows-3b3b42?style=flat&labelColor=121214" alt="Platforms">
  <a href="LICENSE"><img src="https://img.shields.io/github/license/Nathandona/Devora?style=flat&label=license&labelColor=121214&color=3b3b42" alt="License"></a>
</p>

<p align="center">
  A universal, modular project scaffolding framework.<br>
  Scaffold a project in any language from a single, self-contained binary.
</p>

<p align="center">
  <a href="#installation">Installation</a> &nbsp;&middot;&nbsp;
  <a href="#quick-start">Quick start</a> &nbsp;&middot;&nbsp;
  <a href="#usage">Usage</a> &nbsp;&middot;&nbsp;
  <a href="#agent--scripting-mode">Agent mode</a> &nbsp;&middot;&nbsp;
  <a href="#how-it-works">How it works</a> &nbsp;&middot;&nbsp;
  <a href="#contributing">Contributing</a>
</p>

---

## Installation

Devora ships prebuilt binaries. You do not need a Rust toolchain to use it.

**macOS and Linux**

```bash
curl -LsSf https://github.com/Nathandona/Devora/releases/latest/download/devora-installer.sh | sh
```

**Windows (PowerShell)**

```powershell
irm https://github.com/Nathandona/Devora/releases/latest/download/devora-installer.ps1 | iex
```

The installers are produced by [dist](https://github.com/axodotdev/cargo-dist) on
every tagged release. They detect your platform, download the right binary, and
add it to your PATH.

<details>
<summary>Install from source (requires Rust)</summary>

```bash
cargo install --git https://github.com/Nathandona/Devora
```

Plugins are embedded into the binary at build time, so the installed `devora`
works from any directory with no extra files.

</details>

## Quick start

```bash
devora new my-project rust
cd my-project
cargo run
```

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

Or scaffold a C++ project (CMake, with vendored doctest tests that build offline):

```bash
devora new my-app cpp
cd my-app
cmake -B build
cmake --build build
ctest --test-dir build
```

## Features

| Capability | What it gives you |
| --- | --- |
| Plugin architecture | Each language is a self-contained plugin (a manifest plus Tera templates). The engine stays generic. |
| Template engine | Tera templating with variable substitution and template inheritance. |
| Interactive or scripted | Prompt for required values, or run fully non-interactive. |
| Lifecycle hooks | Run formatters, git init, or installs after generation. Skippable with `--no-hooks`. |
| Self-contained binary | Plugins are baked in at build time via `include_dir!`. No runtime plugin directory. |
| Agent friendly | `--json` output and stable exit codes for scripting and CI. |

## Usage

### List languages

```bash
devora list
```

```
stable     rust          1 template
stable     c++           1 template
stable     go            1 template
stable     python        1 template
stable     c#            1 template
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
# Interactive (prompts for missing required values)
devora new my-app rust

# Non-interactive
devora new my-app rust --non-interactive

# Pick a framework
devora new my-app rust --framework=base

# Pass custom variables
devora new my-app rust --var="description=My project" --var="license=Apache-2.0"

# Preview without writing files
devora new my-app rust --dry-run

# Skip pre and post hooks
devora new my-app rust --non-interactive --no-hooks
```

### Inspect a language or framework

```bash
devora info rust
devora info rust base
```

## Agent / scripting mode

Every command accepts `--json` for machine-readable output and stable exit codes
(`0` on success, non-zero on error with a `{"error": "..."}` envelope on stderr).
Combined with `--non-interactive` and `--no-hooks`, this lets agents and CI drive
Devora deterministically.

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

## How it works

Devora uses a three-layer modularity system:

```
LANGUAGE  ->  FRAMEWORK  ->  TEMPLATE
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
description   = { description = "Project description", required = false }
license       = { description = "Project license", default = "MIT", required = false }
include_tests = { description = "Include tests", default = true, required = false }

[[post_hooks]]
command = "cargo fmt"
description = "Format code"
```

### Template files

Located at `plugins/{language}/frameworks/{framework}/templates/`:

- Files ending in `.tera` are processed with variable substitution.
- Other files are copied as-is.
- The full directory structure is preserved.
- Files named `base.*` and anything under `partials/` are used only for template inheritance and are not emitted.

### Template variables

Built-in variables available to every template:

| Variable | Value |
| --- | --- |
| `project_name` | The project name from the command line |
| `project_slug` | Sanitized project name for file paths |
| `author` | Author name from git config |
| `email` | Author email from git config |
| `date` | Current date (YYYY-MM-DD) |
| `year` | Current year |
| `framework` | Selected framework name |

Custom variables are declared in framework manifests and provided via interactive
prompts, `--var="key=value"`, or manifest defaults. Boolean-looking values
(`true` / `false`) passed via `--var` are coerced to real booleans so template
conditionals behave correctly.

## Adding new plugins

Plugins are embedded into the binary at build time, so adding a language or
framework means editing the `plugins/` tree and rebuilding (`cargo build`).

**Add a language**

1. Create `plugins/{language}/`
2. Add `plugins/{language}/manifest.toml`
3. Create at least one framework under `plugins/{language}/frameworks/`
4. Add the framework manifest and templates
5. Rebuild

**Add a framework**

1. Create `plugins/{language}/frameworks/{name}/`
2. Add its `manifest.toml`
3. Create the `templates/` directory and files
4. Declare variables and hooks as needed
5. Rebuild

## Development

```bash
git clone https://github.com/Nathandona/Devora.git
cd Devora
cargo build --release
cargo test
```

Project layout:

```
devora/
├── src/
│   ├── main.rs              CLI entry point
│   ├── lib.rs               Library interface
│   ├── cli.rs               Command-line interface
│   ├── core/
│   │   ├── registry.rs      Plugin discovery (from embedded tree)
│   │   ├── generator.rs     Template rendering and hooks
│   │   ├── context.rs       Template variable builder
│   │   ├── embedded.rs      Plugins baked into the binary (include_dir!)
│   │   └── roadmap.rs       Language status board for `devora list`
│   ├── models/              Manifest data structures
│   ├── commands/            new / list / info
│   ├── error.rs             Error types
│   └── utils/               Filesystem, git, executor helpers
├── plugins/                 Plugin tree (embedded at build time)
│   └── rust/
│       ├── manifest.toml
│       └── frameworks/base/
│           ├── manifest.toml
│           └── templates/
├── dist-workspace.toml      Release / installer config (dist)
├── Cargo.toml
└── README.md
```

## Roadmap

Devora ships Rust, C++, Go, Python, and C# today, each with a single working
template. Run `devora list` for the current status board. New languages and
additional frameworks land when they are ready, not when a calendar says so.

## Contributing

Contributions are welcome. Good places to start:

- New language plugins (TypeScript, Java, Ruby, Zig)
- Additional frameworks for the existing languages
- Documentation and guides
- Tests

## License

Licensed under the [MIT License](LICENSE).
