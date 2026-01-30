//! Command runner abstraction for testing
//!
//! This module provides a trait that abstracts command execution, allowing
//! mock implementations in tests while using the standard `std::process::Command`
//! in production code.

#[cfg(test)]
use mockall::automock;

/// Abstraction over running external commands.
///
/// This trait allows mocking command execution in tests, removing the need
/// for fragile environment variable manipulation.
#[cfg_attr(test, automock)]
pub trait CommandRunner: Send + Sync {
    /// Runs a command with the given program and arguments.
    ///
    /// # Arguments
    ///
    /// * `program` - The command to execute (e.g., "gh")
    /// * `args` - Slice of arguments to pass to the command
    ///
    /// # Returns
    ///
    /// Returns `Ok(Output)` containing the command's stdout, stderr, and exit status.
    /// Returns `Err(std::io::Error)` if the command could not be executed.
    fn run(&self, program: &str, args: &[String]) -> std::io::Result<std::process::Output>;
}

/// Production implementation of `CommandRunner` using `std::process::Command`.
pub struct StdCommandRunner;

impl CommandRunner for StdCommandRunner {
    fn run(&self, program: &str, args: &[String]) -> std::io::Result<std::process::Output> {
        std::process::Command::new(program).args(args).output()
    }
}
