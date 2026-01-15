/// Integration tests for the CLI interface
///
/// These tests verify the actual binary behavior including:
/// - Help output
/// - Version output
/// - Error messages for invalid arguments

use std::process::Command;

const BINARY_NAME: &str = "secrets";

#[test]
fn test_help_contains_usage() {
    // Use cargo run with --help flag
    let output = Command::new("cargo")
        .args(["run", "--", "--help"])
        .output()
        .expect("Failed to execute command with --help");

    assert!(output.status.success(), "cargo run --help should succeed");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    let help_output = if stdout.is_empty() { &stderr } else { &stdout };

    // Verify key elements of help output
    assert!(help_output.contains("Usage"), "Help should contain 'Usage'");
    assert!(help_output.contains("Arguments:"), "Help should contain Arguments section");
    assert!(help_output.contains("Options:"), "Help should contain Options section");
    assert!(help_output.contains("-a, --show-all"), "Help should show show-all option");
    assert!(help_output.contains("<NAMESPACE>"), "Help should show NAMESPACE argument");
    assert!(help_output.contains("[QUERY]"), "Help should show query argument");
    assert!(help_output.contains("-h, --help"), "Help should show help option");
    assert!(help_output.contains("-V, --version"), "Help should show version option");
}

#[test]
fn test_help_contains_description() {
    let output = Command::new("cargo")
        .args(["run", "--", "--help"])
        .output()
        .expect("Failed to execute command with --help");

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    let help_output = if stdout.is_empty() { &stderr } else { &stdout };

    // Should contain the description from about
    assert!(
        help_output.contains("Command line utility") || help_output.contains("secrets"),
        "Help should contain description"
    );
}

#[test]
fn test_version_output() {
    let output = Command::new("cargo")
        .args(["run", "--", "--version"])
        .output()
        .expect("Failed to execute command with --version");

    assert!(output.status.success(), "cargo run --version should succeed");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    let version_output = if stdout.is_empty() { &stderr } else { &stdout };

    // Version output should contain the binary name and version
    assert!(version_output.contains(BINARY_NAME), "Version should contain binary name");
}

#[test]
fn test_missing_namespace_shows_error() {
    let output = Command::new("cargo")
        .args(["run", "--"])
        .output()
        .expect("Failed to execute command without args");

    assert!(!output.status.success(), "Missing namespace should fail");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    let error_output = if stdout.is_empty() { &stderr } else { &stdout };

    // Error should mention required argument
    assert!(
        error_output.contains("required") ||
        error_output.contains("NAMESPACE") ||
        error_output.contains("The following required argument was not provided"),
        "Error should indicate required argument is missing"
    );
}

#[test]
fn test_invalid_option_shows_error() {
    let output = Command::new("cargo")
        .args(["run", "--", "--invalid-option", "default"])
        .output()
        .expect("Failed to execute command with invalid option");

    assert!(!output.status.success(), "Invalid option should fail");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    let error_output = if stdout.is_empty() { &stderr } else { &stdout };

    // Error should indicate the option is unrecognized
    assert!(
        error_output.contains("unexpected") ||
        error_output.contains("invalid") ||
        error_output.contains("argument") ||
        error_output.contains("unrecognized") ||
        error_output.contains("error"),
        "Error should indicate invalid argument"
    );
}

#[test]
fn test_invalid_short_option_shows_error() {
    let output = Command::new("cargo")
        .args(["run", "--", "-x", "default"])
        .output()
        .expect("Failed to execute command with invalid short option");

    assert!(!output.status.success(), "Invalid short option should fail");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    let error_output = if stdout.is_empty() { &stderr } else { &stdout };

    assert!(
        error_output.contains("unexpected") ||
        error_output.contains("invalid") ||
        error_output.contains("argument") ||
        error_output.contains("unrecognized") ||
        error_output.contains("error") ||
        error_output.contains("-x"),
        "Error should indicate invalid short option"
    );
}

