## 1. Dependencies and Project Setup

- [ ] 1.1 Add `scraper` crate to Cargo.toml for HTML parsing
- [ ] 1.2 Add `crossbeam` crate to Cargo.toml for scoped parallelism (or implement with std::thread)
- [ ] 1.3 Run `cargo check` to verify dependencies compile correctly
- [ ] 1.4 Run `cargo test` to ensure existing tests still pass

## 2. GraphQL Query Extensions

- [ ] 2.1 Add `totalCount` field to `DISCUSSION_QUERY` (if applicable for discussion metadata)
- [ ] 2.2 Add `totalCount` field to `COMMENTS_QUERY` in comments connection
- [ ] 2.3 Add `totalCount` field to `REPLIES_QUERY` in replies connection
- [ ] 2.4 Update `DiscussionComments`, `CommentReplies`, and related models in `src/models.rs` to include `totalCount: Option<usize>`
- [ ] 2.5 Write unit tests for `totalCount` field parsing from GraphQL responses

## 3. CLI Argument Extensions

- [ ] 3.1 Add `no_assets: bool` field to `CliArgs` struct with `action = ArgAction::SetTrue`
- [ ] 3.2 Add `parallel: usize` field to `CliArgs` struct with `default_value = "4"` and short flag `-j`
- [ ] 3.3 Add helper method `CliArgs::should_download_assets(&self) -> bool` that returns `!self.no_assets`
- [ ] 3.4 Add helper method `CliArgs::asset_dir_name(&self) -> String` that returns `<number>-discussion-assets`
- [ ] 3.5 Write CLI argument parsing tests for new flags (positive and negative cases)
- [ ] 3.6 Run `cargo test` to verify CLI tests pass

## 4. Asset Detection Module

- [ ] 4.1 Create new module `src/assets.rs` with `mod assets;` in `src/lib.rs`
- [ ] 4.2 Implement function to extract UUID from GitHub asset URL (`extract_asset_uuid(url: &str) -> Option<String>`)
- [ ] 4.3 Implement function to detect all GitHub asset URLs in HTML content (`detect_asset_urls(html: &str) -> Vec<String>`)
- [ ] 4.4 Implement function to detect all GitHub asset URLs in Markdown image syntax (`detect_markdown_assets(text: &str) -> Vec<String>`)
- [ ] 4.5 Implement deduplication function to get unique UUIDs (`dedupe_asset_urls(urls: Vec<String>) -> Vec<String>`)
- [ ] 4.6 Write unit tests for UUID extraction from various URL formats
- [ ] 4.7 Write unit tests for asset URL detection in HTML and Markdown
- [ ] 4.8 Write unit tests for deduplication logic

## 5. Asset Download Module

- [ ] 5.1 Implement function to map Content-Type to file extension (`content_type_to_extension(content_type: &str) -> String`)
- [ ] 5.2 Implement function to download single asset to local path with authentication (`download_asset(client: &Client, token: &str, url: &str, path: &Path) -> Result<()>`)
- [ ] 5.3 Implement function to download assets in parallel with configurable parallelism (`download_assets_parallel(client: &Client, token: &str, urls: Vec<String>, dir: &Path, parallel: usize) -> Vec<Result<String>>`)
- [ ] 5.4 Ensure `.bearer_auth(token)` is called on asset download requests for private repository access
- [ ] 5.5 Implement error handling to log warnings and continue on download failures
- [ ] 5.6 Write unit tests for Content-Type to extension mapping
- [ ] 5.7 Write integration tests for successful download with authentication
- [ ] 5.8 Write integration tests for download failure handling (404, 403, timeout)

## 6. Markdown Transformation Module

- [ ] 6.1 Create new module `src/transform.rs` with `mod transform;` in `src/lib.rs` (or extend `src/output.rs`)
- [ ] 6.2 Implement function to transform HTML `<img>` tag with local path and data-original-url attribute
- [ ] 6.3 Implement function to transform Markdown image syntax with local path and title containing original URL
- [ ] 6.4 Implement function to transform discussion body with asset URL replacements
- [ ] 6.5 Implement function to transform comment body with asset URL replacements
- [ ] 6.6 Implement function to transform reply body with asset URL replacements
- [ ] 6.7 Write unit tests for HTML img tag transformation (preserve all attributes)
- [ ] 6.8 Write unit tests for Markdown image syntax transformation
- [ ] 6.9 Write unit tests that original URL is preserved in output

## 7. Progress Reporting Module

- [ ] 7.1 Create new module `src/progress.rs` with `mod progress;` in `src/lib.rs`
- [ ] 7.2 Implement `ProgressReporter` struct with methods for starting, updating, and completing progress
- [ ] 7.3 Implement terminal detection to check if stdout is a TTY (`is_terminal()`)
- [ ] 7.4 Implement inline progress update with carriage return for TTY output
- [ ] 7.5 Implement line-by-line progress for non-TTY output (piped/file)
- [ ] 7.6 Implement function to format progress message with count and percentage (`format_progress(current: usize, total: usize) -> String`)
- [ ] 7.7 Write unit tests for progress formatting
- [ ] 7.8 Write unit tests for terminal detection logic

