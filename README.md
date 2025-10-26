# Devora

## Overview

Devora is a modern developer tool designed to provide a Vite-like experience for C++ projects. It delivers instant project scaffolding, fast incremental builds, and a smooth development experience with integrated tools such as testing, linting, and live reload capabilities.

Inspired by the simplicity and speed of frontend tools like Vite, Devora aims to bring that same level of productivity and developer experience to C++ development.

---

## Features

- **Instant Project Setup:** Quickly scaffold new C++ projects with standardized folder structures and build configurations using Meson + Ninja by default.
- **Fast Incremental Builds:** Efficiently rebuild only what has changed, reducing wait times dramatically.
- **Live Reload:** Automatically rebuild and restart your application on source file changes to speed up the feedback loop.
- **Integrated Developer Tools:** Out of the box support for unit testing, linting (clang-tidy/clang-format), and logging.
- **Dependency Management:** Compatible with package managers like vcpkg and Conan to easily manage external libraries.
- **Developer-Friendly CLI:** Easy-to-use commands for project creation, development mode, testing, and more with detailed terminal feedback.

---

## Installation

You can install Devora via cargo (Rust) or from prebuilt binaries (TBD on release):

```
cargo install devora-cli
```

Or download from the releases page (coming soon).

---

## Getting Started

### Create a New Project

```
devora create myapp
cd myapp
```

This generates a ready-to-build C++ project using Meson.

### Start Development Mode

```
devora dev
```

This command will watch for changes in your source files, build incrementally, and restart your app automatically.

---

## Usage

- `devora create <project_name>` — Scaffold a new project.
- `devora dev` — Start the development server with live rebuild and reload.
- `devora test` — Run unit tests automatically after build.
- `devora lint` — Run clang-tidy and clang-format checks.
- `devora build` — Perform a release build.

---

## Project Structure

```
myapp/
├── src/                 # Source files
├── tests/               # Unit tests
├── meson.build          # Build configuration
├── devora.toml          # Devora config file
└── README.md
```

---

## Contributing

Contributions are welcome! Feel free to fork, create issues, or submit pull requests.

Please follow the [code of conduct](CODE_OF_CONDUCT.md) and coding style guidelines.

---

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.

---

## Contact & Support

For questions or feedback, open an issue or reach out via email at support@devora.dev.

---

## Roadmap

- Enhanced hot-reload with dynamic library swapping.
- Plugin system for custom templates, linters, and test frameworks.
- IDE integrations and GUI dashboard.
- Extended support for additional build systems.

---

Thank you for choosing Devora — powering fast, modern C++ development.

```