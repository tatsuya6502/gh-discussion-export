## Why

The GitHub Discussion Exporter needs a solid foundation to handle command-line arguments, authentication via GitHub CLI, and error handling throughout the application. This change establishes the project structure and core infrastructure that all subsequent changes depend on.

## What Changes

- Add `clap` dependency for CLI argument parsing
- Implement command-line argument structure (`--owner`, `--repo`, `--number`, `--output`)
- Create authentication module that calls `gh auth token` to retrieve access token
- Define error types using `thiserror` for application-specific errors
- Set up module structure for the project
- Add unit tests for CLI parsing and authentication

## Capabilities

### New Capabilities
- `cli-parsing`: Command-line argument parsing and validation
- `gh-auth`: GitHub CLI authentication integration
- `error-handling`: Application-specific error types and result handling

### Modified Capabilities
- None

## Impact

- New dependencies: `clap`, `thiserror`
- New modules: `cli.rs`, `auth.rs`, `error.rs`
- `main.rs` will contain basic skeleton that will be expanded in integration change
- No breaking changes (project is in initial state)
