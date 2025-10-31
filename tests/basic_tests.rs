// Basic tests for Devora
// Simple functionality tests

#[test]
fn test_version_exists() {
    // Test that we can get version information
    let output = std::process::Command::new("./target/debug/devora")
        .arg("--version")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success(), "Command failed: {:?}", String::from_utf8_lossy(&output.stderr));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("devora"));
}

#[test]
fn test_help_exists() {
    // Test that we can get help information
    let output = std::process::Command::new("./target/debug/devora")
        .arg("--help")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success(), "Command failed: {:?}", String::from_utf8_lossy(&output.stderr));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("devora"));
    assert!(stdout.contains("Usage"));
}

#[test]
fn test_list_command() {
    // Test that list command works
    let output = std::process::Command::new("./target/debug/devora")
        .arg("list")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success(), "Command failed: {:?}", String::from_utf8_lossy(&output.stderr));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Available"));
    assert!(stdout.contains("cpp") || stdout.contains("rust"));
}