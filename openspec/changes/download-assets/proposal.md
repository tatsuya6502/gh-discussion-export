## Why

GitHub Discussions often contain images and other assets referenced via `github.com/user-attachments/assets/` URLs. When exporting discussions to Markdown, these remote references remain in the output, making offline viewing impossible and creating dependency on GitHub's continued availability of the asset URLs. Users need a way to download and preserve these assets locally to create truly self-contained discussion archives.

## What Changes

- **Asset detection and extraction**: Parse discussion body, comments, and replies to extract GitHub asset URLs (`github.com/user-attachments/assets/*`)
- **Parallel asset downloading**: Download discovered assets concurrently with configurable parallelism (default: 4 concurrent downloads)
- **Local asset storage**: Save assets to `<discussion-number>-discussion-assets/` directory alongside the Markdown output. Asset directory is created only when at least one asset is successfully downloaded. If all downloads fail, no asset directory is created.
- **URL transformation**: Modify Markdown to include local asset paths while preserving original URLs for reference. When no assets are successfully downloaded, Markdown is left unchanged and no asset directory is created.
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

**markdown-output**: Output formatting is modified to include local asset paths and preserved original URLs when assets are successfully downloaded. Core discussion fetching behaviors remain unchanged.

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
- **Output behavior**: Markdown files will contain modified `src` attributes in `<img>` tags and image references. For HTML `<img>` tags, original URLs are preserved in `data-original-url` attributes. For Markdown image syntax, original URLs are preserved in HTML comments adjacent to the image.
