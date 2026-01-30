use clap::Parser;

/// Command-line arguments for GitHub Discussion Export
#[derive(Parser, Debug)]
#[command(name = "gh-discussion-export")]
#[command(about = "Export GitHub Discussion to Markdown", version = "0.1.0")]
pub struct CliArgs {
    /// GitHub repository owner
    #[arg(long, required = true, help = "GitHub repository owner", value_parser = validate_non_empty_string)]
    pub owner: String,

    /// GitHub repository name
    #[arg(long, required = true, help = "GitHub repository name", value_parser = validate_non_empty_string)]
    pub repo: String,

    /// Discussion number
    #[arg(long, required = true, help = "Discussion number")]
    pub number: u64,

    /// Output file path (default: <number>-discussion.md)
    #[arg(long, help = "Output file path (default: <number>-discussion.md)")]
    pub output: Option<String>,
}

/// Custom validator to reject empty strings
fn validate_non_empty_string(s: &str) -> Result<String, String> {
    if s.trim().is_empty() {
        Err("Argument cannot be empty".to_string())
    } else {
        Ok(s.to_string())
    }
}

impl CliArgs {
    /// Get the output file path, using default if not specified
    pub fn output_path(&self) -> String {
        match &self.output {
            Some(path) => path.clone(),
            None => format!("{}-discussion.md", self.number),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::OsString;

    #[test]
    fn test_parse_valid_minimal_args() {
        let args = vec![
            OsString::from("gh-discussion-export"),
            OsString::from("--owner"),
            OsString::from("rust-lang"),
            OsString::from("--repo"),
            OsString::from("rust"),
            OsString::from("--number"),
            OsString::from("123"),
        ];
        let cli = CliArgs::try_parse_from(args).unwrap();
        assert_eq!(cli.owner, "rust-lang");
        assert_eq!(cli.repo, "rust");
        assert_eq!(cli.number, 123);
        assert_eq!(cli.output, None);
    }

    #[test]
    fn test_parse_valid_args_with_output() {
        let args = vec![
            OsString::from("gh-discussion-export"),
            OsString::from("--owner"),
            OsString::from("rust-lang"),
            OsString::from("--repo"),
            OsString::from("rust"),
            OsString::from("--number"),
            OsString::from("456"),
            OsString::from("--output"),
            OsString::from("custom.md"),
        ];
        let cli = CliArgs::try_parse_from(args).unwrap();
        assert_eq!(cli.owner, "rust-lang");
        assert_eq!(cli.repo, "rust");
        assert_eq!(cli.number, 456);
        assert_eq!(cli.output, Some("custom.md".to_string()));
    }

    #[test]
    fn test_parse_missing_owner() {
        let args = vec![
            OsString::from("gh-discussion-export"),
            OsString::from("--repo"),
            OsString::from("rust"),
            OsString::from("--number"),
            OsString::from("123"),
        ];
        assert!(CliArgs::try_parse_from(args).is_err());
    }

    #[test]
    fn test_parse_missing_repo() {
        let args = vec![
            OsString::from("gh-discussion-export"),
            OsString::from("--owner"),
            OsString::from("rust-lang"),
            OsString::from("--number"),
            OsString::from("123"),
        ];
        assert!(CliArgs::try_parse_from(args).is_err());
    }

    #[test]
    fn test_parse_missing_number() {
        let args = vec![
            OsString::from("gh-discussion-export"),
            OsString::from("--owner"),
            OsString::from("rust-lang"),
            OsString::from("--repo"),
            OsString::from("rust"),
        ];
        assert!(CliArgs::try_parse_from(args).is_err());
    }

    #[test]
    fn test_parse_invalid_number() {
        let args = vec![
            OsString::from("gh-discussion-export"),
            OsString::from("--owner"),
            OsString::from("rust-lang"),
            OsString::from("--repo"),
            OsString::from("rust"),
            OsString::from("--number"),
            OsString::from("not-a-number"),
        ];
        assert!(CliArgs::try_parse_from(args).is_err());
    }

    #[test]
    fn test_parse_empty_owner() {
        let args = vec![
            OsString::from("gh-discussion-export"),
            OsString::from("--owner"),
            OsString::from(""),
            OsString::from("--repo"),
            OsString::from("rust"),
            OsString::from("--number"),
            OsString::from("123"),
        ];
        assert!(CliArgs::try_parse_from(args).is_err());
    }

    #[test]
    fn test_parse_whitespace_only_owner() {
        let args = vec![
            OsString::from("gh-discussion-export"),
            OsString::from("--owner"),
            OsString::from("   "),
            OsString::from("--repo"),
            OsString::from("rust"),
            OsString::from("--number"),
            OsString::from("123"),
        ];
        assert!(CliArgs::try_parse_from(args).is_err());
    }

    #[test]
    fn test_parse_empty_repo() {
        let args = vec![
            OsString::from("gh-discussion-export"),
            OsString::from("--owner"),
            OsString::from("rust-lang"),
            OsString::from("--repo"),
            OsString::from(""),
            OsString::from("--number"),
            OsString::from("123"),
        ];
        assert!(CliArgs::try_parse_from(args).is_err());
    }

    #[test]
    fn test_output_path_default() {
        let args = vec![
            OsString::from("gh-discussion-export"),
            OsString::from("--owner"),
            OsString::from("rust-lang"),
            OsString::from("--repo"),
            OsString::from("rust"),
            OsString::from("--number"),
            OsString::from("789"),
        ];
        let cli = CliArgs::try_parse_from(args).unwrap();
        assert_eq!(cli.output_path(), "789-discussion.md");
    }

    #[test]
    fn test_output_path_custom() {
        let args = vec![
            OsString::from("gh-discussion-export"),
            OsString::from("--owner"),
            OsString::from("rust-lang"),
            OsString::from("--repo"),
            OsString::from("rust"),
            OsString::from("--number"),
            OsString::from("789"),
            OsString::from("--output"),
            OsString::from("my-discussion.md"),
        ];
        let cli = CliArgs::try_parse_from(args).unwrap();
        assert_eq!(cli.output_path(), "my-discussion.md");
    }
}
