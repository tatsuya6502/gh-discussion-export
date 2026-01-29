## 1. Test Infrastructure Setup

- [ ] 1.1 Create `tests/` directory
- [ ] 1.2 Create `tests/common.rs` with helper functions
- [ ] 1.3 Add test configuration to `Cargo.toml` if needed

## 2. Common Test Utilities

- [ ] 2.1 Implement `run_export(owner, repo, number, output_path)` helper function
- [ ] 2.2 Implement `assert_output_exists(path)` helper function
- [ ] 2.3 Implement `assert_output_contains(path, text)` helper function
- [ ] 2.4 Implement `assert_output_format(path)` validation function
- [ ] 2.5 Implement cleanup helper to remove test output files
- [ ] 2.6 Implement retry helper for transient network failures (with exponential backoff and max attempts, applied to network calls in `run_export`)

## 3. Basic E2E Test

- [ ] 3.1 Create `tests/basic_e2e_test.rs`
- [ ] 3.2 Add test for small discussion export (< 10 comments)
- [ ] 3.3 Use discussion from public repo (e.g., rust-lang/rust)
- [ ] 3.4 Assert output file exists
- [ ] 3.5 Assert output contains expected title
- [ ] 3.6 Assert output contains original post
- [ ] 3.7 Assert output contains all comments
- [ ] 3.8 Assert heading hierarchy is correct
- [ ] 3.9 Clean up output file after test

## 4. Pagination Test

- [ ] 4.1 Create `tests/pagination_test.rs`
- [ ] 4.2 Add test for large discussion (> 100 comments)
- [ ] 4.3 Find a discussion with known large thread
- [ ] 4.4 Assert all comments are present
- [ ] 4.5 Assert comments are ordered chronologically
- [ ] 4.6 Mark as `#[ignore]` to avoid hitting rate limits on every run

## 5. Deleted User Test

- [ ] 5.1 Create `tests/edge_cases_test.rs`
- [ ] 5.2 Add test for discussion with deleted users
- [ ] 5.3 Assert `<deleted>` placeholder appears where appropriate
- [ ] 5.4 Assert no errors occur

## 6. Output Format Validation Test

- [ ] 6.1 Create `tests/format_validation_test.rs`
- [ ] 6.2 Add test for header format validation
- [ ] 6.3 Assert all header fields present: title, discussion ID, URL, created at, author
- [ ] 6.4 Assert `---` separator after header
- [ ] 6.5 Add test for heading hierarchy validation
- [ ] 6.6 Assert `##` for sections, `###` for comments, `####` for replies
- [ ] 6.7 Add test for author format validation
- [ ] 6.8 Assert italic format `_author: ..._`

## 7. Error Path Tests

- [ ] 7.1 Create `tests/error_tests.rs`
- [ ] 7.2 Add test for invalid discussion number
- [ ] 7.3 Assert non-zero exit status
- [ ] 7.4 Assert error message contains "not found"
- [ ] 7.5 Add test for unauthenticated execution (temporarily make `gh` command unavailable by modifying PATH or running in subprocess without gh in PATH)
- [ ] 7.6 Assert error message mentions authentication
- [ ] 7.7 Document that this test requires special handling and should be marked as manual or `#[ignore]` with clear instructions

## 8. Idempotency Test

- [ ] 8.1 Create `tests/idempotency_test.rs`
- [ ] 8.2 Export same discussion to two different files
- [ ] 8.3 Read both files and compare byte-for-byte
- [ ] 8.4 Assert files are identical

## 9. Test Documentation

- [ ] 9.1 Add comments explaining test requirements (auth, network access)
- [ ] 9.2 Document which tests are marked `#[ignore]` and why
- [ ] 9.3 Document test discussion IDs and why they were chosen
- [ ] 9.4 Add README in tests/ directory if needed

## 10. Running Tests

- [ ] 10.1 Run `cargo test` to verify all unit tests pass
- [ ] 10.2 Run `cargo test -- --ignored` to verify integration tests
- [ ] 10.3 Fix any failing tests
- [ ] 10.4 Ensure tests can run in CI/CD if applicable
