## 1. CLI Argument Structure Changes

- [x] 1.1 Update `CliArgs` struct to use positional argument for `number` field
- [x] 1.2 Change `owner` and `repo` fields to single optional `repo: Option<String>` field
- [x] 1.3 Add short form `-o` for `output` field using `short` attribute
- [x] 1.4 Remove legacy `--owner` and `--repo` flags, add new `--repo <OWNER/REPO>` flag
- [x] 1.5 Update help text and about section to reflect new argument structure

## 2. Repo Validation Logic

- [x] 2.1 Implement `validate_repo_format()` function to check OWNER/REPO format
- [x] 2.2 Add validation for exactly one `/` separator in repo argument
- [x] 2.3 Add validation for non-empty owner and repo parts
- [x] 2.4 Support optional trailing `.git` suffix in repo argument
- [x] 2.5 Update clap attributes to use custom validator for `--repo` flag

## 3. Git Repository Detection

- [x] 3.1 Implement `detect_from_git()` function to execute `gh repo view`
- [x] 3.2 Add command execution using `std::process::Command`
- [x] 3.3 Implement `gh_repo_owner()` method to extract owner from detected or explicit repo
- [x] 3.4 Implement `gh_repo_name()` method to extract name from detected or explicit repo
- [x] 3.5 Add error handling for `gh` command not found or execution failures
- [x] 3.6 Add error handling for invalid JSON output from `gh repo view`

## 4. Integration with Main Application

- [x] 4.1 Update `src/main.rs` to call new `repo_owner()` and `repo_name()` methods
- [x] 4.2 Replace direct `cli.owner` and `cli.repo` field access with method calls
- [x] 4.3 Update error handling to use new error types from repo detection

## 5. Unit Tests

- [x] 5.1 Add test for valid positional argument parsing
- [x] 5.2 Add test for missing positional argument (should fail)
- [x] 5.3 Add test for valid repo format "owner/repo"
- [x] 5.4 Add test for valid repo format with ".git" suffix
- [x] 5.5 Add test for invalid repo format (no slash, multiple slashes)
- [x] 5.6 Add test for empty repo argument
- [x] 5.7 Add test for `-o` short form flag parsing
- [x] 5.8 Add test for explicit `--repo` taking precedence over git detection
- [x] 5.9 Add test for `output_path()` default behavior with positional number

## 6. Integration Tests

**Note**: No integration tests exist in the codebase (no `tests/` directory). All functionality is tested via unit tests with `MockCommandRunner`. These items are marked as N/A.

- N/A 6.1 Update existing integration tests to use new argument structure
- N/A 6.2 Add integration test for automatic repo detection in Git repository
- N/A 6.3 Add integration test for error when outside Git repository without `--repo`
- N/A 6.4 Add integration test for `gh` command not available scenario

## 7. Documentation Updates

- [x] 7.1 Update README with new CLI usage examples
- [x] 7.2 Add migration guide section for users upgrading from old CLI format (removed - not needed as only user is developer)
- [x] 7.3 Update help text examples to show positional argument usage
- [x] 7.4 Document automatic repository detection feature
- [x] 7.5 Update any CI/CD scripts or examples in repository

**Note**: No CI/CD scripts exist in the repository, so task 7.5 is marked as complete (not applicable). Migration guide was removed from README as the only current user is the developer.

## 8. Code Cleanup

- [x] 8.1 Remove old validation functions (`validate_non_empty_string` for owner/repo)
- [x] 8.2 Remove unused imports after refactoring
- [x] 8.3 Run `cargo fmt` to ensure consistent formatting
- [x] 8.4 Run `cargo clippy` to verify no warnings
- [x] 8.5 Run `cargo test` to ensure all tests pass

**Note**: The old validation functions were replaced with inline validation logic in the `repo_owner()` and `repo_name()` methods. No unused imports remain.
