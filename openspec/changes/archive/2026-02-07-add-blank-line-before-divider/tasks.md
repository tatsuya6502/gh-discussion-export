## 1. Code Changes

- [x] 1.1 Update `generate_original_post()` format string to add blank line before `---` separator

## 2. Test Updates

- [x] 2.1 Update `test_generate_original_post` to expect blank line before divider
- [x] 2.2 Update any other tests affected by the divider format change

## 3. Documentation Updates

- [x] 3.1 Update output format example in README.md to add blank lines before `---` separators
- [x] 3.2 Create or update CHANGELOG.md with entry for this bug fix

## 4. Verification

- [x] 4.1 Run `cargo test` to verify all tests pass
- [x] 4.2 Run `cargo clippy --lib --tests --all-features --all-targets` to ensure no warnings
- [x] 4.3 Run `cargo fmt --all` to ensure code formatting
- [x] 4.4 Run `cargo check` to verify compilation
