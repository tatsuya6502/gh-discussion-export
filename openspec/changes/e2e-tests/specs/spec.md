## ADDED Requirements

### Requirement: Basic end-to-end test
The system SHALL include an integration test that successfully fetches and exports a small GitHub Discussion.

#### Scenario: Successful export of small discussion
- **WHEN** integration test runs with a known small discussion (< 10 comments)
- **THEN** test completes successfully
- **AND** output file is created
- **AND** output file contains discussion title
- **AND** output file contains original post
- **AND** output file contains all comments
- **AND** output file follows correct Markdown format (##/###/#### headings)

### Requirement: Large discussion test
The system SHALL include an integration test that fetches a discussion with many comments (tests pagination).

#### Scenario: Export of large discussion
- **WHEN** integration test runs with a discussion > 100 comments
- **THEN** test completes successfully
- **AND** all comments are included in output
- **AND** comments are ordered chronologically
- **AND** pagination was successful (no data loss)

### Requirement: Deleted user test
The system SHALL include an integration test that verifies handling of deleted users.

#### Scenario: Discussion with deleted author
- **WHEN** integration test runs with a discussion containing deleted users
- **THEN** test completes successfully
- **AND** deleted users are shown as `<deleted>`
- **AND** no panic or error occurs

### Requirement: Output format validation
The system SHALL include tests that validate the exact output format.

#### Scenario: Header format validation
- **WHEN** output is generated
- **THEN** header contains all required fields in correct order
- **AND** header ends with `---` separator

#### Scenario: Heading hierarchy validation
- **WHEN** output is generated
- **THEN** heading levels are correct (## for sections, ### for comments, #### for replies)

#### Scenario: Author format validation
- **WHEN** output is generated
- **THEN** author lines use italic format: `_author: <login> (<timestamp>)_`

### Requirement: Error path tests
The system SHALL include tests that verify error handling.

#### Scenario: Invalid discussion number
- **WHEN** test runs with non-existent discussion number
- **THEN** tool exits with non-zero status
- **AND** error message indicates discussion not found

#### Scenario: Unauthenticated test
- **WHEN** test runs without GitHub CLI authentication
- **THEN** tool exits with non-zero status
- **AND** error message indicates authentication failure

### Requirement: Idempotency test
The system SHALL verify that running the export multiple times produces identical output.

#### Scenario: Multiple runs produce same output
- **WHEN** same discussion is exported twice
- **THEN** both output files are byte-for-byte identical
