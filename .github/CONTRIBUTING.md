# Contributing to Devora

Thank you for your interest in contributing to Devora! This document provides guidelines and information for contributors.

## 🚀 Getting Started

### Prerequisites

- Rust 1.70 or higher
- Git
- Basic knowledge of Rust and command-line tools

### Development Setup

1. **Fork the Repository**
   ```bash
   # Fork the repository on GitHub, then clone your fork
   git clone https://github.com/Nathandona/Devora.git
   cd Devora
   ```

2. **Set Up Development Environment**
   ```bash
   # Install Rust dependencies
   cargo build

   # Run tests to ensure everything works
   cargo test

   # Install development tools
   cargo install cargo-watch cargo-audit cargo-outdated
   ```

3. **Create a Feature Branch**
   ```bash
   git checkout -b feature/your-feature-name
   ```

## 🏗️ Project Structure

```
Devora/
├── src/                    # Core source code
│   ├── main.rs            # CLI entry point
│   ├── cli.rs             # Command-line interface definitions
│   ├── core/              # Core functionality
│   ├── models/            # Data structures
│   ├── commands/          # CLI command implementations
│   ├── utils/             # Utility functions
│   └── error.rs           # Error handling
├── plugins/               # Language and framework plugins
│   └── rust/              # Rust language plugin
├── tests/                 # Integration and unit tests
├── scripts/               # Build and deployment scripts
├── docker/                # Docker configurations
└── .github/               # GitHub workflows and templates
```

## 🧪 Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with coverage
./scripts/test.sh coverage

# Run integration tests only
./scripts/test.sh integration

# Run specific test
cargo test test_name
```

### Writing Tests

- **Unit Tests**: Test individual functions and modules in `src/`
- **Integration Tests**: Test end-to-end functionality in `tests/`
- **CLI Tests**: Test command-line interface behavior

## 📝 Code Style

### Rust Guidelines

- Follow the official [Rust style guide](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` for code formatting
- Use `cargo clippy` for linting

```bash
# Format code
cargo fmt

# Run linter
cargo clippy -- -D warnings
```

### Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/) specification:

- `feat:` - New feature
- `fix:` - Bug fix
- `docs:` - Documentation changes
- `style:` - Code style changes (formatting, etc.)
- `refactor:` - Code refactoring
- `test:` - Test additions/changes
- `chore:` - Maintenance tasks
- `ci:` - CI/CD related changes

Examples:
```
feat: add support for Python Flask framework
fix: resolve template rendering issue with nested variables
docs: update README with installation instructions
```

## 🔌 Adding Plugins

### Plugin Structure

Plugins follow a three-layer structure: `LANGUAGE → FRAMEWORK → TEMPLATE`

1. **Language Plugin** (`plugins/{language}/manifest.toml`)
2. **Framework Plugin** (`plugins/{language}/frameworks/{framework}/manifest.toml`)
3. **Template Files** (`plugins/{language}/frameworks/{framework}/templates/`)

### Adding a New Language

1. Create language directory: `plugins/{language}/`
2. Add language manifest: `plugins/{language}/manifest.toml`
3. Create at least one framework: `plugins/{language}/frameworks/{framework}/`
4. Add framework manifest and templates
5. Write tests for the new plugin

### Adding a New Framework

1. Create framework directory: `plugins/{language}/frameworks/{framework}/`
2. Add framework manifest: `plugins/{language}/frameworks/{framework}/manifest.toml`
3. Create templates directory and files
4. Define framework-specific variables
5. Add post-generation hooks if needed

## 🐛 Bug Reports

When reporting bugs:

1. Use the bug report template
2. Provide reproduction steps
3. Include environment information
4. Add relevant logs and screenshots

## ✨ Feature Requests

When requesting features:

1. Use the feature request template
2. Describe the problem you're solving
3. Provide use cases and examples
4. Consider implementation complexity

## 🔄 Pull Request Process

### Before Submitting

1. **Run Tests**
   ```bash
   ./scripts/test.sh
   ```

2. **Check Code Quality**
   ```bash
   cargo fmt
   cargo clippy
   ```

3. **Update Documentation**
   - Update README.md if needed
   - Add comments to complex code
   - Update plugin documentation

4. **Commit Changes**
   ```bash
   git add .
   git commit -m "feat: add your feature description"
   ```

### Submitting PRs

1. **Push to Your Fork**
   ```bash
   git push origin feature/your-feature-name
   ```

2. **Create Pull Request**
   - Use a descriptive title
   - Fill out the PR template
   - Link related issues
   - Request review from maintainers

### PR Review Process

- Automated checks must pass
- Code review from maintainers
- Tests must pass
- Documentation must be updated

## 🚀 Release Process

Releases are automated using semantic versioning:

1. **Commits** are automatically analyzed for version bumps
2. **Release PRs** are created with changelogs
3. **Artifacts** are built for multiple platforms
4. **Published** to GitHub releases, crates.io, Docker Hub

### Making a Release

```bash
# Automated release (uses conventional commits)
git commit -m "feat: add new feature"

# Manual release (if needed)
./scripts/release.sh patch  # or minor, major
```

## 📚 Documentation

### Types of Documentation

- **README.md**: Project overview and quick start
- **Inline Comments**: Code explanations
- **Plugin Docs**: Framework/language usage
- **API Docs**: Generated from code comments

### Writing Documentation

- Use clear, concise language
- Provide code examples
- Include troubleshooting sections
- Keep documentation up to date

## 🤝 Community Guidelines

### Code of Conduct

- Be respectful and inclusive
- Welcome newcomers
- Provide constructive feedback
- Focus on technical merit

### Getting Help

- **GitHub Discussions**: Questions and ideas
- **Issues**: Bug reports and feature requests
- **Documentation**: Check README and inline docs

## 🔧 Development Tools

### Recommended VS Code Extensions

- rust-analyzer
- CodeLLDB
- Better TOML
- GitLens

### Useful Cargo Commands

```bash
# Watch for changes and recompile
cargo watch -x run

# Find unused dependencies
cargo machete

# Check for outdated dependencies
cargo outdated

# Audit dependencies for security issues
cargo audit

# Generate documentation
cargo doc --open
```

## 📊 Performance

### Guidelines

- Profile with `cargo flamegraph` for performance issues
- Use `cargo bench` for benchmarking
- Consider memory usage and binary size
- Test on multiple platforms

## 🏆 Recognition

Contributors are recognized in:

- README.md contributors section
- Release notes
- GitHub contributor statistics

Thank you for contributing to Devora! 🎉