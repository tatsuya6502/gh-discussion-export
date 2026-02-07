## Context

Currently, `gh-discussion-export` fetches discussion content via GitHub GraphQL API and outputs a single Markdown file. The tool preserves content verbatim (with only heading escape for document structure). However, discussions often reference assets hosted at `github.com/user-attachments/assets/` URLs, which remain as remote references in the exported Markdown.

**Current architecture:**
- `src/fetch.rs`: Handles GraphQL pagination for comments and replies
- `src/output.rs`: Formats and writes Markdown output
- `src/client.rs`: Provides `reqwest::blocking::Client` for HTTP requests
- `src/graphql.rs`: Contains GraphQL query definitions
- `src/models.rs`: Serde models for API responses

**Constraints:**
- Must preserve lossless fidelity principle (only heading escape is currently permitted)
- Must use GitHub CLI for authentication (no PAT management)
- Must use blocking `reqwest` client (no `tokio`)

## Goals / Non-Goals

**Goals:**
- Download GitHub assets referenced in discussions to local directory
- Enable offline viewing of exported discussions with images
- Maintain backward compatibility (default behavior includes asset download)
- Provide progress feedback for long-running operations
- Handle network errors gracefully (skip failed assets, continue processing)

**Non-Goals:**
- Detecting and downloading non-GitHub asset URLs (e.g., external image hosts)
- Modifying core discussion fetching logic
- Changing the existing markdown structure (header levels, author format, etc.)
- Implementing sophisticated retry logic (basic single-attempt download only)
- Asset deduplication across multiple discussions (each discussion gets its own asset directory)

## Decisions

### 1. URL Transformation Strategy

**Decision**: Transform `<img>` tags and markdown image references to include local paths while preserving original URLs in `data-original-url` attribute.

**Alternatives considered:**
- Replace URLs entirely: Rejected - loses provenance information, violates lossless fidelity spirit
- Add local path in separate comment: Rejected - harder to parse, less usable
- Use HTML title attribute: Rejected - conflicts with potential existing title usage

**Rationale**: Using `data-original-url` attribute:
- Preserves original URL for reference/provenance
- Maintains compatibility with Markdown viewers that understand local paths
- Allows easy extraction of original URL if needed
- Minimally invasive to existing structure

**Example transformation:**
```html
<!-- Before -->
<img src="https://github.com/user-attachments/assets/abc123" alt="Diagram" />

<!-- After -->
<img src="1041-discussion-assets/abc123.png" alt="Diagram" data-original-url="https://github.com/user-attachments/assets/abc123" />
```

### 2. File Naming Convention

**Decision**: Use full UUID from asset URL as filename, with extension derived from Content-Type header.

**Format:** `<uuid>.<ext>` (e.g., `6c72b402-4a5c-45cc-9b0a-50717f8a09a7.png`)

**Alternatives considered:**
- Sequential numbering: Rejected - non-deterministic across runs, no semantic meaning
- Alt text + hash: Rejected - requires handling internationalization, potential filename conflicts
- Truncated UUID: Rejected - loses uniqueness guarantee

