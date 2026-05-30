use crate::error::{DevoraError, Result};
use git2::{Repository, Signature};
use serde_json::{Map, Value};
use std::path::Path;

/// Initialize a git repository and make initial commit
pub fn initialize_git_repo(project_path: &Path, context: &Map<String, Value>) -> Result<()> {
    // Initialize repository
    let repo = Repository::init(project_path).map_err(|e| DevoraError::HookExecutionError {
        hook: "git init".to_string(),
        details: format!("Failed to initialize git repository: {}", e),
    })?;

    // Create .gitignore if it doesn't exist
    let gitignore_path = project_path.join(".gitignore");
    if !gitignore_path.exists() {
        create_gitignore(&gitignore_path, context)?;
    }

    // Get author information from context or git config
    let signature = get_git_signature(context)?;

    // Add all files to index
    let mut index = repo.index().map_err(|e| DevoraError::HookExecutionError {
        hook: "git add".to_string(),
        details: format!("Failed to get git index: {}", e),
    })?;

    index
        .add_all(["."].iter(), git2::IndexAddOption::DEFAULT, None)
        .map_err(|e| DevoraError::HookExecutionError {
            hook: "git add".to_string(),
            details: format!("Failed to add files to git index: {}", e),
        })?;

    index.write().map_err(|e| DevoraError::HookExecutionError {
        hook: "git add".to_string(),
        details: format!("Failed to write git index: {}", e),
    })?;

    // Create tree object
    let tree_id = index
        .write_tree()
        .map_err(|e| DevoraError::HookExecutionError {
            hook: "git write-tree".to_string(),
            details: format!("Failed to write git tree: {}", e),
        })?;

    let tree = repo
        .find_tree(tree_id)
        .map_err(|e| DevoraError::HookExecutionError {
            hook: "git find-tree".to_string(),
            details: format!("Failed to find git tree: {}", e),
        })?;

    // Create initial commit
    let parent_commit = None;
    let message = "Initial commit: Generated with Devora";

    repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        message,
        &tree,
        parent_commit
            .iter()
            .collect::<Vec<&git2::Commit>>()
            .as_slice(),
    )
    .map_err(|e| DevoraError::HookExecutionError {
        hook: "git commit".to_string(),
        details: format!("Failed to create initial commit: {}", e),
    })?;

    Ok(())
}

/// Get git signature from context or system git config
fn get_git_signature(context: &Map<String, Value>) -> Result<Signature<'static>> {
    let name = context
        .get("author")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .or_else(|| get_git_config_name().ok())
        .unwrap_or_else(|| "Anonymous".to_string());

    let email = context
        .get("email")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .or_else(|| get_git_config_email().ok())
        .unwrap_or_else(|| "anonymous@example.com".to_string());

    Signature::now(&name, &email).map_err(|e| DevoraError::HookExecutionError {
        hook: "git signature".to_string(),
        details: format!("Failed to create git signature: {}", e),
    })
}

/// Get name from git config
fn get_git_config_name() -> Result<String> {
    let config = git2::Config::open_default().map_err(|e| DevoraError::HookExecutionError {
        hook: "git config".to_string(),
        details: format!("Failed to open git config: {}", e),
    })?;

    config
        .get_string("user.name")
        .map_err(|e| DevoraError::HookExecutionError {
            hook: "git config".to_string(),
            details: format!("Failed to get git user.name: {}", e),
        })
}

/// Get email from git config
fn get_git_config_email() -> Result<String> {
    let config = git2::Config::open_default().map_err(|e| DevoraError::HookExecutionError {
        hook: "git config".to_string(),
        details: format!("Failed to open git config: {}", e),
    })?;

    config
        .get_string("user.email")
        .map_err(|e| DevoraError::HookExecutionError {
            hook: "git config".to_string(),
            details: format!("Failed to get git user.email: {}", e),
        })
}

/// Create a .gitignore file
fn create_gitignore(gitignore_path: &Path, context: &Map<String, Value>) -> Result<()> {
    use std::fs::write;

    let framework = context
        .get("framework")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown");

    let gitignore_content = match framework {
        "rust" => get_rust_gitignore_template(),
        _ => create_generic_gitignore(),
    };

    // Simple template replacement (not full Tera processing since we're in utils)
    let project_name = context
        .get("project_name")
        .and_then(|v| v.as_str())
        .unwrap_or("my-project");

    let content = gitignore_content.replace("{{ project_name }}", project_name);

    write(gitignore_path, content).map_err(|e| DevoraError::FileSystemError {
        path: gitignore_path.to_string_lossy().to_string(),
        message: format!("Failed to create .gitignore: {}", e),
    })?;

    Ok(())
}

/// Get Rust-specific .gitignore template
fn get_rust_gitignore_template() -> &'static str {
    r#"# Rust
/target/
**/*.rs.bk

# Cargo lock file (optional - uncomment if you don't want to commit it)
# Cargo.lock

# IDE files
.vscode/
.idea/
*.swp
*.swo
*~

# OS generated files
.DS_Store
.DS_Store?
._*
.Spotlight-V100
.Trashes
ehthumbs.db
Thumbs.db
"#
}

/// Create a generic .gitignore content
fn create_generic_gitignore() -> &'static str {
    r#"# OS generated files
.DS_Store
.DS_Store?
._*
.Spotlight-V100
.Trashes
ehthumbs.db
Thumbs.db

# IDE files
.vscode/
.idea/
*.swp
*.swo
*~

# Build directories
target/
build/
dist/
out/
"#
}
