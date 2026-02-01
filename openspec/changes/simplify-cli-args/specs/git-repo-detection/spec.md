## ADDED Requirements

### Requirement: Detect repository from Git remote
The system SHALL automatically detect GitHub repository owner and name from the current Git repository when `--repo` argument is omitted.

#### Scenario: Git repository with origin remote
- **WHEN** user omits `--repo` argument while in a Git repository with origin pointing to GitHub
- **THEN** system executes `gh repo view --json owner,name --jq '.owner.login + "/" + .name'`
- **AND** system parses output to extract owner and repository name
- **AND** system uses detected values for the GitHub API call

#### Scenario: Git repository with non-origin remote
- **WHEN** user omits `--repo` argument while in a Git repository without origin remote
- **THEN** system allows `gh repo view` to use its default remote selection logic
- **AND** system uses the repository detected by `gh` CLI

#### Scenario: Outside Git repository
- **WHEN** user omits `--repo` argument while outside of any Git repository
- **THEN** system displays error message: "not a git repository (or any of the parent directories): .git"
- **AND** system suggests specifying `--repo` explicitly
- **AND** system exits with non-zero status

#### Scenario: Git repository without GitHub remote
- **WHEN** user omits `--repo` argument in a Git repository without any GitHub remotes
- **THEN** system delegates error handling to `gh repo view`
- **AND** system displays the error from `gh` CLI
- **AND** system exits with non-zero status

### Requirement: Parse gh repo view output
The system SHALL parse the JSON output from `gh repo view` command to extract owner and repository name.

#### Scenario: Valid JSON output
- **WHEN** `gh repo view` returns `{"owner":{"login":"tatsuya6502"},"name":"gh-discussion-export"}`
- **THEN** system extracts owner="tatsuya6502" and repo="gh-discussion-export"
- **AND** system combines them into "tatsuya6502/gh-discussion-export" format

#### Scenario: Invalid JSON output
- **WHEN** `gh repo view` returns malformed or unexpected JSON
- **THEN** system displays error message indicating failed to parse repository information
- **AND** system suggests specifying `--repo` explicitly
- **AND** system exits with non-zero status

#### Scenario: gh command not available
- **WHEN** `gh` command is not installed or not in PATH
- **THEN** system displays error message indicating `gh` CLI is required
- **AND** system exits with non-zero status

### Requirement: Prioritize explicit repo argument
The system SHALL prioritize explicitly provided `--repo` argument over automatic Git repository detection.

#### Scenario: Both --repo and Git repository present
- **WHEN** user provides `--repo cli/cli` while in a different Git repository (e.g., user/repo)
- **THEN** system uses the explicitly provided "cli/cli" value
- **AND** system ignores the Git repository detection
