## ADDED Requirements

### Requirement: Display discussion fetching progress

The system SHALL display progress information when fetching the discussion metadata.

#### Scenario: Fetch single discussion

- **WHEN** user runs `gh-discussion-export 1041`
- **THEN** system displays "Fetching discussion #1041..."
- **AND** when complete, displays confirmation with discussion title

### Requirement: Display comment fetching progress with percentage

The system SHALL display progress indicators for comment pagination, showing current count and total percentage.

#### Scenario: Fetch 15 comments across 2 pages

- **GIVEN** discussion has 15 comments across 2 API pages (10 + 5)
- **WHEN** system fetches comments
- **THEN** system displays "Fetching comments... 0/15 (0%)"
- **AND** after first page: "Fetching comments... 10/15 (67%)"
- **AND** after second page: "Fetching comments... 15/15 (100%)"
- **AND** final message confirms total count

#### Scenario: Discussion with no comments

- **GIVEN** discussion has zero comments
- **WHEN** system fetches comments
- **THEN** system displays "Fetching comments... 0/0 (100%)"
- **AND** no progress updates are displayed (single complete message)

#### Scenario: Inline update for comment progress

- **GIVEN** discussion has 25 comments across 3 pages
- **WHEN** system fetches comments
- **THEN** progress line updates in-place (using carriage return)
- **AND** terminal shows single line that changes from "0/25 (0%)" to "25/25 (100%)"
- **AND** final state shows "✓ Loaded 25 comments"

### Requirement: Display reply fetching progress

The system SHALL display progress indicators for fetching replies to comments.

#### Scenario: Fetch replies for comments

- **GIVEN** discussion has 10 comments with varying reply counts (0, 2, 5, 1, 0, 3, 0, 4, 2, 0)
- **AND** total replies: 17
- **WHEN** system fetches replies
- **THEN** system displays "Fetching replies... 0/17 (0%)"
- **AND** progress updates as replies are fetched
- **AND** final message: "Fetching replies... 17/17 (100%)"
- **AND** summary: "✓ Loaded 17 replies across 10 comments"

#### Scenario: Comments with no replies

- **GIVEN** all 5 comments have zero replies
- **WHEN** system fetches replies
- **THEN** system displays "Fetching replies... 0/0 (100%)"
- **AND** displays "✓ No replies found"

#### Scenario: Skip comment progress when totalCount unavailable

- **GIVEN** GraphQL API does not return `totalCount` for replies (API limitation)
- **WHEN** system fetches replies
- **THEN** system displays "Fetching replies..." without percentage
- **AND** displays "✓ Loaded replies for 10 comments" when complete

### Requirement: Display asset detection progress

The system SHALL display progress when scanning for and downloading assets.

#### Scenario: Detect and display asset count

- **GIVEN** discussion contains 10 GitHub asset URLs
- **WHEN** system scans discussion content
- **THEN** system displays "Scanning for assets..."
- **AND** displays "✓ Found 10 assets"

#### Scenario: No assets found

- **GIVEN** discussion contains no GitHub asset URLs
- **WHEN** system scans discussion content
- **THEN** system displays "Scanning for assets..."
- **AND** displays "✓ No assets found"
- **AND** no download progress is shown

### Requirement: Display asset download progress with percentage

The system SHALL display real-time progress for asset downloads, showing completed count and percentage.

#### Scenario: Download 10 assets with default parallelism

- **GIVEN** 10 unique assets detected
- **AND** parallelism is 4 (default)
- **WHEN** system downloads assets
- **THEN** system displays "Downloading assets... 0/10 (0%)"
- **AND** progress updates as downloads complete: "Downloading assets... 4/10 (40%)", "Downloading assets... 7/10 (70%)"
- **AND** final state: "Downloading assets... 10/10 (100%)"
- **AND** displays "✓ Downloaded 10 assets to: 1041-discussion-assets/"

#### Scenario: Download assets with some failures

- **GIVEN** 10 assets detected
- **AND** 2 assets fail to download
- **WHEN** system completes download attempts
- **THEN** final message displays "✓ Downloaded 8 assets to: 1041-discussion-assets/"
- **AND** displays "⚠ Skipped 2 assets (download failed)"

#### Scenario: Download single asset

- **GIVEN** 1 asset detected
- **WHEN** system downloads asset
- **THEN** system displays "Downloading assets... 0/1 (0%)"
- **AND** completes with "Downloading assets... 1/1 (100%)"
- **AND** displays "✓ Downloaded 1 asset to: 1041-discussion-assets/"

#### Scenario: Inline update for asset download progress

- **GIVEN** 15 assets being downloaded with parallelism 4
- **WHEN** downloads are in progress
- **THEN** progress line updates in-place: "Downloading assets... 6/15 (40%)"
- **AND** updates continue until "Downloading assets... 15/15 (100%)"
- **AND** then moves to next line for final summary

### Requirement: Display final export summary

The system SHALL display a summary message after successful export.

#### Scenario: Successful export with assets

- **GIVEN** discussion #1041 exported successfully
- **AND** 10 assets downloaded
- **WHEN** export completes
- **THEN** system displays:
  ```text
  ✓ Exported to: 1041-discussion.md
  ✓ Downloaded 10 assets to: 1041-discussion-assets/
  ```

#### Scenario: Successful export without assets

- **GIVEN** discussion #1041 exported successfully
- **AND** `--no-assets` flag was used
- **WHEN** export completes
- **THEN** system displays "✓ Exported to: 1041-discussion.md"
- **AND** no asset download message is shown

#### Scenario: Successful export with failed assets

- **GIVEN** discussion #1041 exported successfully
- **AND** 8 assets downloaded, 2 failed
- **WHEN** export completes
- **THEN** system displays:
  ```text
  ✓ Exported to: 1041-discussion.md
  ✓ Downloaded 8 assets to: 1041-discussion-assets/
  ⚠ Skipped 2 assets (download failed)
  ```

### Requirement: Fetch totalCount for progress calculation

The system SHALL request `totalCount` fields in GraphQL queries to enable accurate progress reporting.

#### Scenario: Include totalCount in comments query

- **WHEN** system constructs COMMENTS_QUERY
- **THEN** query includes `totalCount` field at comments level
- **AND** response includes total number of comments

#### Scenario: Include totalCount in replies query

- **WHEN** system constructs REPLIES_QUERY or fetches replies
- **THEN** query includes `totalCount` field at replies level (if API supports)
- **AND** response includes total number of replies for each comment

#### Scenario: Handle missing totalCount gracefully

- **GIVEN** GraphQL response does not include `totalCount` (API limitation or error)
- **WHEN** totalCount is unavailable
- **THEN** system displays progress without percentage
- **AND** system shows count of items fetched so far
- **AND** system displays final count when complete

### Requirement: Suppress progress output for non-terminal usage

The system SHALL detect when output is not a terminal (e.g., piped to file) and suppress inline progress updates; emit line-by-line progress messages instead.

#### Scenario: Piped output to file

- **GIVEN** user runs `gh-discussion-export 1041 > output.log 2>&1`
- **WHEN** stdout is not a terminal
- **THEN** system prints each progress message on a new line (no in-place carriage-return updates)
- **AND** final summary messages are printed normally

#### Scenario: Terminal output

- **GIVEN** user runs `gh-discussion-export 1041` in interactive terminal
- **WHEN** stdout is a terminal
- **THEN** system displays inline progress updates with carriage returns
- **AND** final summary is printed on new line
