## ADDED Requirements

### Requirement: Fetch complete discussion
The system SHALL provide a function to fetch a complete discussion by number with all comments and replies.

#### Scenario: Successful fetch
- **WHEN** valid owner, repo, and discussion number are provided
- **THEN** system fetches discussion metadata
- **AND** system fetches all comments using pagination
- **AND** system fetches all replies to each comment using pagination
- **AND** system returns complete Discussion object with all data

#### Scenario: Discussion not found
- **WHEN** discussion number doesn't exist
- **THEN** system returns error indicating discussion not found

### Requirement: Paginate comments
The system SHALL paginate through all comment pages using cursor-based pagination until completion.

#### Scenario: Multiple pages of comments
- **WHEN** discussion has more comments than fit in one page
- **THEN** system fetches first page with `after: null`
- **AND** system checks `pageInfo.hasNextPage`
- **AND** while `hasNextPage` is true, fetches next page with `after: <endCursor>`
- **AND** continues until `hasNextPage` is false

#### Scenario: Single page of comments
- **WHEN** all comments fit on first page
- **THEN** system fetches single page
- **AND** does not make additional requests

### Requirement: Paginate replies
The system SHALL paginate through all reply pages for each comment using cursor-based pagination.

#### Scenario: Multiple pages of replies
- **WHEN** a comment has more replies than fit in one page
- **THEN** system fetches first page with `after: null`
- **AND** system checks `pageInfo.hasNextPage`
- **AND** while `hasNextPage` is true, fetches next page with `after: <endCursor>`
- **AND** continues until `hasNextPage` is false

#### Scenario: No replies
- **WHEN** a comment has no replies
- **THEN** system returns empty replies vector
- **AND** does not make pagination requests

### Requirement: Sort chronologically
The system SHALL ensure all comments and replies are ordered by `createdAt` timestamp in ascending order.

#### Scenario: Comments ordered correctly
- **WHEN** multiple pages of comments are fetched
- **THEN** final comments vector is sorted by `createdAt` ascending

#### Scenario: Replies ordered correctly
- **WHEN** multiple pages of replies are fetched
- **THEN** replies vector for each comment is sorted by `createdAt` ascending

### Requirement: Handle deleted authors
The system SHALL replace null author fields with `<deleted>` placeholder string.

#### Scenario: Comment with deleted author
- **WHEN** comment has null author field
- **THEN** system uses `<deleted>` as author login

#### Scenario: Reply with deleted author
- **WHEN** reply has null author field
- **THEN** system uses `<deleted>` as author login

#### Scenario: Original post with deleted author
- **WHEN** original post has null author
- **THEN** system uses `<deleted>` as author login

### Requirement: Propagate pagination errors
The system SHALL return errors if pagination fails at any point.

#### Scenario: API error during pagination
- **WHEN** any pagination request fails (network error, timeout, etc.)
- **THEN** system returns error immediately
- **AND** does not return partial results

#### Scenario: Rate limit during pagination
- **WHEN** API returns 403 status during pagination
- **THEN** system returns rate limit error
