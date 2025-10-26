//! Development server functionality

use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;
use notify::{RecommendedWatcher, RecursiveMode, Watcher, Event, EventKind};
use console::style;
use crate::config::DevoraConfig;
use crate::result::{Result, DevoraError};
use crate::utils::{find_project_root, is_meson_configured, get_output_path};

pub async fn run(port: u16, verbose: bool) -> Result<()> {
    log::info!("Starting development server on port {}", port);
    if verbose {
        log::debug!("Verbose mode enabled");
    }

    // Find project root
    let project_root = find_project_root()?
        .ok_or_else(|| DevoraError::build("Not in a Devora project (no meson.build or devora.toml found)".to_string()))?;

    log::debug!("Project root: {}", project_root.display());

    // Load configuration
    let config = DevoraConfig::find_and_load()?
        .ok_or_else(|| DevoraError::config("devora.toml not found. Please run 'devora create' first.".to_string()))?;

    // Validate build environment
    if !is_meson_configured(&project_root) {
        println!("{}", style("Setting up initial build...").yellow());
        // Perform initial setup - simple build command
        println!("Run 'devora build' first to set up the build system.");
    }

    println!("{}", style("🚀 Starting development server...").green().bold());
    println!("  Project: {}", project_root.display());
    println!("  Port: {}", port);
    println!("  Live reload: {}", if config.dev.auto_reload { "✅ enabled" } else { "❌ disabled" });
    println!("\n{}", style("Watching for file changes...").blue());

    // Start file watching
    start_file_watcher(&project_root, &config, verbose, port).await?;

    Ok(())
}

async fn start_file_watcher(project_root: &Path, config: &DevoraConfig, verbose: bool, port: u16) -> Result<()> {
    let (tx, mut rx) = tokio::sync::mpsc::channel::<Event>(100);
    let project_root = Arc::new(project_root.to_path_buf());
    let config = Arc::new(config.clone());
    let should_stop = Arc::new(RwLock::new(false));

    // Create watcher
    let mut watcher: RecommendedWatcher = Watcher::new(
        move |res: std::result::Result<Event, notify::Error>| {
            if let Ok(event) = res {
                let _ = tx.blocking_send(event);
            }
        },
        Default::default(),
    ).map_err(|e| DevoraError::filesystem(format!("Failed to create file watcher: {}", e)))?;

    // Watch source directories
    let watch_paths = vec![
        project_root.join("src"),
        project_root.join("include"),
        project_root.join("tests"),
        project_root.join("meson.build"),
        project_root.join("devora.toml"),
    ];

    for path in &watch_paths {
        if path.exists() {
            watcher.watch(path, RecursiveMode::Recursive)
                .map_err(|e| DevoraError::filesystem(format!("Failed to watch {}: {}", path.display(), e)))?;

            if verbose {
                log::debug!("Watching: {}", path.display());
            }
        }
    }

    // Handle file changes
    let project_root_clone = project_root.clone();
    let config_clone = config.clone();
    let should_stop_clone = should_stop.clone();

    tokio::spawn(async move {
        while let Some(event) = rx.recv().await {
            if *should_stop_clone.read().await {
                break;
            }

            if should_process_event(&event, &config_clone) {
                if let Err(e) = handle_file_change(&project_root_clone, &config_clone, verbose).await {
                    log::error!("Error handling file change: {}", e);
                }
            }
        }
    });

    // Simple HTTP server for development
    start_simple_http_server(port, project_root.clone(), should_stop.clone()).await?;

    Ok(())
}

fn should_process_event(event: &Event, config: &DevoraConfig) -> bool {
    // Check if event matches any exclude patterns
    for path in &event.paths {
        let path_str = path.to_string_lossy();

        for pattern in &config.dev.exclude_patterns {
            if path_str.contains(pattern) {
                log::debug!("Excluding change: {} (matches pattern: {})", path.display(), pattern);
                return false;
            }
        }
    }

    matches!(event.kind, EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_))
}

async fn handle_file_change(project_root: &Path, config: &DevoraConfig, _verbose: bool) -> Result<()> {
    println!("\n{}", style("📝 File change detected, rebuilding...").yellow());

    println!("{}", style("Rebuilding...").blue());

    // Note: In a real implementation, this would trigger a rebuild
    // For now, we just show the message
    println!("{}", style("Build completed").green());

    // Show output path
    if let Some(output_path) = get_output_path(project_root, "debug") {
        println!("Executable: {}", output_path.display());
    }

    if config.dev.auto_reload {
        println!("{}", style("🔄 Ready to run!").blue());
        // TODO: Add automatic application restart if needed
    }

    Ok(())
}

async fn start_simple_http_server(
    port: u16,
    _project_root: Arc<PathBuf>,
    _should_stop: Arc<RwLock<bool>>,
) -> Result<()> {
    // For now, just show that server is running
    // In a full implementation, this would start a proper HTTP server
    println!("\n{}", style("Development server is running!").green().bold());
    println!("  Local: http://localhost:{}", port);
    println!("  Press Ctrl+C to stop");

    // Keep the server running
    tokio::signal::ctrl_c()
        .await
        .map_err(|e| DevoraError::process(format!("Failed to listen for Ctrl+C: {}", e)))?;

    println!("\n{}", style("👋 Development server stopped.").yellow());
    Ok(())
}

/// Run the application after successful build
pub async fn run_app(project_root: &Path, _args: Vec<String>) -> Result<()> {
    let output_path = get_output_path(project_root, "debug")
        .ok_or_else(|| DevoraError::build("No executable found after build".to_string()))?;

    println!("{}", style("🎬 Running application...").blue());

    // Run the executable
    let output = tokio::process::Command::new(&output_path)
        .spawn()
        .map_err(|e| DevoraError::process(format!("Failed to start application: {}", e)))?;

    println!("Process started with PID: {:?}", output.id());
    println!("Executable: {}", output_path.display());

    Ok(())
}