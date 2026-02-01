use clap::Parser;

use crate::command_runner::CommandRunner;
use crate::error::{Error, Result};

/// Custom validator to ensure discussion number is positive (>= 1)
fn validate_positive_number(s: &str) -> std::result::Result<u64, String> {
    match s.parse::<u64>() {
        Ok(num) if num > 0 => Ok(num),
        _ => Err("Discussion number must be greater than zero.".to_string()),
    }
}

/// Command-line arguments for GitHub Discussion Export
#[derive(Parser, Debug)]
#[command(name = "gh-discussion-export")]
#[command(about = "Export GitHub Discussion to Markdown", version = "0.1.0")]
pub struct CliArgs {
    /// Discussion number
    #[arg(value_name = "NUMBER", help = "Discussion number", value_parser = validate_positive_number)]
    pub number: u64,

    /// GitHub repository in OWNER/REPO format (auto-detected from Git repository if omitted)
    #[arg(
        long,
        value_name = "OWNER/REPO",
        help = "GitHub repository in OWNER/REPO format (auto-detected from Git repository if omitted)"
    )]
    pub repo: Option<String>,

    /// Output file path (default: <number>-discussion.md)
    #[arg(
        short = 'o',
        long,
        value_name = "PATH",
        help = "Output file path (default: <number>-discussion.md)"
    )]
    pub output: Option<String>,
}

impl CliArgs {
    /// Get the output file path, using default if not specified
    pub fn output_path(&self) -> String {
        match &self.output {
            Some(path) => path.clone(),
            None => format!("{}-discussion.md", self.number),
        }
    }

    /// Get both repository owner and name, avoiding duplicate `gh repo view` calls.
    ///
    /// This method should be preferred when you need both owner and name,
    /// as it only calls `gh repo view` once instead of twice.
    pub fn repo_components(&self) -> Result<(String, String)> {
        let repo_str = match &self.repo {
            Some(repo) => repo.clone(),
            None => Self::detect_from_git_with_runner(&crate::command_runner::StdCommandRunner)?,
        };

        // Parse OWNER/REPO format
        let repo_without_git = repo_str.strip_suffix(".git").unwrap_or(&repo_str);
        let parts: Vec<&str> = repo_without_git.split('/').collect();

        if parts.len() == 2 && !parts[0].is_empty() && !parts[1].is_empty() {
            Ok((parts[0].to_string(), parts[1].to_string()))
        } else {
            Err(Error::InvalidArgs(
                "Repository must be in OWNER/REPO format".to_string(),
            ))
        }
    }

    /// Get the repository owner from explicit --repo flag or auto-detect from Git
    pub fn repo_owner(&self) -> Result<String> {
        let (owner, _) = self.repo_components()?;
        Ok(owner)
    }

    /// Get the repository name from explicit --repo flag or auto-detect from Git
    pub fn repo_name(&self) -> Result<String> {
        let (_, name) = self.repo_components()?;
        Ok(name)
    }

    /// Detect repository from current Git directory using gh CLI with a custom command runner.
    ///
    /// This function is primarily used for testing with mock command runners.
    fn detect_from_git_with_runner(command_runner: &dyn CommandRunner) -> Result<String> {
        // Execute gh repo view command
        let output = command_runner
            .run("gh", &["repo", "view", "--json", "owner,name", "--jq", ".owner.login + \"/\" + .name"])
            .map_err(|e| {
                if e.kind() == std::io::ErrorKind::NotFound {
                    Error::GitHubCliNotFound
                } else {
                    Error::InvalidArgs(format!(
                        "Failed to execute 'gh repo view': {}. Specify --repo explicitly or ensure you're in a Git repository.",
                        e
                    ))
                }
            })?;

        // Check if command failed
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(Error::InvalidArgs(format!(
                "{}. Specify --repo explicitly.",
                stderr.trim()
            )));
        }

        // Parse stdout
        let repo_str = String::from_utf8(output.stdout).map_err(|_| Error::InvalidArgs(
            "Failed to parse repository information from 'gh repo view'. Specify --repo explicitly.".to_string()
        ))?;