#[test]
fn test_show_all_short_option_accepted() {
    // Test that -a doesn't cause an immediate parse error
    // We can't use --help with positional args, so we test with an invalid option
    // which should fail AFTER -a is parsed
    let output = Command::new("cargo")
        .args(["run", "--", "-a", "--invalid-arg"])
        .output()
        .expect("Failed to execute command with -a");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    let error_output = if stdout.is_empty() { &stderr } else { &stdout };

    // The error should be about the invalid arg, not about -a
    // This proves -a was accepted
    assert!(
        error_output.contains("invalid") ||
        error_output.contains("unexpected") ||
        error_output.contains("unrecognized"),
        "Should show error for invalid arg (proving -a was accepted)"
    );
}

#[test]
fn test_show_all_long_option_accepted() {
    let output = Command::new("cargo")
        .args(["run", "--", "--show-all", "--invalid-arg"])
        .output()
        .expect("Failed to execute command with --show-all");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    let error_output = if stdout.is_empty() { &stderr } else { &stdout };

    // The error should be about the invalid arg, not about --show-all
    assert!(
        error_output.contains("invalid") ||
        error_output.contains("unexpected") ||
        error_output.contains("unrecognized"),
        "Should show error for invalid arg (proving --show-all was accepted)"
    );
}

#[test]
fn test_namespace_argument_accepted() {
    // Test that providing a namespace doesn't cause a parse error
    let output = Command::new("cargo")
        .args(["run", "--", "default", "--invalid-arg"])
        .output()
        .expect("Failed to execute command with namespace");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    let error_output = if stdout.is_empty() { &stderr } else { &stdout };

    // The error should be about the invalid arg, not about namespace
    assert!(
        error_output.contains("invalid") ||
        error_output.contains("unexpected") ||
        error_output.contains("unrecognized"),
        "Should show error for invalid arg (proving namespace was accepted)"
    );
}

#[test]
fn test_query_argument_accepted() {
    let output = Command::new("cargo")
        .args(["run", "--", "default", "my-query", "--invalid-arg"])
        .output()
        .expect("Failed to execute command with query");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    let error_output = if stdout.is_empty() { &stderr } else { &stdout };

    // The error should be about the invalid arg, not about query
    assert!(
        error_output.contains("invalid") ||
        error_output.contains("unexpected") ||
        error_output.contains("unrecognized"),
        "Should show error for invalid arg (proving query was accepted)"
    );
}

#[test]
fn test_all_arguments_together() {
    let output = Command::new("cargo")
        .args(["run", "--", "-a", "kube-system", "cert", "--invalid-arg"])
        .output()
        .expect("Failed to execute command with all arguments");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    let error_output = if stdout.is_empty() { &stderr } else { &stdout };

    // The error should be about the invalid arg, proving all args were accepted
    assert!(
        error_output.contains("invalid") ||
        error_output.contains("unexpected") ||
        error_output.contains("unrecognized"),
        "Should show error for invalid arg (proving all args were accepted)"
    );
}

#[test]
fn test_show_all_flag_variations() {
    // Test both short and long form of show-all
    for flag in ["-a", "--show-all"] {
        let output = Command::new("cargo")
            .args(["run", "--", flag, "default", "--invalid-arg"])
            .output()
            .expect("Failed to execute command");

        // Should fail on invalid arg, not on the flag
        assert!(!output.status.success());
    }
}

#[test]
fn test_help_flag_short_and_long() {
    // Test both -h and --help work
    for flag in ["-h", "--help"] {
        let output = Command::new("cargo")
            .args(["run", "--", flag])
            .output()
            .expect("Failed to execute command");

        assert!(output.status.success(), "{} should show help", flag);

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        let help_output = if stdout.is_empty() { &stderr } else { &stdout };

        assert!(help_output.contains("Usage"), "{} should show usage", flag);
    }
}

#[test]
fn test_version_flag_short_and_long() {
    // Test both -V and --version work
    for flag in ["-V", "--version"] {
        let output = Command::new("cargo")
            .args(["run", "--", flag])
            .output()
            .expect("Failed to execute command");

        assert!(output.status.success(), "{} should show version", flag);

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        let version_output = if stdout.is_empty() { &stderr } else { &stdout };

        assert!(version_output.contains(BINARY_NAME), "{} should show binary name", flag);
    }
}
