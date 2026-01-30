//! GitHub CLI authentication module
//!
//! This module provides functionality to retrieve GitHub authentication tokens
//! using the GitHub CLI (`gh`).

use crate::error::{Error, Result};
use std::io::ErrorKind;
use std::process::Command;

/// Retrieves a GitHub authentication token by calling `gh auth token`.
///
/// This function executes the GitHub CLI command to retrieve the current
/// authentication token. It distinguishes between the GitHub CLI not being
/// installed and the user not being authenticated.
///
/// # Returns
///
/// Returns `Ok(String)` containing the GitHub token if successful.
///
/// Returns `Err(Error::GitHubCliNotFound)` if the GitHub CLI is not installed.
/// Returns `Err(Error::Authentication)` if the user is not authenticated or
/// the token is empty.
///
/// # Example
///
/// ```no_run
/// use gh_discussion_export::auth::get_github_token;
///
/// match get_github_token() {
///     Ok(token) => println!("Got token: {}", token),
///     Err(e) => eprintln!("Error: {}", e),
/// }
/// ```
pub fn get_github_token() -> Result<String> {
    // Execute `gh auth token` command
    let output = Command::new("gh")
        .args(["auth", "token"])
        .output()
        .map_err(|err| {
            // Distinguish between "gh not found" vs other I/O errors
            if err.kind() == ErrorKind::NotFound {
                Error::GitHubCliNotFound
            } else {
                Error::Io(err)
            }
        })?;

    // Check if command succeeded (exit code 0)
    if !output.status.success() {
        return Err(Error::Authentication);
    }

    // Convert stdout to String and trim whitespace
    let token = String::from_utf8_lossy(&output.stdout).trim().to_string();

    // Check for empty token
    if token.is_empty() {
        return Err(Error::Authentication);
    }

    Ok(token)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;
    use std::os::unix::fs::PermissionsExt;
    use tempfile::tempdir;

    // IMPORTANT: These tests modify the PATH environment variable to mock the `gh` command.
    // When running all tests together, use --test-threads=1 to ensure these tests run
    // sequentially and avoid race conditions. Example: cargo test --lib -- --test-threads=1
    //
    // This is a known limitation of using environment variable override for mocking
    // external commands in tests, and is acceptable for this project's testing strategy.

    #[test]
    fn test_get_github_token_success() {
        // Create mock gh script
        let temp_dir = tempdir().unwrap();
        let mock_gh = temp_dir.path().join("gh");

        // Script that outputs a valid token
        let script = "#!/bin/sh\necho 'ghp_test_token_123'";
        fs::write(&mock_gh, script).unwrap();

        // Make executable
        fs::set_permissions(&mock_gh, std::fs::Permissions::from_mode(0o755)).unwrap();

        // Override PATH
        let original_path = env::var("PATH").unwrap();
        unsafe {
            env::set_var(
                "PATH",
                format!("{}:{}", temp_dir.path().display(), original_path),
            );
        }

        // Test
        let result = get_github_token();

        // Restore PATH
        unsafe {
            env::set_var("PATH", original_path);
        }

        // Verify
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "ghp_test_token_123");
    }

    #[test]
    fn test_get_github_token_not_found() {
        // Set empty PATH to make gh not found
        let original_path = env::var("PATH").unwrap();
        unsafe {
            env::set_var("PATH", "");
        }

        let result = get_github_token();

        unsafe {
            env::set_var("PATH", original_path);
        }

        assert!(matches!(result, Err(Error::GitHubCliNotFound)));
    }

    #[test]
    fn test_get_github_token_auth_failure() {
        // Create mock gh that exits with error
        let temp_dir = tempdir().unwrap();
        let mock_gh = temp_dir.path().join("gh");

        let script = "#!/bin/sh\nexit 1";
        fs::write(&mock_gh, script).unwrap();
        fs::set_permissions(&mock_gh, std::fs::Permissions::from_mode(0o755)).unwrap();

        let original_path = env::var("PATH").unwrap();
        unsafe {
            env::set_var(
                "PATH",
                format!("{}:{}", temp_dir.path().display(), original_path),
            );
        }

        let result = get_github_token();

        unsafe {
            env::set_var("PATH", original_path);
        }

        assert!(matches!(result, Err(Error::Authentication)));
    }

    #[test]
    fn test_get_github_token_empty_token() {
        // Create mock gh that outputs empty string
        let temp_dir = tempdir().unwrap();
        let mock_gh = temp_dir.path().join("gh");

        let script = "#!/bin/sh\necho ''";
        fs::write(&mock_gh, script).unwrap();
        fs::set_permissions(&mock_gh, std::fs::Permissions::from_mode(0o755)).unwrap();

        let original_path = env::var("PATH").unwrap();
        unsafe {
            env::set_var(
                "PATH",
                format!("{}:{}", temp_dir.path().display(), original_path),
            );
        }

        let result = get_github_token();

        unsafe {
            env::set_var("PATH", original_path);
        }

        assert!(matches!(result, Err(Error::Authentication)));
    }
}