        let repo_str = repo_str.trim();
        if repo_str.is_empty() {
            return Err(Error::InvalidArgs(
                "Could not detect repository. Specify --repo explicitly.".to_string(),
            ));
        }

        Ok(repo_str.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::OsString;

    #[test]
    fn test_parse_valid_positional_number() {
        let args = vec![
            OsString::from("gh-discussion-export"),
            OsString::from("123"),
        ];
        let cli = CliArgs::try_parse_from(args).unwrap();
        assert_eq!(cli.number, 123);
        assert_eq!(cli.repo, None);
        assert_eq!(cli.output, None);
    }

    #[test]
    fn test_parse_valid_with_repo_flag() {
        let args = vec![
            OsString::from("gh-discussion-export"),
            OsString::from("456"),
            OsString::from("--repo"),
            OsString::from("rust-lang/rust"),
        ];
        let cli = CliArgs::try_parse_from(args).unwrap();
        assert_eq!(cli.number, 456);
        assert_eq!(cli.repo, Some("rust-lang/rust".to_string()));
        assert_eq!(cli.output, None);
    }

    #[test]
    fn test_parse_valid_with_output_flag() {
        let args = vec![
            OsString::from("gh-discussion-export"),
            OsString::from("789"),
            OsString::from("--repo"),
            OsString::from("owner/repo"),
            OsString::from("--output"),
            OsString::from("custom.md"),
        ];
        let cli = CliArgs::try_parse_from(args).unwrap();
        assert_eq!(cli.number, 789);
        assert_eq!(cli.repo, Some("owner/repo".to_string()));
        assert_eq!(cli.output, Some("custom.md".to_string()));
    }

    #[test]
    fn test_parse_valid_with_short_output_flag() {
        let args = vec![
            OsString::from("gh-discussion-export"),
            OsString::from("999"),
            OsString::from("-o"),
            OsString::from("output.md"),
        ];
        let cli = CliArgs::try_parse_from(args).unwrap();
        assert_eq!(cli.number, 999);
        assert_eq!(cli.repo, None);
        assert_eq!(cli.output, Some("output.md".to_string()));
    }

    #[test]
    fn test_parse_valid_repo_with_git_suffix() {
        let args = vec![
            OsString::from("gh-discussion-export"),
            OsString::from("111"),
            OsString::from("--repo"),
            OsString::from("rust-lang/rust.git"),
        ];
        let cli = CliArgs::try_parse_from(args).unwrap();
        assert_eq!(cli.number, 111);
        assert_eq!(cli.repo, Some("rust-lang/rust.git".to_string()));
    }

    #[test]
    fn test_parse_missing_positional_number() {
        let args = vec![OsString::from("gh-discussion-export")];
        assert!(CliArgs::try_parse_from(args).is_err());
    }

    #[test]
    fn test_parse_invalid_repo_format_no_slash() {
        let args = vec![
            OsString::from("gh-discussion-export"),
            OsString::from("123"),
            OsString::from("--repo"),
            OsString::from("rust-lang"),
        ];
        let cli = CliArgs::try_parse_from(args).unwrap();
        // The validation happens when we call repo_owner() or repo_name()
        assert!(cli.repo_owner().is_err());
    }

    #[test]
    fn test_parse_invalid_repo_format_multiple_slashes() {
        let args = vec![
            OsString::from("gh-discussion-export"),
            OsString::from("123"),
            OsString::from("--repo"),
            OsString::from("rust-lang/rust/extra"),
        ];
        let cli = CliArgs::try_parse_from(args).unwrap();
        // The validation happens when we call repo_owner() or repo_name()
        assert!(cli.repo_owner().is_err());
    }

    #[test]
    fn test_output_path_default() {
        let args = vec![OsString::from("gh-discussion-export"), OsString::from("42")];
        let cli = CliArgs::try_parse_from(args).unwrap();
        assert_eq!(cli.output_path(), "42-discussion.md");
    }

    #[test]
    fn test_output_path_custom() {
        let args = vec![
            OsString::from("gh-discussion-export"),
            OsString::from("42"),
            OsString::from("--output"),
            OsString::from("my-discussion.md"),
        ];
        let cli = CliArgs::try_parse_from(args).unwrap();
        assert_eq!(cli.output_path(), "my-discussion.md");
    }

    #[test]
    fn test_repo_owner_with_explicit_repo() {
        let args = vec![
            OsString::from("gh-discussion-export"),
            OsString::from("123"),
            OsString::from("--repo"),
            OsString::from("rust-lang/rust"),
        ];
        let cli = CliArgs::try_parse_from(args).unwrap();
        assert_eq!(cli.repo_owner().unwrap(), "rust-lang");
    }

    #[test]
    fn test_repo_owner_with_git_suffix() {
        let args = vec![
            OsString::from("gh-discussion-export"),
            OsString::from("123"),
            OsString::from("--repo"),
            OsString::from("rust-lang/rust.git"),
        ];
        let cli = CliArgs::try_parse_from(args).unwrap();
        assert_eq!(cli.repo_owner().unwrap(), "rust-lang");
    }

    #[test]
    fn test_repo_name_with_explicit_repo() {
        let args = vec![
            OsString::from("gh-discussion-export"),
            OsString::from("123"),
            OsString::from("--repo"),
            OsString::from("rust-lang/rust"),
        ];
        let cli = CliArgs::try_parse_from(args).unwrap();
        assert_eq!(cli.repo_name().unwrap(), "rust");
    }

    #[test]
    fn test_repo_name_with_git_suffix() {
        let args = vec![
            OsString::from("gh-discussion-export"),
            OsString::from("123"),
            OsString::from("--repo"),
            OsString::from("rust-lang/rust.git"),
        ];
        let cli = CliArgs::try_parse_from(args).unwrap();
        assert_eq!(cli.repo_name().unwrap(), "rust");
    }

    #[test]
    fn test_repo_owner_invalid_format() {
        let args = vec![
            OsString::from("gh-discussion-export"),
            OsString::from("123"),
            OsString::from("--repo"),
            OsString::from("invalid-format"),
        ];
        let cli = CliArgs::try_parse_from(args).unwrap();
        assert!(cli.repo_owner().is_err());
    }

    #[test]
    fn test_repo_name_invalid_format() {
        let args = vec![
            OsString::from("gh-discussion-export"),
            OsString::from("123"),
            OsString::from("--repo"),
            OsString::from("invalid-format"),
        ];
        let cli = CliArgs::try_parse_from(args).unwrap();
        assert!(cli.repo_name().is_err());
    }

    #[test]
    fn test_parse_zero_number() {
        let args = vec![OsString::from("gh-discussion-export"), OsString::from("0")];
        assert!(CliArgs::try_parse_from(args).is_err());
    }

    #[test]
    fn test_repo_components_empty_owner() {
        let args = vec![
            OsString::from("gh-discussion-export"),
            OsString::from("123"),
            OsString::from("--repo"),
            OsString::from("/repo"),
        ];
        let cli = CliArgs::try_parse_from(args).unwrap();
        assert!(cli.repo_components().is_err());
    }

    #[test]
    fn test_repo_components_empty_name() {
        let args = vec![
            OsString::from("gh-discussion-export"),
            OsString::from("123"),
            OsString::from("--repo"),
            OsString::from("owner/"),
        ];
        let cli = CliArgs::try_parse_from(args).unwrap();
        assert!(cli.repo_components().is_err());
    }

    #[test]
    fn test_repo_components_both_empty() {
        let args = vec![
            OsString::from("gh-discussion-export"),
            OsString::from("123"),
            OsString::from("--repo"),
            OsString::from("/"),
        ];
        let cli = CliArgs::try_parse_from(args).unwrap();
        assert!(cli.repo_components().is_err());
    }

    #[test]
    fn test_repo_components_with_explicit_repo() {
        let args = vec![
            OsString::from("gh-discussion-export"),
            OsString::from("123"),
            OsString::from("--repo"),
            OsString::from("rust-lang/rust"),
        ];
        let cli = CliArgs::try_parse_from(args).unwrap();
        let (owner, name) = cli.repo_components().unwrap();
        assert_eq!(owner, "rust-lang");
        assert_eq!(name, "rust");
    }

    // Helper to create exit status for testing (cross-platform)
    #[cfg(unix)]
    fn exit_status(code: i32) -> std::process::ExitStatus {
        use std::os::unix::process::ExitStatusExt;
        ExitStatusExt::from_raw(code << 8)
    }

    #[cfg(windows)]
    fn exit_status(code: i32) -> std::process::ExitStatus {
        use std::os::windows::process::ExitStatusExt;
        ExitStatusExt::from_raw(code as u32)
    }

    // Helper to create a successful output with stdout content
    fn mock_success_output(stdout: &str) -> std::process::Output {
        std::process::Output {
            status: exit_status(0),
            stdout: stdout.as_bytes().to_vec(),
            stderr: Vec::new(),
        }
    }

    // Helper to create a failed output (non-zero exit code)
    fn mock_failure_output(stderr: &str) -> std::process::Output {
        std::process::Output {
            status: exit_status(1),
            stdout: Vec::new(),
            stderr: stderr.as_bytes().to_vec(),
        }
    }

    #[test]
    fn test_detect_from_git_success() {
        use crate::command_runner::MockCommandRunner;

        let mut mock = MockCommandRunner::new();
        mock.expect_run()
            .times(1)
            .returning(|_, _| Ok(mock_success_output("tatsuya6502/gh-discussion-export")));

        let result = CliArgs::detect_from_git_with_runner(&mock);
        assert_eq!(result.unwrap(), "tatsuya6502/gh-discussion-export");
    }

    #[test]
    fn test_detect_from_git_with_whitespace() {
        use crate::command_runner::MockCommandRunner;

        let mut mock = MockCommandRunner::new();
        mock.expect_run()
            .times(1)
            .returning(|_, _| Ok(mock_success_output("  owner/repo  \n")));

        let result = CliArgs::detect_from_git_with_runner(&mock);
        assert_eq!(result.unwrap(), "owner/repo");
    }

    #[test]
    fn test_detect_from_git_not_found() {
        use crate::command_runner::MockCommandRunner;

        let mut mock = MockCommandRunner::new();
        mock.expect_run().times(1).returning(|_, _| {
            Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "gh not found",
            ))
        });

