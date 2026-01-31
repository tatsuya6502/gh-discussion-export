## ADDED Requirements

### Requirement: Parse command-line arguments
The system SHALL parse command-line arguments using clap derive macros with the following structure:
- `--owner <owner>`: GitHub repository owner (required)
- `--repo <repo>`: GitHub repository name (required)
- `--number <number>`: Discussion number (required)
- `--output <path>`: Output file path (optional, defaults to `<number>-discussion.md`)

#### Scenario: Valid arguments provided
- **WHEN** user provides all required arguments (`--owner`, `--repo`, `--number`)
- **THEN** system successfully parses arguments into a `CliArgs` struct

#### Scenario: Missing required argument
- **WHEN** user omits any required argument
- **THEN** system displays error message indicating which argument is missing and exits with non-zero status

#### Scenario: Output path omitted
- **WHEN** user provides required arguments but omits `--output`
- **THEN** system defaults to `<number>-discussion.md` where `<number>` is the discussion number

#### Scenario: Help flag
- **WHEN** user invokes with `--help` or `-h`
- **THEN** system displays usage information including all arguments, their purposes, and default values

### Requirement: Validate argument types
The system SHALL validate that arguments match expected types.

#### Scenario: Discussion number is not numeric
- **WHEN** user provides non-numeric value for `--number`
- **THEN** system displays error message and exits with non-zero status

#### Scenario: Empty string arguments
- **WHEN** user provides empty string for any required argument
- **THEN** system displays error message and exits with non-zero status
