//! GitHub CLI authentication module
//!
//! This module provides functionality to retrieve GitHub authentication tokens
//! using the GitHub CLI (`gh`).

use crate::command_runner::CommandRunner;
use crate::error::{Error, Result};
use std::io::ErrorKind;

/// Retrieves a GitHub authentication token by calling `gh auth token`.
///
/// This function executes the GitHub CLI command to retrieve the current
/// authentication token. It distinguishes between the GitHub CLI not being
/// installed and the user not being authenticated.
///
/// # Arguments
///
/// * `command_runner` - A `CommandRunner` implementation for executing commands
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
/// use gh_discussion_export::command_runner::StdCommandRunner;
///
/// let runner = StdCommandRunner;
/// match get_github_token(&runner) {
///     Ok(token) => println!("Got token: {}", token),
///     Err(e) => eprintln!("Error: {}", e),
/// }
/// ```
pub fn get_github_token(command_runner: &dyn CommandRunner) -> Result<String> {
    // Execute `gh auth token` command
    let args = vec!["auth".to_string(), "token".to_string()];
    let output = command_runner.run("gh", &args).map_err(|err| {
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
    use crate::command_runner::MockCommandRunner;
    use std::os::unix::process::ExitStatusExt;

    /// Helper to create a successful output with stdout content
    fn mock_success_output(stdout: &str) -> std::process::Output {
        std::process::Output {
            status: ExitStatusExt::from_raw(0),
            stdout: stdout.as_bytes().to_vec(),
            stderr: Vec::new(),
        }
    }

    /// Helper to create a failed output (non-zero exit code)
    fn mock_failure_output() -> std::process::Output {
        std::process::Output {
            status: ExitStatusExt::from_raw(256), // exit code 1
            stdout: Vec::new(),
            stderr: Vec::new(),
        }
    }

    #[test]
    fn test_get_github_token_success() {
        let mut mock = MockCommandRunner::new();
        mock.expect_run()
            .times(1)
            .returning(|_, _| Ok(mock_success_output("ghp_test_token_123")));

        let result = get_github_token(&mock);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "ghp_test_token_123");
    }

    #[test]
    fn test_get_github_token_not_found() {
        let mut mock = MockCommandRunner::new();
        mock.expect_run()
            .times(1)
            .returning(|_, _| {
                Err(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "gh not found",
                ))
            });

        let result = get_github_token(&mock);
        assert!(matches!(result, Err(Error::GitHubCliNotFound)));
    }

    #[test]
    fn test_get_github_token_auth_failure() {
        let mut mock = MockCommandRunner::new();
        mock.expect_run()
            .times(1)
            .returning(|_, _| Ok(mock_failure_output()));

        let result = get_github_token(&mock);
        assert!(matches!(result, Err(Error::Authentication)));
    }

    #[test]
    fn test_get_github_token_empty_token() {
        let mut mock = MockCommandRunner::new();
        mock.expect_run()
            .times(1)
            .returning(|_, _| Ok(mock_success_output("")));

        let result = get_github_token(&mock);
        assert!(matches!(result, Err(Error::Authentication)));
    }
}
