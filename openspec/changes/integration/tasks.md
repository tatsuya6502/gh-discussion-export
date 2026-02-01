## 1. Main Function Structure

- [ ] 1.1 Update `src/main.rs` with complete main() function
- [ ] 1.2 Import all necessary modules and types
- [ ] 1.3 Set up error handling with Result return type

## 2. CLI Argument Parsing

- [ ] 2.1 Call `CliArgs::parse()` to get arguments
- [ ] 2.2 Extract owner, repo, number, output from arguments
- [ ] 2.3 Determine output path (use arg value or default to `<number>-discussion.md`)

## 3. Authentication Pipeline

- [ ] 3.1 Call `get_github_token()` from auth module
- [ ] 3.2 Match on Result and print error message if failed
- [ ] 3.3 Exit with status code 1 on authentication failure

## 4. GitHub Client Setup

- [ ] 4.1 Create `GitHubClient::new(token)` instance
- [ ] 4.2 Verify client creation succeeds

## 5. Discussion Fetching Pipeline

- [ ] 5.1 Call `fetch_discussion(&client, &owner, &repo, number)`
- [ ] 5.2 Match on Result and print error message if failed
- [ ] 5.3 Exit with status code 1 on fetch failure
- [ ] 5.4 Verify Discussion object is returned

## 6. Markdown Output Pipeline

- [ ] 6.1 Call `format_discussion(&discussion, &owner, &repo)` to generate Markdown
- [ ] 6.2 Call `write_output(&markdown, &output_path)` to write file
- [ ] 6.3 Match on Result and print error message if failed
- [ ] 6.4 Exit with status code 1 on write failure

## 7. Success Path

- [ ] 7.1 Print success message with output path
- [ ] 7.2 Exit with status code 0

## 8. Error Handling

- [ ] 8.1 Ensure all Result matches handle both Ok and Err cases
- [ ] 8.2 Print user-friendly error messages (not debug output)
- [ ] 8.3 Use `eprintln!` for errors (stderr)
- [ ] 8.4 Include remediation hints where applicable (e.g., "Run 'gh auth login'")

## 9. Compilation and Verification

- [ ] 9.1 Run `cargo build` and verify no compilation errors
- [ ] 9.2 Verify all modules are properly imported
- [ ] 9.3 Verify function signatures match between modules
- [ ] 9.4 Fix any type mismatches or missing imports

## 10. Basic Manual Testing

- [ ] 10.1 Run `cargo run -- --help` to verify CLI help displays
- [ ] 10.2 Run with invalid args to verify error handling
- [ ] 10.3 Run with valid small discussion to verify end-to-end (if token available)
- [ ] 10.4 Test default output path: verify file is created as `<number>-discussion.md` when no `--output` arg provided
