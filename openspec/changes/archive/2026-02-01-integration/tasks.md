## 1. Main Function Structure

- [x] 1.1 Update `src/main.rs` with complete main() function
- [x] 1.2 Import all necessary modules and types
- [x] 1.3 Set up error handling with Result return type

## 2. CLI Argument Parsing

- [x] 2.1 Call `CliArgs::parse()` to get arguments
- [x] 2.2 Extract owner, repo, number, output from arguments
- [x] 2.3 Determine output path (use arg value or default to `<number>-discussion.md`)

## 3. Authentication Pipeline

- [x] 3.1 Call `get_github_token()` from auth module
- [x] 3.2 Match on Result and print error message if failed
- [x] 3.3 Exit with status code 1 on authentication failure

## 4. GitHub Client Setup

- [x] 4.1 Create `GitHubClient::new(token)` instance
- [x] 4.2 Verify client creation succeeds

## 5. Discussion Fetching Pipeline

- [x] 5.1 Call `fetch_discussion(&client, &owner, &repo, number)`
- [x] 5.2 Match on Result and print error message if failed
- [x] 5.3 Exit with status code 1 on fetch failure
- [x] 5.4 Verify Discussion object is returned

## 6. Markdown Output Pipeline

- [x] 6.1 Call `format_discussion(&discussion, &owner, &repo)` to generate Markdown
- [x] 6.2 Call `write_output(&markdown, &output_path)` to write file
- [x] 6.3 Match on Result and print error message if failed
- [x] 6.4 Exit with status code 1 on write failure

## 7. Success Path

- [x] 7.1 Print success message with output path
- [x] 7.2 Exit with status code 0

## 8. Error Handling

- [x] 8.1 Ensure all Result matches handle both Ok and Err cases
- [x] 8.2 Print user-friendly error messages (not debug output)
- [x] 8.3 Use `eprintln!` for errors (stderr)
- [x] 8.4 Include remediation hints where applicable (e.g., "Run 'gh auth login'")

## 9. Compilation and Verification

- [x] 9.1 Run `cargo build` and verify no compilation errors
- [x] 9.2 Verify all modules are properly imported
- [x] 9.3 Verify function signatures match between modules
- [x] 9.4 Fix any type mismatches or missing imports

## 10. Basic Manual Testing

- [x] 10.1 Run `cargo run -- --help` to verify CLI help displays
- [x] 10.2 Run with invalid args to verify error handling
- [x] 10.3 Run with valid small discussion to verify end-to-end (if token available)
- [x] 10.4 Test default output path: verify file is created as `<number>-discussion.md` when no `--output` arg provided
