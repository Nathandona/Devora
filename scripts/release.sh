#!/bin/bash
# Devora Release Script
# Automates the release process for Devora

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
PROJECT_NAME="devora"
GITHUB_REPO="Nathandona/Devora"
RELEASE_DIR="release"
CHANGELOG_FILE="CHANGELOG.md"
VERSION_FILE="Cargo.toml"

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
    log_info "Checking release dependencies..."

    local missing_deps=()

    if ! command -v git &> /dev/null; then
        missing_deps+=("git")
    fi

    if ! command -v cargo &> /dev/null; then
        missing_deps+=("cargo")
    fi

    if ! command -v gh &> /dev/null; then
        missing_deps+=("gh (GitHub CLI)")
    fi

    if [ ${#missing_deps[@]} -ne 0 ]; then
        log_error "Missing dependencies: ${missing_deps[*]}"
        exit 1
    fi

    # Check if we're authenticated with GitHub
    if ! gh auth status &> /dev/null; then
        log_error "Not authenticated with GitHub. Run 'gh auth login' first."
        exit 1
    fi

    log_success "All dependencies found and authenticated"
}

# Validate repository state
validate_repository() {
    log_info "Validating repository state..."

    # Check if we're on main branch
    local current_branch=$(git rev-parse --abbrev-ref HEAD)
    if [ "$current_branch" != "main" ]; then
        log_error "You must be on the main branch to create a release"
        exit 1
    fi

    # Check if working directory is clean
    if ! git diff-index --quiet HEAD --; then
        log_error "Working directory is not clean. Commit or stash changes first."
        exit 1
    fi

    # Check if we're up to date with remote
    git fetch origin
    local local_commit=$(git rev-parse HEAD)
    local remote_commit=$(git rev-parse origin/main)

    if [ "$local_commit" != "$remote_commit" ]; then
        log_error "Local branch is not up to date with remote. Pull latest changes first."
        exit 1
    fi

    # Check if tests pass
    log_info "Running tests to ensure quality..."
    if ! ./scripts/test.sh; then
        log_error "Tests failed. Cannot proceed with release."
        exit 1
    fi

    log_success "Repository validation passed"
}

# Get current version
get_current_version() {
    grep -E '^version = ' "$VERSION_FILE" | sed 's/version = "//' | sed 's/"//'
}

# Get next version
get_next_version() {
    local current_version=$1
    local bump_type=${2:-"patch"}

    # Parse version components
    local major=$(echo "$current_version" | cut -d. -f1)
    local minor=$(echo "$current_version" | cut -d. -f2)
    local patch=$(echo "$current_version" | cut -d. -f3)

    case "$bump_type" in
        "major")
            major=$((major + 1))
            minor=0
            patch=0
            ;;
        "minor")
            minor=$((minor + 1))
            patch=0
            ;;
        "patch")
            patch=$((patch + 1))
            ;;
        *)
            log_error "Invalid bump type: $bump_type"
            exit 1
            ;;
    esac

    echo "${major}.${minor}.${patch}"
}

# Update version in files
update_version() {
    local new_version=$1

    log_info "Updating version to $new_version"

    # Update Cargo.toml
    sed -i.bak "s/^version = .*/version = \"$new_version\"/" "$VERSION_FILE"
    rm "$VERSION_FILE.bak"

    # Update version in README if it exists
    if [ -f "README.md" ]; then
        sed -i.bak "s/v[0-9]\+\.[0-9]\+\.[0-9]\+/v$new_version/g" README.md
        rm "README.md.bak"
    fi

    log_success "Version updated to $new_version"
}

# Generate changelog
generate_changelog() {
    local new_version=$1
    local prev_tag=$(git describe --tags --abbrev=0 2>/dev/null || echo "")

    log_info "Generating changelog for $new_version"

    local changelog_entry="## [$new_version] - $(date +%Y-%m-%d)

"

    if [ -n "$prev_tag" ]; then
        # Get commits since last tag
        local commits=$(git log --pretty=format:"- %s (%h)" "$prev_tag"..HEAD)
        changelog_entry+="$commits
"
    else
        # All commits if no previous tag
        local commits=$(git log --pretty=format:"- %s (%h)")
        changelog_entry+="$commits
"
    fi

    # Create new changelog
    local temp_changelog=$(mktemp)

    if [ -f "$CHANGELOG_FILE" ]; then
        # Add new entry at the top of existing changelog
        echo "$changelog_entry" > "$temp_changelog"
        cat "$CHANGELOG_FILE" >> "$temp_changelog"
    else
        # Create new changelog
        cat > "$temp_changelog" << EOF
# Changelog

All notable changes to this project will be documented in this file.

$changelog_entry
EOF
    fi

    mv "$temp_changelog" "$CHANGELOG_FILE"

    log_success "Changelog generated"
}

