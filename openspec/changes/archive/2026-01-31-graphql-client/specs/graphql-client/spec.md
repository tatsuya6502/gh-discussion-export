## ADDED Requirements

### Requirement: Execute GraphQL query
The system SHALL provide a function to execute GraphQL queries against GitHub's API endpoint at `https://api.github.com/graphql`.

#### Scenario: Successful query execution
- **WHEN** valid GraphQL query and authentication token are provided
- **THEN** system sends POST request with `Authorization: Bearer <token>` header and returns JSON response

#### Scenario: Authentication failure
- **WHEN** query is executed with invalid token
- **THEN** system returns error indicating 401 status

#### Scenario: Rate limit exceeded
- **WHEN** query execution returns 403 or 429 status
- **THEN** system returns error indicating rate limit exceeded

### Requirement: GraphQL query construction
The system SHALL define a GraphQL query that fetches discussion data including title, author, body, comments, and replies.

#### Scenario: Query includes required fields
- **WHEN** GraphQL query is constructed
- **THEN** query includes discussion title, number, author login, body, createdAt
- **AND** query includes comments with id, author login, body, createdAt
- **AND** query includes replies to comments with id, author login, body, createdAt

#### Scenario: Query uses proper pagination cursors
- **WHEN** GraphQL query is constructed
- **THEN** query includes cursor fields for comments and replies
- **AND** query includes hasNextPage and hasPreviousPage fields

### Requirement: Parse GraphQL response
The system SHALL deserialize GraphQL JSON responses into Rust structs using serde.

#### Scenario: Successful response parsing
- **WHEN** valid GraphQL JSON response is received
- **THEN** system deserializes response into Discussion struct
- **AND** all fields are populated from JSON data

#### Scenario: GraphQL errors in response
- **WHEN** response contains GraphQL errors (HTTP 200 with errors array)
- **THEN** system returns error indicating GraphQL validation or syntax error

### Requirement: Handle nullable fields
The system SHALL properly handle nullable fields in GraphQL responses (e.g., deleted users, null bodies).

#### Scenario: Author is null
- **WHEN** response contains null author field
- **THEN** system deserializes as `<deleted>` placeholder

#### Scenario: Optional fields missing
- **WHEN** optional fields are not present in response
- **THEN** system treats them as None or default value
