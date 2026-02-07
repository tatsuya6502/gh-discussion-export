## ADDED Requirements

### Requirement: Extract GitHub asset URLs from discussion content

The system SHALL extract all GitHub asset URLs matching the pattern `github.com/user-attachments/assets/*` from discussion body, comments, and replies.

#### Scenario: Extract single image from HTML img tag

- **WHEN** discussion body contains `<img src="https://github.com/user-attachments/assets/6c72b402-4a5c-45cc-9b0a-50717f8a09a7" alt="Diagram" />`
- **THEN** system extracts URL `https://github.com/user-attachments/assets/6c72b402-4a5c-45cc-9b0a-50717f8a09a7`

#### Scenario: Extract multiple images from comment

- **WHEN** comment contains three `<img>` tags with GitHub asset URLs
- **THEN** system extracts all three URLs

#### Scenario: Extract image from Markdown syntax

- **WHEN** reply contains `![Diagram](https://github.com/user-attachments/assets/abc123)`
- **THEN** system extracts URL `https://github.com/user-attachments/assets/abc123`

#### Scenario: No assets in discussion

- **WHEN** discussion contains no GitHub asset URLs
- **THEN** system extracts zero URLs
- **AND** system does not create asset directory

#### Scenario: Ignore non-GitHub asset URLs

- **WHEN** discussion contains `<img src="https://example.com/image.png" />`
- **THEN** system does not extract the URL
- **AND** system does not attempt to download non-GitHub assets

### Requirement: Download assets with Content-Type-based extension

The system SHALL download each extracted asset and determine the file extension from the HTTP Content-Type header.

#### Scenario: Download PNG asset

- **WHEN** asset URL returns Content-Type `image/png`
- **THEN** system downloads file with `.png` extension
- **AND** filename is UUID from URL (e.g., `6c72b402-4a5c-45cc-9b0a-50717f8a09a7.png`)

#### Scenario: Download JPEG asset

- **WHEN** asset URL returns Content-Type `image/jpeg`
- **THEN** system downloads file with `.jpg` extension

#### Scenario: Unknown Content-Type defaults to .bin

- **WHEN** asset URL returns Content-Type `application/octet-stream` or unknown type
- **THEN** system downloads file with `.bin` extension
- **AND** file data is preserved completely

#### Scenario: Handle SVG assets

- **WHEN** asset URL returns Content-Type `image/svg+xml`
- **THEN** system downloads file with `.svg` extension

### Requirement: Save assets to discussion-specific directory

The system SHALL save downloaded assets to a directory named `<discussion-number>-discussion-assets/` in the same location as the output Markdown file.

#### Scenario: Default asset directory location

- **WHEN** discussion #1041 is exported to `1041-discussion.md` in current directory
- **THEN** assets are saved to `./1041-discussion-assets/` directory
- **AND** directory is created if it does not exist

#### Scenario: Custom output path

- **WHEN** discussion #1041 is exported to `./output/1041.md` with `--output ./output/1041.md`
- **THEN** assets are saved to `./output/1041-discussion-assets/` directory
- **AND** asset directory is created alongside Markdown file

#### Scenario: Multiple discussions in same directory

- **WHEN** discussion #1041 and #1042 are exported to the same directory
- **THEN** assets are saved to separate directories: `1041-discussion-assets/` and `1042-discussion-assets/`
- **AND** no conflict occurs between asset directories

### Requirement: Transform Markdown to reference local assets

The system SHALL transform the exported Markdown to reference local asset paths while preserving original URLs for reference.

#### Scenario: Transform HTML img tag with local path

- **GIVEN** discussion contains `<img src="https://github.com/user-attachments/assets/6c72b402-4a5c-45cc-9b0a-50717f8a09a7" alt="ER図" width="1192" height="861" />`
- **AND** asset is downloaded to `1041-discussion-assets/6c72b402-4a5c-45cc-9b0a-50717f8a09a7.png`
- **WHEN** Markdown output is written
- **THEN** img tag is transformed to `<img src="1041-discussion-assets/6c72b402-4a5c-45cc-9b0a-50717f8a09a7.png" alt="ER図" width="1192" height="861" data-original-url="https://github.com/user-attachments/assets/6c72b402-4a5c-45cc-9b0a-50717f8a09a7" />`

#### Scenario: Transform markdown image syntax with local path

- **GIVEN** discussion contains `![ER図](https://github.com/user-attachments/assets/6c72b402-4a5c-45cc-9b0a-50717f8a09a7)`
- **AND** asset is downloaded to `1041-discussion-assets/6c72b402-4a5c-45cc-9b0a-50717f8a09a7.png`
- **WHEN** Markdown output is written
- **THEN** image reference is transformed to `![ER図](1041-discussion-assets/6c72b402-4a5c-45cc-9b0a-50717f8a09a7.png "https://github.com/user-attachments/assets/6c72b402-4a5c-45cc-9b0a-50717f8a09a7")`