# Build release artifacts
build_artifacts() {
    local version=$1

    log_info "Building release artifacts for version $version"

    # Set version for build script
    export VERSION="$version"

    # Run build script
    if [ -f "./scripts/build.sh" ]; then
        ./scripts/build.sh
    else
        log_error "Build script not found"
        exit 1
    fi

    log_success "Release artifacts built"
}

# Create GitHub release
create_github_release() {
    local version=$1
    local prerelease=${2:-false}

    log_info "Creating GitHub release for v$version"

    # Create release notes
    local release_notes=""
    if [ -f "$CHANGELOG_FILE" ]; then
        # Extract the latest changelog entry
        release_notes=$(sed -n "/^## \[$version\]/,/^## \[/p" "$CHANGELOG_FILE" | head -n -1)
    fi

    if [ -z "$release_notes" ]; then
        release_notes="Release version $version"
    fi

    # Create release
    local release_args=(
        "--title" "Devora v$version"
        "--notes" "$release_notes"
        "--repo" "$GITHUB_REPO"
    )

    if [ "$prerelease" = "true" ]; then
        release_args+=("--prerelease")
    fi

    gh release create "v$version" "${release_args[@]}" release/*

    log_success "GitHub release created: v$version"
}

# Publish to crates.io
publish_to_crates() {
    log_info "Publishing to crates.io..."

    # Dry run first
    cargo publish --dry-run

    # Actually publish
    cargo publish

    log_success "Published to crates.io"
}

# Update Homebrew formula (if applicable)
update_homebrew() {
    log_info "Triggering Homebrew formula update..."

    # This would typically be handled by a separate workflow or PR
    log_info "Homebrew formula update will be handled by CI/CD pipeline"
}

# Commit and tag changes
commit_and_tag() {
    local version=$1

    log_info "Committing version changes and creating tag"

    # Commit changes
    git add "$VERSION_FILE" "$CHANGELOG_FILE" README.md 2>/dev/null || true
    git commit -m "chore: Release v$version"

    # Create tag
    git tag -a "v$version" -m "Release v$version"

    log_success "Changes committed and tagged"
}

# Push to remote
push_to_remote() {
    log_info "Pushing changes and tags to remote"

    git push origin main
    git push origin --tags

    log_success "Changes pushed to remote"
}

# Main release function
main() {
    local version_type=${1:-"patch"}
    local prerelease=${2:-false}

    log_info "Starting Devora release process"

    check_dependencies
    validate_repository

    local current_version=$(get_current_version)
    local new_version=$(get_next_version "$current_version" "$version_type")

    log_info "Current version: $current_version"
    log_info "New version: $new_version"

    # Confirm release
    read -p "Do you want to release version $new_version? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        log_info "Release cancelled"
        exit 0
    fi

    # Release process
    update_version "$new_version"
    generate_changelog "$new_version"
    commit_and_tag "$new_version"

    # Build artifacts before pushing
    build_artifacts "$new_version"

    # Push changes
    push_to_remote

    # Create release
    create_github_release "$new_version" "$prerelease"

    # Publish to package registries
    if [ "$prerelease" != "true" ]; then
        publish_to_crates
        update_homebrew
    fi

    log_success "Release v$new_version completed successfully!"
    log_info "GitHub release: https://github.com/$GITHUB_REPO/releases/v$new_version"
}

# Script entry point
case "${1:-}" in
    "patch"|"minor"|"major")
        main "$1" "${2:-false}"
        ;;
    "prerelease")
        main "patch" "true"
        ;;
    "help"|"-h"|"--help")
        echo "Devora Release Script"
        echo ""
        echo "Usage: $0 [version_type] [prerelease]"
        echo ""
        echo "Version types:"
        echo "  patch      - Bump patch version (0.1.0 -> 0.1.1)"
        echo "  minor      - Bump minor version (0.1.0 -> 0.2.0)"
        echo "  major      - Bump major version (0.1.0 -> 1.0.0)"
        echo "  prerelease - Create prerelease"
        echo ""
        echo "Examples:"
        echo "  $0 patch         # Release patch version"
        echo "  $0 minor         # Release minor version"
        echo "  $0 prerelease    # Create prerelease"
        echo ""
        echo "Dependencies:"
        echo "  - git"
        echo "  - cargo"
        echo "  - gh (GitHub CLI)"
        ;;
    *)
        log_error "Invalid argument. Use 'help' for usage information."
        exit 1
        ;;
esac