**Rationale**: Full UUID provides:
- Guaranteed uniqueness (GitHub's asset URLs are globally unique)
- Easy mapping between URL and filename
- No encoding/escaping issues
- Simple deduplication detection (same UUID = same file)

### 3. Directory Structure

**Decision**: Create `<discussion-number>-discussion-assets/` directory alongside Markdown file.

**Alternatives considered:**
- Single shared `assets/` directory: Rejected - potential filename collisions across discussions, unclear ownership
- Nested `ghd-assets/<number>/`: Rejected - unnecessary complexity for single-discussion tool

**Rationale**: Per-discussion asset directory:
- Clear ownership (one directory per discussion)
- No cross-discussion conflicts
- Easy to delete/move entire discussion archive
- Scales to future multi-discussion support

**Example structure:**
```
1041-discussion.md
1041-discussion-assets/
  └── 6c72b402-4a5c-45cc-9b0a-50717f8a09a7.png
```

### 4. HTML Parsing Library

**Decision**: Use `scraper` crate for HTML parsing.

**Alternatives considered:**
- `select`: Rejected - less maintained, fewer features
- `html5ever`: Rejected - overkill, larger dependency, steeper learning curve
- Regex: Rejected - brittle, error-prone for HTML parsing

**Rationale**: `scraper` provides:
- CSS selector-based API (familiar to web developers)
- Good maintenance status and documentation
- Lightweight dependency
- Sufficient for our simple `<img>` tag extraction needs

### 5. Progress Display Strategy

**Decision**: Use simple print statements with inline updates using carriage return (`\r`).

**Alternatives considered:**
- `indicatif` crate: Rejected - adds dependency, blocking client complicates progress bar updates
- No progress display: Rejected - poor UX for long-running operations
- Log file only: Rejected - doesn't provide real-time feedback

**Rationale**: Simple print-based progress:
- Zero additional dependencies
- Works with blocking operations
- Sufficient feedback for CLI tool
- Easy to implement correctly with `std::io::stdout().flush()`

**Implementation:**
```rust
print!("\rDownloading assets... {}/{} ({}%)", current, total, percent);
stdout().flush().unwrap();
```

### 6. Parallel Download Implementation

**Decision**: Use thread pool with `crossbeam::scope` for scoped parallelism.

**Alternatives considered:**
- `rayon`: Rejected - adds dependency, designed for data parallelism not I/O
- Async with `tokio`: Rejected - conflicts with blocking client requirement
- Sequential downloads: Rejected - too slow for multiple assets

**Rationale**: `crossbeam::scope` provides:
- Scoped threads (can borrow from stack)
- Works with blocking HTTP client
- No runtime/executor complexity
- Lightweight alternative to async

**Fallback**: If `crossbeam` is deemed too heavy, use simple `std::thread::spawn` with `Arc<Mutex<>>` for shared state.

### 7. Error Handling for Asset Downloads

**Decision**: Log warning to stderr and continue processing on download failure.

**Alternatives considered:**
- Fail entire export: Rejected - one bad asset shouldn't prevent discussion export
- Silent skip: Rejected - users should know what was skipped
- Retry logic: Rejected (out of scope) - adds complexity, may not help with 404s

**Rationale**: Continue-on-error:
- Maximizes useful output (most assets still downloaded)
- Clear feedback on what failed
- Aligns with Unix philosophy (be liberal in what you accept)
- User can retry specific assets manually if needed

### 8. CLI Flag Naming (--no-assets)

**Decision**: Use `--no-assets` flag with internal negation logic.

**Implementation**:
```rust
#[arg(long, default_value = "true")]
pub assets: bool,

#[arg(long, action = ArgAction::SetTrue)]
pub no_assets: bool,
```

Usage: `if !args.no_assets { download_assets(); }`

**Alternatives considered:**
- `--download-assets` flag: Rejected - awkward default "on" behavior
- `--assets` / `--no-assets` pair: Rejected - conflicts with clap's bool handling

**Rationale**: `--no-assets` pattern:
- Natural CLI UX (opt-out of default behavior)
- Common pattern in CLI tools (e.g., `--no-verify` in git)
- Clear intent

### 9. Extension Detection Strategy

**Decision**: Use Content-Type header from HTTP response, with fallback to `.bin` for unknown types.

**Mapping:**
```rust
match content_type {
    "image/png" => "png",
    "image/jpeg" => "jpg",
    "image/gif" => "gif",
    "image/svg+xml" => "svg",
    "image/webp" => "webp",
    _ => "bin",
}
```

**Alternatives considered:**
- Use URL extension: Rejected - GitHub asset URLs don't have extensions
- Magic bytes detection: Rejected - requires reading entire file first
- Assume all PNG: Rejected - incorrect for JPEGs, GIFs, etc.

**Rationale**: Content-Type header:
- Provided by GitHub's CDN
- Reliable for GitHub-hosted assets
- Minimal performance overhead
- Fallback to `.bin` prevents data loss

## Risks / Trade-offs

### Risk 1: Concurrent GraphQL Client Usage

**Risk**: The `reqwest::blocking::Client` is not designed for concurrent use across threads.
**Mitigation**: Use `Arc<Client>` for thread-safe sharing, or create one client per thread. The `reqwest` documentation indicates `Client` is safe to share via `Arc` for blocking requests.

### Risk 2: File System Race Conditions

**Risk**: Multiple threads downloading assets with same UUID (shouldn't happen, but defensive).
**Mitigation**: Use `HashSet` to track downloaded UUIDs before spawning download tasks. Each UUID downloaded exactly once.

### Risk 3: Content-Type Mismatch

**Risk**: GitHub returns incorrect Content-Type for some assets.
**Mitigation**: Fallback to `.bin` preserves data even if extension is wrong. Users can manually rename if needed. Trade-off: some inconvenience vs. data corruption risk.

### Risk 4: Network Timeout/Slow Downloads

**Risk**: Asset downloads hang, causing poor UX.
**Mitigation**: Set reasonable timeout on HTTP client (existing client should have timeout). Consider per-download timeout if not present.

### Risk 5: Large Asset Downloads

**Risk**: Discussion contains many large assets, consuming significant disk space/time.
**Mitigation**: `--no-assets` flag allows opting out. Progress indicators show what's being downloaded. User can interrupt with Ctrl+C.

### Risk 6: Modified Markdown Breaks Tools

**Risk**: Some Markdown parsers don't handle `data-original-url` attribute or relative paths correctly.
**Mitigation**: Use standard HTML attributes (`data-*` is valid HTML5). Relative paths are widely supported. Original URL preserved for manual fixes.

### Risk 7: GraphQL totalCount Accuracy

**Risk**: `totalCount` may not be accurate during pagination (race condition with edits).
**Mitigation**: Accept approximate progress. Progress is informational, not critical. Use actual fetched count for final display.

### Risk 8: Authentication for Private Repository Assets

**Risk**: Assets in private repositories require authentication. Without proper auth, downloads fail with 404/403 errors.
**Mitigation**: Reuse the same GitHub token (from `gh auth token`) for asset downloads. The token is already available in the main function where `ReqwestClient` is created. Pass this token to asset download functions.

**Implementation approach:**
```rust
// Option A: Pass token explicitly (simplest)
pub fn download_assets(
    token: &str,
    urls: Vec<String>,
    dir: &Path,
    parallel: usize
) -> Result<Vec<String>> {
    let client = reqwest::blocking::Client::new();
    for url in urls {
        let response = client
            .get(&url)
            .bearer_auth(token)  // Use same token as GraphQL API
            .send()?;
        // ... save to file
    }
}

// Option B: Create authenticated client wrapper
pub struct AuthenticatedClient {
    client: reqwest::blocking::Client,
    token: String,
}

impl AuthenticatedClient {
    pub fn download_asset(&self, url: &str) -> Result<Vec<u8>> {
        Ok(self.client
            .get(url)
            .bearer_auth(&self.token)
            .send()?
            .bytes()?
            .to_vec())
    }
}
```

**Rationale**: Using bearer authentication:
- Consistent with existing GraphQL client implementation
- GitHub CDN accepts bearer tokens for private assets
- Token already retrieved via `gh auth token`
- No additional authentication flow needed

## Migration Plan

No migration required. This is a new feature with backward-compatible defaults.

**Deployment:**
1. Add new dependencies (`scraper`, optionally `crossbeam`)
2. Implement new modules (`src/assets.rs`, `src/progress.rs`)
3. Extend existing modules (`cli.rs`, `output.rs`, `graphql.rs`)
4. Add integration tests
5. Update documentation

**Rollback strategy:**
- `--no-assets` flag provides immediate workaround if bugs encountered
- Can revert to previous version (assets directory is purely additive)

## Open Questions

1. **Should we add a `--assets-dir` flag for custom asset directory naming?**
   - Current design: Always `<number>-discussion-assets/`
   - Consider if users want custom naming
   - Deferred to future enhancement if requested

2. **Should we add `--verbose` flag for detailed asset download logging?**
   - Current design: Simple progress, warnings on stderr
   - Verbose mode could show successful downloads, skipped items, etc.
   - Deferred to future enhancement if requested

3. **Should we validate asset checksums after download?**
   - Current design: No validation
   - Could add SHA-256 verification for critical use cases
   - Deferred to future enhancement if requested

4. **Should we add a `--dry-run` flag to preview what would be downloaded?**
   - Current design: No preview mode
   - Could list assets without downloading
   - Deferred to future enhancement if requested
