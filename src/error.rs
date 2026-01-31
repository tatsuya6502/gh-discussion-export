use thiserror::Error;

/// Application-specific error types
#[derive(Error, Debug)]
pub enum Error {
    /// GitHub CLI not installed
    #[error("GitHub CLI not found. Install from https://cli.github.com/")]
    GitHubCliNotFound,

    /// Authentication failed via GitHub CLI
    #[error("Failed to authenticate with GitHub CLI. Run 'gh auth login' to authenticate.")]
    Authentication,

    /// Invalid command-line arguments
    #[error("Invalid arguments: {0}")]
    InvalidArgs(String),

    /// I/O error
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// HTTP request error
    #[error("HTTP request failed: {0}")]
    Http(String),

    /// GraphQL error response
    #[error("GraphQL error: {0}")]
    GraphQL(String),

    /// JSON parsing error
    #[error("Failed to parse response: {0}")]
    JsonParse(String),

    /// Rate limit exceeded
    #[error("GitHub API rate limit exceeded. Please wait before trying again.")]
    RateLimit,
}

/// Convenient Result type alias for application errors
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_github_cli_not_found_display() {
        let err = Error::GitHubCliNotFound;
        assert_eq!(
            err.to_string(),
            "GitHub CLI not found. Install from https://cli.github.com/"
        );
    }

    #[test]
    fn test_error_authentication_display() {
        let err = Error::Authentication;
        assert_eq!(
            err.to_string(),
            "Failed to authenticate with GitHub CLI. Run 'gh auth login' to authenticate."
        );
    }

    #[test]
    fn test_error_invalid_args_display() {
        let err = Error::InvalidArgs("missing required flag".to_string());
        assert_eq!(err.to_string(), "Invalid arguments: missing required flag");
    }

    #[test]
    fn test_error_io_display() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "test file");
        let err = Error::from(io_err);
        assert_eq!(err.to_string(), "I/O error: test file");
    }

    #[test]
    fn test_error_from_io_error() {
        let io_err = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "access denied");
        let err: Error = io_err.into();
        assert!(matches!(err, Error::Io(_)));
    }

    #[test]
    fn test_result_type_alias() {
        fn test_function() -> Result<String> {
            Ok("test".to_string())
        }
        assert!(test_function().is_ok());
    }

    #[test]
    fn test_result_type_alias_with_error() {
        fn test_function() -> Result<String> {
            Err(Error::Authentication)
        }
        assert!(test_function().is_err());
    }

    #[test]
    fn test_error_http_display() {
        let err = Error::Http("Connection failed".to_string());
        assert_eq!(err.to_string(), "HTTP request failed: Connection failed");
    }

    #[test]
    fn test_error_graphql_display() {
        let err = Error::GraphQL("Syntax error".to_string());
        assert_eq!(err.to_string(), "GraphQL error: Syntax error");
    }

    #[test]
    fn test_error_json_parse_display() {
        let err = Error::JsonParse("Unexpected token".to_string());
        assert_eq!(
            err.to_string(),
            "Failed to parse response: Unexpected token"
        );
    }

    #[test]
    fn test_error_rate_limit_display() {
        let err = Error::RateLimit;
        assert_eq!(
            err.to_string(),
            "GitHub API rate limit exceeded. Please wait before trying again."
        );
    }
}
