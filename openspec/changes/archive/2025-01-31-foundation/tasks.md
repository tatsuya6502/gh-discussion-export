## 1. Project Setup

- [x] 1.1 Add dependencies to `Cargo.toml`: `clap` (v4 with derive feature), `thiserror`
- [x] 1.2 Add `mockall` to dev-dependencies for test mocking
- [x] 1.3 Create module structure in `src/`: `cli.rs`, `auth.rs`, `error.rs`, `command_runner.rs`
- [x] 1.4 Update `main.rs` with basic module declarations
- [x] 1.5 Update `src/lib.rs` with `pub mod command_runner`

## 2. Error Types

- [x] 2.1 Define `Error` enum in `src/error.rs` with variants: `Authentication`, `InvalidArgs`, `Io`
- [x] 2.2 Implement `Display` trait for `Error` with descriptive messages
- [x] 2.3 Implement `std::error::Error` trait for `Error`
- [x] 2.4 Implement `From<std::io::Error>` for `Error` to wrap I/O errors
- [x] 2.5 Create type alias `type Result<T> = std::result::Result<T, Error>;`

## 3. Command Runner Abstraction

- [x] 3.1 Create `src/command_runner.rs` module
- [x] 3.2 Define `CommandRunner` trait with `run(&self, program: &str, args: &[String])` method
- [x] 3.3 Add `#[cfg_attr(test, automock)]` to trait for automatic mock generation
- [x] 3.4 Implement `StdCommandRunner` struct using `std::process::Command`
- [x] 3.5 Ensure `use mockall::automock;` is guarded by `#[cfg(test)]`

## 4. CLI Parsing Module

- [x] 4.1 Define `CliArgs` struct in `src/cli.rs` with fields: `owner`, `repo`, `number`, `output`
- [x] 4.2 Derive `Parser` trait from clap for `CliArgs`
- [x] 4.3 Add argument attributes: required flags, help text, default value for output
- [x] 4.4 Implement unit tests for valid argument parsing
- [x] 4.5 Implement unit tests for missing required arguments
- [x] 4.6 Implement unit tests for output path default value

## 5. Authentication Module

- [x] 5.1 Implement `get_github_token(command_runner: &dyn CommandRunner)` function in `src/auth.rs`
- [x] 5.2 Use `command_runner.run("gh", &["auth".to_string(), "token".to_string()])` to call GitHub CLI
- [x] 5.3 Capture stdout and trim whitespace from token output
- [x] 5.4 Return `Result<String>` with `Error::Authentication` on failure
- [x] 5.5 Check for empty token and return error if found
- [x] 5.6 Implement unit test for successful token retrieval using `MockCommandRunner`
- [x] 5.7 Implement unit test for `gh` not found using `MockCommandRunner` returning `ErrorKind::NotFound`
- [x] 5.8 Implement unit test for authentication failure (non-zero exit code) using `MockCommandRunner`
- [x] 5.9 Implement unit test for empty token using `MockCommandRunner`
- [x] 5.10 Remove all `unsafe` blocks and PATH environment variable manipulation from tests

## 6. Main Module Skeleton

- [x] 6.1 Add basic `main()` function that declares `CliArgs` and parses arguments
- [x] 6.2 Add stub for token retrieval with `&StdCommandRunner` (to be connected in integration change)
- [x] 6.3 Ensure `cargo run -- --help` displays proper usage information
