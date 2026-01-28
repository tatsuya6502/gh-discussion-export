## ADDED Requirements

### Requirement: Discussion data model
The system SHALL define a `Discussion` struct matching GitHub's GraphQL Discussion type.

#### Scenario: Required fields
- **WHEN** Discussion struct is defined
- **THEN** it includes fields: title, number, url, createdAt, body (author, body, createdAt)
- **AND** all fields use appropriate Rust types (String, DateTime, etc.)

#### Scenario: Comments relationship
- **WHEN** Discussion struct is defined
- **THEN** it includes a comments field containing vector of Comment structs

### Requirement: Comment data model
The system SHALL define a `Comment` struct matching GitHub's GraphQL Comment type.

#### Scenario: Required fields
- **WHEN** Comment struct is defined
- **THEN** it includes fields: id, databaseId, author (login), createdAt, body

#### Scenario: Replies relationship
- **WHEN** Comment struct is defined
- **THEN** it includes a replies field containing vector of Reply structs

#### Scenario: Pagination metadata
- **WHEN** Comment struct is defined
- **THEN** it includes pageInfo with hasNextPage, endCursor

### Requirement: Reply data model
The system SHALL define a `Reply` struct matching GitHub's GraphQL CommentReply type.

#### Scenario: Required fields
- **WHEN** Reply struct is defined
- **THEN** it includes fields: id, databaseId, author (login), createdAt, body

### Requirement: Author data model
The system SHALL define an `Author` struct representing GitHub user or organization.

#### Scenario: Required fields
- **WHEN** Author struct is defined
- **THEN** it includes login field (String)
- **AND** author is optional to handle deleted users

### Requirement: PageInfo data model
The system SHALL define a `PageInfo` struct for GraphQL pagination metadata.

#### Scenario: Required fields
- **WHEN** PageInfo struct is defined
- **THEN** it includes hasNextPage (bool) and endCursor (Option<String>)

### Requirement: Serde deserialization
All data models SHALL derive `Deserialize` trait to support JSON parsing.

#### Scenario: Deserialize from JSON
- **WHEN** JSON response is received from GitHub API
- **THEN** serde can deserialize JSON into model structs

#### Scenario: Handle field renaming
- **WHEN** JSON field names use camelCase
- **THEN** Rust struct fields use snake_case with `#[serde(rename)]` attributes

### Requirement: Response wrapper
The system SHALL define a top-level response struct matching GraphQL response format.

#### Scenario: Response includes data field
- **WHEN** response struct is defined
- **THEN** it includes optional `data` field containing actual discussion data
- **AND** it includes optional `errors` field for GraphQL errors

#### Scenario: Check for errors
- **WHEN** response is received
- **THEN** system checks if `errors` field is present and returns error if so
