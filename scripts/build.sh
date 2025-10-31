#!/bin/bash
# Devora Build Script
# Automates the build process for multiple targets and platforms

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
PROJECT_NAME="devora"
VERSION=${VERSION:-"0.1.0"}
BUILD_DIR="target"
RELEASE_DIR="release"
CARGO_TOML="Cargo.toml"

# Targets to build
TARGETS=(
    "x86_64-unknown-linux-gnu"
    "x86_64-unknown-linux-musl"
    "aarch64-unknown-linux-gnu"
    "x86_64-apple-darwin"
    "aarch64-apple-darwin"
    "x86_64-pc-windows-msvc"
)

# Utility functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if required tools are installed
check_dependencies() {
    log_info "Checking dependencies..."

    local missing_deps=()

    if ! command -v cargo &> /dev/null; then
        missing_deps+=("cargo")
    fi

    if ! command -v rustc &> /dev/null; then
        missing_deps+=("rustc")
    fi

    if ! command -v git &> /dev/null; then
        missing_deps+=("git")
    fi

    if [ ${#missing_deps[@]} -ne 0 ]; then
        log_error "Missing dependencies: ${missing_deps[*]}"
        exit 1
    fi

    log_success "All dependencies found"
}

# Clean previous builds
clean_build() {
    log_info "Cleaning previous builds..."
    rm -rf "$RELEASE_DIR"
    rm -rf "$BUILD_DIR/release"
    cargo clean
    log_success "Build directory cleaned"
}

# Install Rust targets
install_targets() {
    log_info "Installing Rust targets..."

    for target in "${TARGETS[@]}"; do
        log_info "Installing target: $target"
        rustup target add "$target" || log_warning "Failed to install target: $target"
    done

    log_success "Rust targets installed"
}

# Install additional tools
install_tools() {
    log_info "Installing additional tools..."

    # Install cargo-zigbuild for cross-compilation
    if ! command -v cargo-zigbuild &> /dev/null; then
        cargo install cargo-zigbuild
    fi

    # Install strip for cross-compilation if needed
    if command -v apt-get &> /dev/null; then
        sudo apt-get update && sudo apt-get install -y binutils-multiarch || true
    fi

    log_success "Additional tools installed"
}

# Build for a specific target
build_target() {
    local target=$1
    log_info "Building for target: $target"

    case "$target" in
        *-windows-*)
            cargo build --release --target "$target"
            ;;
        *-musl)
            cargo zigbuild --release --target "$target"
            ;;
        *-linux-*)
            if command -v zig &> /dev/null; then
                cargo zigbuild --release --target "$target"
            else
                cargo build --release --target "$target"
            fi
            ;;
        *-darwin-*)
            cargo build --release --target "$target"
            ;;
        *)
            cargo build --release --target "$target"
            ;;
    esac

    log_success "Build completed for $target"
}

# Create release package
create_package() {
    local target=$1
    local binary_name="$PROJECT_NAME"

    # Adjust binary name for Windows
    if [[ "$target" == *-windows-* ]]; then
        binary_name="$PROJECT_NAME.exe"
    fi

    local package_name="$PROJECT_NAME-$target-v$VERSION"
    local package_dir="$RELEASE_DIR/$package_name"

    log_info "Creating package for $target"

    mkdir -p "$package_dir"

    # Copy binary
    cp "$BUILD_DIR/$target/release/$binary_name" "$package_dir/"

    # Copy additional files
    cp README.md "$package_dir/" 2>/dev/null || true
    cp LICENSE "$package_dir/" 2>/dev/null || true

    # Generate shell completions
    if command -v "$package_dir/$PROJECT_NAME" &> /dev/null; then
        mkdir -p "$package_dir/completion"
        "$package_dir/$PROJECT_NAME" --generate-completion bash > "$package_dir/completion/$PROJECT_NAME.bash" 2>/dev/null || true
        "$package_dir/$PROJECT_NAME" --generate-completion fish > "$package_dir/completion/$PROJECT_NAME.fish" 2>/dev/null || true
        "$package_dir/$PROJECT_NAME" --generate-completion zsh > "$package_dir/completion/_$PROJECT_NAME" 2>/dev/null || true
    fi

    # Create archive
    cd "$RELEASE_DIR"

    if [[ "$target" == *-windows-* ]]; then
        7z a "$package_name.zip" "$package_name"
        log_success "Created $package_name.zip"
    else
        tar czf "$package_name.tar.gz" "$package_name"
        log_success "Created $package_name.tar.gz"
    fi

    # Generate checksums
    if [[ "$target" == *-windows-* ]]; then
        if command -v certutil &> /dev/null; then
            certutil -hashfile "$package_name.zip" SHA256 > "$package_name.zip.sha256"
        else
            sha256sum "$package_name.zip" > "$package_name.zip.sha256"
        fi
    else
        sha256sum "$package_name.tar.gz" > "$package_name.tar.gz.sha256"
    fi

    cd - > /dev/null
}

# Main build function
main() {
    log_info "Starting Devora build process"
    log_info "Version: $VERSION"

    check_dependencies
    clean_build
    install_targets
    install_tools

    mkdir -p "$RELEASE_DIR"

    # Build for each target
    for target in "${TARGETS[@]}"; do
        build_target "$target"
        create_package "$target"
    done

    # Create a summary
    log_info "Build summary:"
    ls -la "$RELEASE_DIR"/*.*

    log_success "Build process completed successfully!"
    log_info "Release artifacts are available in: $RELEASE_DIR"
}

# Script entry point
case "${1:-}" in
    "clean")
        clean_build
        ;;
    "targets")
        install_targets
        ;;
    "help"|"-h"|"--help")
        echo "Devora Build Script"
        echo ""
        echo "Usage: $0 [command]"
        echo ""
        echo "Commands:"
        echo "  clean    - Clean build artifacts"
        echo "  targets  - Install Rust targets only"
        echo "  help     - Show this help message"
        echo ""
        echo "Environment Variables:"
        echo "  VERSION  - Override version (default: 0.1.0)"
        ;;
    *)
        main
        ;;
esac