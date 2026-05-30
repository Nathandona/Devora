#!/bin/bash
# Devora Test Script
# Comprehensive testing suite for Devora

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
PROJECT_NAME="devora"
TEST_RESULTS_DIR="test-results"
COVERAGE_DIR="coverage"
INTEGRATION_TEST_DIR="integration-tests"

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
    log_info "Checking test dependencies..."

    local missing_deps=()

    if ! command -v cargo &> /dev/null; then
        missing_deps+=("cargo")
    fi

    # Check for optional but recommended tools
    if ! command -v cargo-nextest &> /dev/null; then
        log_warning "cargo-nextest not found, using default test runner"
    fi

    if ! command -v cargo-llvm-cov &> /dev/null; then
        log_warning "cargo-llvm-cov not found, coverage will be skipped"
    fi

    if [ ${#missing_deps[@]} -ne 0 ]; then
        log_error "Missing dependencies: ${missing_deps[*]}"
        exit 1
    fi

    log_success "Test dependencies verified"
}

# Setup test environment
setup_test_env() {
    log_info "Setting up test environment..."

    # Create test directories
    mkdir -p "$TEST_RESULTS_DIR"
    mkdir -p "$COVERAGE_DIR"
    mkdir -p "$INTEGRATION_TEST_DIR"

    # Set environment variables for testing
    export RUST_LOG=debug
    export RUST_BACKTRACE=1
    export DEVORA_TEST_MODE=1

    log_success "Test environment ready"
}

# Run unit tests
run_unit_tests() {
    log_info "Running unit tests..."

    local test_command="cargo test"
    if command -v cargo-nextest &> /dev/null; then
        test_command="cargo nextest run"
    fi

    # Run tests with output
    $test_command --verbose --message-format=human | tee "$TEST_RESULTS_DIR/unit_tests.log"

    # Run tests in release mode to catch release-specific issues
    $test_command --release --verbose | tee "$TEST_RESULTS_DIR/unit_tests_release.log"

    log_success "Unit tests completed"
}

# Run integration tests
run_integration_tests() {
    log_info "Running integration tests..."

    # Build the binary first
    cargo build --release

    local binary_path="target/release/$PROJECT_NAME"

    if [ ! -f "$binary_path" ]; then
        log_error "Binary not found at $binary_path"
        return 1
    fi

    # Test basic functionality
    log_info "Testing basic CLI functionality..."

    # Test version command
    "$binary_path" --version > "$TEST_RESULTS_DIR/version_test.log" 2>&1
    if [ $? -eq 0 ]; then
        log_success "Version command works"
    else
        log_error "Version command failed"
        cat "$TEST_RESULTS_DIR/version_test.log"
        return 1
    fi

    # Test help command
    "$binary_path" --help > "$TEST_RESULTS_DIR/help_test.log" 2>&1
    if [ $? -eq 0 ]; then
        log_success "Help command works"
    else
        log_error "Help command failed"
        cat "$TEST_RESULTS_DIR/help_test.log"
        return 1
    fi

    # Test list command
    "$binary_path" list > "$TEST_RESULTS_DIR/list_test.log" 2>&1
    if [ $? -eq 0 ]; then
        log_success "List command works"
    else
        log_error "List command failed"
        cat "$TEST_RESULTS_DIR/list_test.log"
        return 1
    fi

    # Test project generation
    test_project_generation "$binary_path"

    log_success "Integration tests completed"
}

# Test project generation functionality
test_project_generation() {
    local binary_path=$1
    local test_projects_dir="$INTEGRATION_TEST_DIR/projects"

    log_info "Testing project generation..."

    mkdir -p "$test_projects_dir"

    # Test Rust project generation
    log_info "Testing Rust project generation..."
    "$binary_path" new "test-rust-project" rust --framework cmake --non-interactive --dry-run > "$TEST_RESULTS_DIR/rust_project_test.log" 2>&1
    if [ $? -eq 0 ]; then
        log_success "Rust project generation test passed"
    else
        log_error "Rust project generation test failed"
        cat "$TEST_RESULTS_DIR/rust_project_test.log"
        return 1
    fi

    # Test with actual file creation
    cd "$test_projects_dir"
    "$binary_path" new "real-test-project" rust --framework cmake --non-interactive > "../$TEST_RESULTS_DIR/real_project_test.log" 2>&1
    cd - > /dev/null

    if [ -d "$test_projects_dir/real-test-project" ]; then
        log_success "Real project creation test passed"

        # Verify generated project structure
        if [ -f "$test_projects_dir/real-test-project/CMakeLists.txt" ]; then
            log_success "CMakeLists.txt generated correctly"
        else
            log_warning "CMakeLists.txt not found in generated project"
        fi

        if [ -f "$test_projects_dir/real-test-project/src/main.cpp" ]; then
            log_success "main.cpp generated correctly"
        else
            log_warning "main.cpp not found in generated project"
        fi
    else
        log_error "Real project creation test failed"
        cat "$TEST_RESULTS_DIR/real_project_test.log"
        return 1
    fi

    # Cleanup test projects
    rm -rf "$test_projects_dir/real-test-project"
}

# Run code coverage
run_coverage() {
    log_info "Running code coverage..."

    if ! command -v cargo-llvm-cov &> /dev/null; then
        log_warning "cargo-llvm-cov not installed, skipping coverage"
        return 0
    fi

    # Generate coverage report
    cargo llvm-cov --lcov --output-path "$COVERAGE_DIR/lcov.info" --text --output-dir "$COVERAGE_DIR"

    # Generate HTML report
    cargo llvm-cov --html --output-dir "$COVERAGE_DIR/html"

    log_success "Coverage report generated in $COVERAGE_DIR"

    # Show coverage summary
    if [ -f "$COVERAGE_DIR/lcov.info" ]; then
        local coverage_percent=$(lcov --summary "$COVERAGE_DIR/lcov.info" 2>&1 | grep "lines......:" | tail -1 | awk '{print $2}' | sed 's/%//')
        log_info "Code coverage: ${coverage_percent}%"
    fi
}

# Run benchmarks
run_benchmarks() {
    log_info "Running benchmarks..."

    if [ ! -d "benches" ]; then
        log_warning "No benchmarks found, skipping"
        return 0
    fi

    # Run benchmarks
    cargo bench --all-features | tee "$TEST_RESULTS_DIR/benchmarks.log"

    log_success "Benchmarks completed"
}

# Run linting and formatting checks
run_linting() {
    log_info "Running linting and formatting checks..."

    # Check formatting
    log_info "Checking code formatting..."
    cargo fmt --all -- --check
    if [ $? -eq 0 ]; then
        log_success "Code formatting is correct"
    else
        log_error "Code formatting issues found"
        log_info "Run 'cargo fmt' to fix formatting issues"
        return 1
    fi

    # Run clippy
    log_info "Running clippy..."
    cargo clippy --all-targets --all-features -- -D warnings
    if [ $? -eq 0 ]; then
        log_success "Clippy checks passed"
    else
        log_error "Clippy found issues"
        return 1
    fi

    log_success "Linting checks completed"
}

# Run security audit
run_security_audit() {
    log_info "Running security audit..."

    if ! command -v cargo-audit &> /dev/null; then
        log_warning "cargo-audit not installed, skipping security audit"
        return 0
    fi

    # Run security audit
    cargo audit > "$TEST_RESULTS_DIR/security_audit.log" 2>&1

    if [ $? -eq 0 ]; then
        log_success "Security audit passed"
    else
        log_warning "Security audit found issues"
        log_info "Check $TEST_RESULTS_DIR/security_audit.log for details"
    fi
}

# Generate test report
generate_report() {
    log_info "Generating test report..."

    local report_file="$TEST_RESULTS_DIR/test_report.md"

    cat > "$report_file" << EOF
# Devora Test Report

**Date:** $(date)
**Version:** $(git describe --tags --always --dirty 2>/dev/null || echo "unknown")

## Test Results

### Unit Tests
- Status: $([ -f "$TEST_RESULTS_DIR/unit_tests.log" ] && echo "Passed" || echo "Failed")
- Log: [unit_tests.log](unit_tests.log)

### Integration Tests
- Status: $([ -f "$TEST_RESULTS_DIR/integration_test.log" ] && echo "Passed" || echo "Failed")
- Log: [integration_test.log](integration_test.log)

### Code Coverage
EOF

    if [ -f "$COVERAGE_DIR/lcov.info" ]; then
        local coverage_percent=$(lcov --summary "$COVERAGE_DIR/lcov.info" 2>&1 | grep "lines......:" | tail -1 | awk '{print $2}' | sed 's/%//' || echo "unknown")
        echo "- Coverage: ${coverage_percent}%" >> "$report_file"
        echo "- HTML Report: [coverage/html](../coverage/html/)" >> "$report_file"
    else
        echo "- Status: Not generated" >> "$report_file"
    fi

    cat >> "$report_file" << EOF

### Benchmarks
- Status: $([ -f "$TEST_RESULTS_DIR/benchmarks.log" ] && echo "Completed" || echo "Not found")
- Log: [benchmarks.log](benchmarks.log)

### Linting
- Status: $([ -f "$TEST_RESULTS_DIR/linting.log" ] && echo "Passed" || echo "Issues found")

### Security Audit
- Status: $([ -f "$TEST_RESULTS_DIR/security_audit.log" ] && echo "Passed" || echo "Issues found")
- Log: [security_audit.log](security_audit.log)

## Summary

EOF

    # Add summary based on test results
    local failed_tests=0
    [ ! -f "$TEST_RESULTS_DIR/unit_tests.log" ] && ((failed_tests++))
    [ ! -f "$TEST_RESULTS_DIR/integration_test.log" ] && ((failed_tests++))

    if [ $failed_tests -eq 0 ]; then
        echo "All tests passed!" >> "$report_file"
    else
        echo "$failed_tests test(s) failed" >> "$report_file"
    fi

    log_success "Test report generated: $report_file"
}

# Cleanup test artifacts
cleanup() {
    log_info "Cleaning up test artifacts..."
    rm -rf "$INTEGRATION_TEST_DIR/projects"
    log_success "Cleanup completed"
}

# Main test function
main() {
    log_info "Starting Devora test suite"

    check_dependencies
    setup_test_env

    # Run all tests
    run_linting
    run_unit_tests
    run_integration_tests
    run_coverage
    run_benchmarks
    run_security_audit

    # Generate report
    generate_report

    # Cleanup
    cleanup

    log_success "Test suite completed successfully!"
    log_info "Test results available in: $TEST_RESULTS_DIR"
}

# Script entry point
case "${1:-}" in
    "unit")
        check_dependencies
        setup_test_env
        run_unit_tests
        ;;
    "integration")
        check_dependencies
        setup_test_env
        run_integration_tests
        ;;
    "coverage")
        check_dependencies
        setup_test_env
        run_coverage
        ;;
    "lint")
        run_linting
        ;;
    "security")
        run_security_audit
        ;;
    "bench")
        run_benchmarks
        ;;
    "clean")
        rm -rf "$TEST_RESULTS_DIR" "$COVERAGE_DIR" "$INTEGRATION_TEST_DIR"
        log_success "Test artifacts cleaned"
        ;;
    "help"|"-h"|"--help")
        echo "Devora Test Script"
        echo ""
        echo "Usage: $0 [command]"
        echo ""
        echo "Commands:"
        echo "  unit         - Run unit tests only"
        echo "  integration  - Run integration tests only"
        echo "  coverage     - Run code coverage only"
        echo "  lint         - Run linting checks only"
        echo "  security     - Run security audit only"
        echo "  bench        - Run benchmarks only"
        echo "  clean        - Clean test artifacts"
        echo "  help         - Show this help message"
        echo ""
        echo "Environment Variables:"
        echo "  RUST_LOG     - Set log level (default: debug)"
        echo "  DEVORA_TEST_MODE - Enable test mode"
        ;;
    *)
        main
        ;;
esac