## 8. Integration with Fetch Module

- [ ] 8.1 Modify `src/fetch.rs` to report progress when fetching discussion metadata
- [ ] 8.2 Modify `src/fetch.rs` to report comment pagination progress using `totalCount`
- [ ] 8.3 Modify `src/fetch.rs` to report reply fetching progress using `totalCount`
- [ ] 8.4 Pass `ProgressReporter` through fetch call chain (or use shared progress module)
- [ ] 8.5 Write integration tests for progress reporting during pagination
- [ ] 8.6 Run `cargo test` to verify fetch module tests still pass

## 9. Integration with Output Module

- [ ] 9.1 Extend `src/output.rs::generate_original_post()` to accept asset mapping and transform body
- [ ] 9.2 Extend `src/output.rs::generate_comments()` to accept asset mapping and transform comments/replies
- [ ] 9.3 Modify `src/output.rs::format_discussion()` to:
  - Call asset detection if `--no-assets` flag is not set
  - Download detected assets to asset directory
  - Build asset URL mapping (original URL -> local path)
  - Transform all content with asset URL mapping
- [ ] 9.4 Handle case when asset directory creation fails (error with clear message)
- [ ] 9.5 Handle case when no assets are detected (skip directory creation)
- [ ] 9.6 Write integration tests for discussion export with assets
- [ ] 9.7 Write integration tests for discussion export with `--no-assets` flag
- [ ] 9.8 Run `cargo test` to verify output module tests still pass

## 10. Main Entry Point Integration

- [ ] 10.1 Modify `src/main.rs` to create asset directory if downloading assets
- [ ] 10.2 Modify `src/main.rs` to pass GitHub token to asset download functions (token already retrieved from `gh auth token`)
- [ ] 10.3 Modify `src/main.rs` to pass CLI flags to output formatting function
- [ ] 10.4 Ensure proper error propagation for asset download failures
- [ ] 10.5 Display final summary with asset count and location
- [ ] 10.6 Display warning count if any assets failed to download
- [ ] 10.7 Test end-to-end with real discussion containing images (public repository)
- [ ] 10.8 Test end-to-end with private repository discussion containing images (verify authentication works)
- [ ] 10.9 Test end-to-end with discussion containing no images
- [ ] 10.10 Test end-to-end with `--no-assets` flag
- [ ] 10.11 Test end-to-end with custom parallelism (`-j 1`, `-j 8`)

## 11. Error Handling and Edge Cases

- [ ] 11.1 Handle asset directory already exists (continue without error)
- [ ] 11.2 Handle asset with same UUID already downloaded (skip re-download)
- [ ] 11.3 Handle network timeout during download with warning message
- [ ] 11.4 Handle 404 responses with warning message (asset not found)
- [ ] 11.5 Handle 403 responses with warning message (authentication failed or private repo access denied)
- [ ] 11.6 Handle 401 responses with clear error message (invalid token, prompt user to run `gh auth login`)
- [ ] 11.7 Handle permission errors when writing asset files
- [ ] 11.8 Handle disk space errors gracefully
- [ ] 11.9 Write integration tests for each error scenario (404, 403, 401, timeout, permission denied)

## 12. Documentation

- [ ] 12.1 Update README.md with new `--no-assets` and `-j` flag documentation
- [ ] 12.2 Add example usage in README showing asset download behavior
- [ ] 12.3 Update CLI help text (clap derive should handle most automatically)
- [ ] 12.4 Document asset directory naming convention in README
- [ ] 12.5 Document URL transformation behavior in README
- [ ] 12.6 Document that assets from private repositories require authentication (same as discussion access)
- [ ] 12.7 Document error messages for authentication failures (401, 403)
- [ ] 12.8 Add CHANGELOG.md entry for new feature

## 13. Final Verification

- [ ] 13.1 Run `cargo check` to ensure no compilation errors
- [ ] 13.2 Run `cargo clippy --lib --tests --all-features --all-targets` to ensure no warnings
- [ ] 13.3 Run `cargo fmt --all` to ensure code formatting
- [ ] 13.4 Run `cargo test` to ensure all tests pass
- [ ] 13.5 Manually test with discussion containing multiple images
- [ ] 13.6 Manually test with discussion containing no images
- [ ] 13.7 Manually test with `--no-assets` flag
- [ ] 13.8 Manually test with various parallelism settings
- [ ] 13.9 Verify generated Markdown renders correctly with local asset paths
- [ ] 13.10 Verify offline viewing works (disconnect network, open Markdown in viewer)
