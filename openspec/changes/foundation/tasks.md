## 1. Project Setup

- [x] 1.1 Add dependencies to `Cargo.toml`: `clap` (v4 with derive feature), `thiserror`
- [x] 1.2 Create module structure in `src/`: `cli.rs`, `auth.rs`, `error.rs`
- [x] 1.3 Update `main.rs` with basic module declarations

## 2. Error Types

- [x] 2.1 Define `Error` enum in `src/error.rs` with variants: `Authentication`, `InvalidArgs`, `Io`
- [x] 2.2 Implement `Display` trait for `Error` with descriptive messages
- [x] 2.3 Implement `std::error::Error` trait for `Error`
- [x] 2.4 Implement `From<std::io::Error>` for `Error` to wrap I/O errors
- [x] 2.5 Create type alias `type Result<T> = std::result::Result<T, Error>;`

## 3. CLI Parsing Module

- [x] 3.1 Define `CliArgs` struct in `src/cli.rs` with fields: `owner`, `repo`, `number`, `output`
- [x] 3.2 Derive `Parser` trait from clap for `CliArgs`
- [x] 3.3 Add argument attributes: required flags, help text, default value for output
- [x] 3.4 Implement unit tests for valid argument parsing
- [x] 3.5 Implement unit tests for missing required arguments
- [x] 3.6 Implement unit tests for output path default value

## 4. Authentication Module

- [x] 4.1 Implement `get_github_token()` function in `src/auth.rs`
- [x] 4.2 Use `std::process::Command` to call `gh auth token`
- [x] 4.3 Capture stdout and trim whitespace from token output
- [x] 4.4 Return `Result<String>` with `Error::Authentication` on failure
- [x] 4.5 Check for empty token and return error if found
- [x] 4.6 Implement unit tests for successful token retrieval (mock `gh` command)
- [x] 4.7 Implement unit tests for `gh` not found (mock Command behavior)
- [x] 4.8 Implement unit tests for authentication failure (non-zero exit code)

## 5. Main Module Skeleton

- [x] 5.1 Add basic `main()` function that declares `CliArgs` and parses arguments
- [x] 5.2 Add stub for token retrieval (to be connected in integration change)
- [x] 5.3 Ensure `cargo run -- --help` displays proper usage information