#### Scenario: Preserve all other img attributes

- **GIVEN** img tag has attributes `width`, `height`, `alt`, and `loading`
- **WHEN** Markdown is transformed
- **THEN** all original attributes are preserved
- **AND** only `src` is changed to local path
- **AND** `data-original-url` attribute is added

### Requirement: Deduplicate assets by UUID

The system SHALL download each unique asset UUID only once per discussion, even if the same asset appears multiple times.

#### Scenario: Same asset appears in body and comment

- **GIVEN** same UUID `abc123` appears in discussion body and a comment
- **WHEN** assets are downloaded
- **THEN** asset is downloaded only once
- **AND** both Markdown references point to same local file

#### Scenario: Same asset appears multiple times in same comment

- **GIVEN** same UUID `def456` appears three times in one comment
- **WHEN** assets are downloaded
- **THEN** asset is downloaded only once
- **AND** all three references point to same local file

### Requirement: Support parallel asset downloads

The system SHALL download assets concurrently using a configurable number of parallel downloads.

#### Scenario: Default parallelism of 4

- **WHEN** user runs `gh-discussion-export 1041` without parallelism flag
- **AND** 10 assets are detected
- **THEN** system downloads assets with up to 4 concurrent connections
- **AND** all 10 assets are downloaded successfully

#### Scenario: Custom parallelism with -j flag

- **WHEN** user runs `gh-discussion-export 1041 -j 8`
- **AND** 20 assets are detected
- **THEN** system downloads assets with up to 8 concurrent connections

#### Scenario: Single thread with -j 1

- **WHEN** user runs `gh-discussion-export 1041 -j 1`
- **THEN** system downloads assets sequentially (one at a time)

### Requirement: Handle download errors gracefully

The system SHALL continue processing when asset download fails, logging a warning message to stderr.

#### Scenario: Network timeout on one asset

- **GIVEN** 5 assets to download
- **AND** asset #3 times out during download
- **WHEN** download timeout occurs
- **THEN** system prints warning message to stderr
- **AND** system continues downloading assets #4 and #5
- **AND** Markdown file is still created successfully
- **AND** failed asset's URL is not transformed to local path

#### Scenario: 404 Not Found for asset

- **GIVEN** asset URL returns 404 status
- **WHEN** download fails with 404
- **THEN** system prints warning message: "Failed to download asset: HTTP 404"
- **AND** system continues processing
- **AND** original URL remains in Markdown (no transformation)

#### Scenario: All assets fail to download

- **GIVEN** all 3 assets fail to download (network errors)
- **WHEN** all downloads fail
- **THEN** system prints 3 warning messages to stderr
- **AND** Markdown file is still created
- **AND** asset directory may or may not be created (implementation choice)
- **AND** all asset URLs remain unchanged in Markdown

### Requirement: Support --no-assets flag

The system SHALL provide a `--no-assets` CLI flag to disable asset downloading.

#### Scenario: Disable asset downloading

- **WHEN** user runs `gh-discussion-export 1041 --no-assets`
- **THEN** system does not download any assets
- **AND** system does not create asset directory
- **AND** Markdown output contains original GitHub URLs (not transformed)
- **AND** no progress is displayed for asset downloading

#### Scenario: --no-assets with -j flag

- **WHEN** user runs `gh-discussion-export 1041 --no-assets -j 8`
- **THEN** `-j` flag is ignored (no downloads occur)
- **AND** behavior is identical to `--no-assets` alone

#### Scenario: Default behavior downloads assets

- **WHEN** user runs `gh-discussion-export 1041` without any flags
- **THEN** system downloads all detected assets
- **AND** assets are saved to local directory
- **AND** Markdown is transformed to reference local paths

### Requirement: Authenticate asset downloads with GitHub token

The system SHALL use the same GitHub authentication token (retrieved via `gh auth token`) for asset downloads.

#### Scenario: Download private asset

- **GIVEN** asset is in a private repository
- **AND** user has authenticated with `gh auth login`
- **WHEN** system downloads asset
- **THEN** download includes Authorization header with token from `gh auth token`
- **AND** asset is successfully downloaded

#### Scenario: Download fails with invalid token

- **GIVEN** user is not authenticated or token is expired
- **WHEN** system attempts to download asset
- **THEN** download fails with authentication error
- **AND** system logs warning message
- **AND** system continues processing (graceful degradation)