        let result = CliArgs::detect_from_git_with_runner(&mock);
        assert!(matches!(result, Err(Error::GitHubCliNotFound)));
    }

    #[test]
    fn test_detect_from_git_command_failure() {
        use crate::command_runner::MockCommandRunner;

        let mut mock = MockCommandRunner::new();
        mock.expect_run().times(1).returning(|_, _| {
            Ok(mock_failure_output(
                "not a git repository (or any of the parent directories): .git",
            ))
        });

        let result = CliArgs::detect_from_git_with_runner(&mock);
        assert!(result.is_err());
        if let Err(Error::InvalidArgs(msg)) = result {
            assert!(msg.contains("not a git repository"));
        } else {
            panic!("Expected Error::InvalidArgs");
        }
    }

    #[test]
    fn test_detect_from_git_empty_output() {
        use crate::command_runner::MockCommandRunner;

        let mut mock = MockCommandRunner::new();
        mock.expect_run()
            .times(1)
            .returning(|_, _| Ok(mock_success_output("   \n  ")));

        let result = CliArgs::detect_from_git_with_runner(&mock);
        assert!(result.is_err());
        if let Err(Error::InvalidArgs(msg)) = result {
            assert!(msg.contains("Could not detect repository"));
        } else {
            panic!("Expected Error::InvalidArgs");
        }
    }

    #[test]
    fn test_detect_from_git_invalid_utf8() {
        use crate::command_runner::MockCommandRunner;

        let mut mock = MockCommandRunner::new();
        mock.expect_run().times(1).returning(|_, _| {
            Ok(std::process::Output {
                status: exit_status(0),
                stdout: vec![0xFF, 0xFE, 0xFD], // Invalid UTF-8
                stderr: Vec::new(),
            })
        });

        let result = CliArgs::detect_from_git_with_runner(&mock);
        assert!(result.is_err());
        if let Err(Error::InvalidArgs(msg)) = result {
            assert!(msg.contains("Failed to parse repository information"));
        } else {
            panic!("Expected Error::InvalidArgs");
        }
    }
}
