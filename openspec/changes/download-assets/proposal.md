## Why

GitHub Discussions often contain images and other assets referenced via `github.com/user-attachments/assets/` URLs. When exporting discussions to Markdown, these remote references remain in the output, making offline viewing impossible and creating dependency on GitHub's continued availability of the asset URLs. Users need a way to download and preserve these assets locally to create truly self-contained discussion archives.

## What Changes

- **Asset detection and extraction**: Parse discussion body, comments, and replies to extract GitHub asset URLs (`github.com/user-attachments/assets/*`)
- **Parallel asset downloading**: Download discovered assets concurrently with configurable parallelism (default: 4 concurrent downloads)
- **Local asset storage**: Save assets to `<discussion-number>-discussion-assets/` directory alongside the Markdown output
- **URL transformation**: Modify Markdown to include local asset paths while preserving original URLs for reference
- **CLI options**:
  - `--no-assets`: Flag to disable asset downloading
  - `--parallel <num>` / `-j <num>`: Configure number of parallel downloads (default: 4)
- **Progress reporting**: Display download progress for comments, replies, and assets
- **Error resilience**: Skip failed asset downloads with warning messages; continue processing

## Capabilities

### New Capabilities

- `asset-download`: Download and locally store GitHub assets (images, files) referenced in discussions
- `progress-reporting`: Display progress indicators for long-running operations (comment fetching, reply fetching, asset downloading)

### Modified Capabilities

None. This is a new feature that does not modify existing requirement-level behaviors. The core discussion fetching and markdown generation capabilities remain unchanged in their requirements.

## Impact

- **New dependencies**: `scraper` crate for HTML parsing; `crossbeam` crate for scoped parallelism (optional, can use std::thread)
- **Modified modules**:
  - `src/cli.rs`: Add `--no-assets` and `--parallel` arguments
  - `src/output.rs`: Extend to detect assets, transform URLs, and write assets to disk
  - New module `src/assets.rs` for asset downloading logic
  - New module `src/progress.rs` for progress reporting (or integrate into existing modules)
- **GraphQL queries**: Extend `DISCUSSION_QUERY`, `COMMENTS_QUERY`, and `REPLIES_QUERY` to include `totalCount` fields for progress reporting
- **File system operations**: Create asset directories, write downloaded files
- **Network operations**: HTTP requests to asset URLs (using existing `reqwest` client)
- **Output behavior**: Markdown files will contain modified `src` attributes in `<img>` tags and image references, with original URLs preserved in `data-original-url` attributes or title text
