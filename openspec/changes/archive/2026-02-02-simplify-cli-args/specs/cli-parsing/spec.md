## MODIFIED Requirements

### Requirement: Parse command-line arguments
The system SHALL parse command-line arguments using clap derive macros with the following structure:
- Positional argument: `<number>` - Discussion number (required, first positional argument)
- `--repo <OWNER/REPO>`: GitHub repository in owner/name format (optional if in Git repository, otherwise required)
- `-o <path>, --output <path>`: Output file path (optional, defaults to `<number>-discussion.md`)

#### Scenario: Valid arguments with explicit repo
- **WHEN** user provides discussion number as positional argument and `--repo rust-lang/rust`
- **THEN** system successfully parses arguments with owner="rust-lang" and repo="rust"

#### Scenario: Valid arguments with automatic repo detection
- **WHEN** user provides discussion number as positional argument within a Git repository and omits `--repo`
- **THEN** system successfully detects owner/repo from Git repository and parses arguments

#### Scenario: Missing discussion number
- **WHEN** user omits the positional discussion number argument
- **THEN** system displays error message indicating discussion number is required and exits with non-zero status

#### Scenario: Missing repo outside Git repository
- **WHEN** user provides discussion number but omits `--repo` outside of a Git repository
- **THEN** system displays error message indicating --repo is required and exits with non-zero status

#### Scenario: Output path omitted
- **WHEN** user provides required arguments but omits `-o` or `--output`
- **THEN** system defaults to `<number>-discussion.md` where `<number>` is the discussion number

#### Scenario: Help flag
- **WHEN** user invokes with `--help` or `-h`
- **THEN** system displays usage information including all arguments, their purposes, and default values

### Requirement: Validate argument types
The system SHALL validate that arguments match expected types.

#### Scenario: Discussion number is not numeric
- **WHEN** user provides non-numeric value for positional discussion number argument
- **THEN** system displays error message and exits with non-zero status

#### Scenario: Discussion number is zero or negative
- **WHEN** user provides zero or negative value for positional discussion number argument
- **THEN** system displays error message and exits with non-zero status

#### Scenario: Invalid repo format
- **WHEN** user provides `--repo` value not in OWNER/REPO format (e.g., "rust-lang", "rust-lang/rust/extra")
- **THEN** system displays error message indicating correct format and exits with non-zero status

#### Scenario: Empty string arguments
- **WHEN** user provides empty string for `--repo` argument
- **THEN** system displays error message and exits with non-zero status

## REMOVED Requirements

### Requirement: Parse command-line arguments (legacy structure)
**Reason**: Replaced by unified `--repo <OWNER/REPO>` format and positional argument for discussion number

**Note**: This is a breaking change to pre-1.0 software. The only current user is the developer, so no migration documentation is needed.

### Requirement: Validate argument types (legacy --number flag)
**Reason**: Discussion number is now a positional argument instead of a flag

**Note**: Validation logic preserved, now applied to positional argument instead of flag.
