## 1. CLI Argument Structure Changes

- [ ] 1.1 Update `CliArgs` struct to use positional argument for `number` field
- [ ] 1.2 Change `owner` and `repo` fields to single optional `repo: Option<String>` field
- [ ] 1.3 Add short form `-o` for `output` field using `short` attribute
- [ ] 1.4 Remove legacy `--owner` and `--repo` flags, add new `--repo <OWNER/REPO>` flag
- [ ] 1.5 Update help text and about section to reflect new argument structure

## 2. Repo Validation Logic

- [ ] 2.1 Implement `validate_repo_format()` function to check OWNER/REPO format
- [ ] 2.2 Add validation for exactly one `/` separator in repo argument
- [ ] 2.3 Add validation for non-empty owner and repo parts
- [ ] 2.4 Support optional trailing `.git` suffix in repo argument
- [ ] 2.5 Update clap attributes to use custom validator for `--repo` flag

## 3. Git Repository Detection

- [ ] 3.1 Implement `detect_from_git()` function to execute `gh repo view`
- [ ] 3.2 Add command execution using `std::process::Command`
- [ ] 3.3 Implement `gh_repo_owner()` method to extract owner from detected or explicit repo
- [ ] 3.4 Implement `gh_repo_name()` method to extract name from detected or explicit repo
- [ ] 3.5 Add error handling for `gh` command not found or execution failures
- [ ] 3.6 Add error handling for invalid JSON output from `gh repo view`

## 4. Integration with Main Application

- [ ] 4.1 Update `src/main.rs` to call new `repo_owner()` and `repo_name()` methods
- [ ] 4.2 Replace direct `cli.owner` and `cli.repo` field access with method calls
- [ ] 4.3 Update error handling to use new error types from repo detection

## 5. Unit Tests

- [ ] 5.1 Add test for valid positional argument parsing
- [ ] 5.2 Add test for missing positional argument (should fail)
- [ ] 5.3 Add test for valid repo format "owner/repo"
- [ ] 5.4 Add test for valid repo format with ".git" suffix
- [ ] 5.5 Add test for invalid repo format (no slash, multiple slashes)
- [ ] 5.6 Add test for empty repo argument
- [ ] 5.7 Add test for `-o` short form flag parsing
- [ ] 5.8 Add test for explicit `--repo` taking precedence over git detection
- [ ] 5.9 Add test for `output_path()` default behavior with positional number

## 6. Integration Tests

- [ ] 6.1 Update existing integration tests to use new argument structure
- [ ] 6.2 Add integration test for automatic repo detection in Git repository
- [ ] 6.3 Add integration test for error when outside Git repository without `--repo`
- [ ] 6.4 Add integration test for `gh` command not available scenario

## 7. Documentation Updates

- [ ] 7.1 Update README with new CLI usage examples
- [ ] 7.2 Add migration guide section for users upgrading from old CLI format
- [ ] 7.3 Update help text examples to show positional argument usage
- [ ] 7.4 Document automatic repository detection feature
- [ ] 7.5 Update any CI/CD scripts or examples in repository

## 8. Code Cleanup

- [ ] 8.1 Remove old validation functions (`validate_non_empty_string` for owner/repo)
- [ ] 8.2 Remove unused imports after refactoring
- [ ] 8.3 Run `cargo fmt` to ensure consistent formatting
- [ ] 8.4 Run `cargo clippy` to verify no warnings
- [ ] 8.5 Run `cargo test` to ensure all tests pass
