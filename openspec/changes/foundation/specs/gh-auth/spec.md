## ADDED Requirements

### Requirement: Retrieve GitHub authentication token
The system SHALL retrieve an authentication token by calling the GitHub CLI command `gh auth token` and capturing stdout.

#### Scenario: Successful token retrieval
- **WHEN** GitHub CLI is installed and user is authenticated
- **THEN** system returns the token string

#### Scenario: GitHub CLI not installed
- **WHEN** `gh` command is not found on system
- **THEN** system exits with error message directing user to install GitHub CLI

#### Scenario: User not authenticated
- **WHEN** `gh auth token` returns an error or non-zero exit code
- **THEN** system exits with error message directing user to run `gh auth login`

### Requirement: Token format validation
The system SHALL validate that the retrieved token is a non-empty string.

#### Scenario: Empty token
- **WHEN** `gh auth token` returns an empty string
- **THEN** system returns an authentication error
