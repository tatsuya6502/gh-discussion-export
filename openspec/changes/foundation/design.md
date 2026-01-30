## Context

The GitHub Discussion Exporter is a new Rust CLI tool starting from a blank project. The tool must integrate with GitHub's GraphQL API and requires GitHub CLI authentication. This change establishes the foundational infrastructure that all subsequent changes will build upon.

**Constraints:**
- Must use GitHub CLI for authentication (no direct PAT management)
- Must provide clear error messages for authentication failures
- Project structure should support modular development across multiple changes

## Goals / Non-Goals

**Goals:**
- Establish project structure and module organization
- Provide reusable CLI argument parsing
- Integrate GitHub CLI authentication
- Define application-specific error types
- Enable unit testing for each component

**Non-Goals:**
- GraphQL API client (handled by `graphql-client` change)
- Data fetching logic (handled by `discussion-fetch` change)
- Markdown output generation (handled by `markdown-output` change)
- End-to-end functionality (assembled in `integration` change)

## Decisions

### Module Structure
Organize code into separate modules for clear separation of concerns:
- `cli.rs` - CLI argument parsing and validation
- `auth.rs` - GitHub CLI authentication
- `error.rs` - Application error types

**Rationale:** Each module has a single responsibility and can be tested independently. This structure supports the horizontal slicing approach where each change adds discrete functionality.

### CLI Framework: `clap` v4
Use `clap` with derive macros for type-safe argument parsing.

**Rationale:** `clap` is the de-facto standard for Rust CLI tools. The derive API provides compile-time verification and cleaner code compared to the builder pattern. Alternative `structopt` is deprecated.

### Error Handling: `thiserror`
Use `thiserror` for ergonomic error type definitions with the following error variants:
- `GitHubCliNotFound` - GitHub CLI is not installed on the system
- `Authentication` - User is not authenticated with GitHub CLI
- `InvalidArgs(String)` - Invalid command-line arguments
- `Io(std::io::Error)` - Wrapped I/O errors with automatic conversion

**Rationale:** `thiserror` simplifies error enum creation with macros for `Display` and `Error` traits. It integrates seamlessly with `anyhow` for error propagation while maintaining structured error types for users. The `GitHubCliNotFound` variant provides a distinct error from `Authentication`, allowing users to understand whether they need to install GitHub CLI or authenticate with it.

### Authentication Strategy
Call `gh auth token` command and capture stdout.

**Rationale:** The GitHub CLI already handles token management, including multiple auth contexts and token refresh. Leveraging `gh` avoids implementing token storage, rotation, and user interaction. Alternative (reading `~/.config/gh/hosts.yml`) would be fragile and format-dependent.

**Alternative considered:** Direct GitHub OAuth flow - Rejected due to complexity and requirement for interactive authentication.

### Test Organization
Create unit test modules alongside implementation code using `#[cfg(test)]`.

**Rationale:** Rust's convention of co-locating tests with implementation keeps tests close to the code they test and ensures they're run with `cargo test`.

### Test Mocking Strategy: Dependency Injection with `mockall`
Use a thin `CommandRunner` trait abstraction over `std::process::Command` to enable mocking in tests without environment variable manipulation.

**Rationale:** Direct mocking of `std::process::Command` is not possible. The traditional approach of overriding `PATH` environment variable in tests requires:
- Running tests with `--test-threads=1` (serial execution only)
- `unsafe` blocks for environment variable manipulation
- Race conditions when multiple tests modify `PATH`
- Fragile test setup with temporary files

By using `mockall` with a dependency injection pattern:
- Tests can run in parallel without race conditions
- No `unsafe` blocks required
- Mock behavior is explicit and clear in test code
- Better error messages from mock expectations
- Reusable pattern for other modules that need to execute commands

**Implementation:**
- `CommandRunner` trait with `run(&self, program: &str, args: &[String])` method
- `StdCommandRunner` production implementation using `std::process::Command`
- `#[cfg_attr(test, automock)]` generates `MockCommandRunner` in tests
- `get_github_token()` accepts `&dyn CommandRunner` parameter
- Tests inject `MockCommandRunner` with `.expect()` and `.returning()`

**Alternative considered:** PATH manipulation with temporary scripts - Rejected due to thread safety issues and `unsafe` code requirements.

## Risks / Trade-offs

| Risk | Mitigation |
|------|------------|
| `gh` not installed on user's system | Clear error message directing users to install GitHub CLI |
| `gh` not authenticated | Catch token retrieval failure and exit with helpful message |
| Future changes need to refactor module structure | Keep modules loosely coupled through clear interfaces; anticipate that some refactoring is expected during development |

## Migration Plan

Not applicable - this is initial project setup.

## Open Questions

None - this is foundational infrastructure with clear requirements from specs.md.
