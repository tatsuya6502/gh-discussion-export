## ADDED Requirements

### Requirement: README documentation
The system SHALL include a README.md file in the project root with complete documentation.

#### Scenario: README sections
- **WHEN** README.md is viewed
- **THEN** it includes project description
- **AND** it includes installation instructions
- **AND** it includes prerequisites
- **AND** it includes usage examples
- **AND** it includes contribution guidelines
- **AND** it includes license information

#### Scenario: Project description
- **WHEN** README description is read
- **THEN** it explains what the tool does
- **AND** it mentions the output format (Markdown)
- **AND** it references the fidelity goal (lossless archive)

#### Scenario: Installation instructions
- **WHEN** README installation section is read
- **THEN** it includes `cargo install` instructions
- **AND** it includes building from source instructions
- **AND** it mentions pre-compiled binary option if available

#### Scenario: Prerequisites documentation
- **WHEN** README prerequisites section is read
- **THEN** it explicitly lists GitHub CLI (`gh`) requirement
- **AND** it explains `gh auth login` requirement
- **AND** it mentions Rust toolchain requirement for building from source

### Requirement: Usage documentation
The README SHALL include clear usage instructions and examples.

#### Scenario: Command reference
- **WHEN** README usage section is read
- **THEN** it shows all command-line arguments
- **AND** it explains each argument
- **AND** it indicates which arguments are required vs optional

#### Scenario: Basic usage example
- **WHEN** README examples section is read
- **THEN** it includes basic usage example
- **AND** example shows real repo, owner, and discussion number
- **AND** example is copy-pasteable

#### Scenario: Output customization example
- **WHEN** README examples section is read
- **THEN** it shows how to use `--output` argument
- **AND** example demonstrates custom output path

#### Scenario: Help command reference
- **WHEN** README is read
- **THEN** it mentions `--help` flag for built-in help

### Requirement: Contribution guidelines
The system SHALL include contribution guidelines for developers.

#### Scenario: CONTRIBUTING.md file
- **WHEN** developer wants to contribute
- **THEN** CONTRIBUTING.md file exists
- **AND** it explains how to set up development environment
- **AND** it explains how to run tests
- **AND** it explains pull request workflow

#### Scenario: Development setup
- **WHEN** CONTRIBUTING.md is followed
- **THEN** developer can clone repository
- **AND** developer can run `cargo build`
- **AND** developer can run `cargo test`

### Requirement: License information
The README SHALL include license information.

#### Scenario: License section
- **WHEN** README license section is read
- **THEN** it specifies the license (MIT)
- **AND** it includes copyright notice if applicable

### Requirement: Output format documentation
The README SHALL describe the output format to set user expectations.

#### Scenario: Format description
- **WHEN** README output format section is read
- **THEN** it explains the Markdown structure
- **AND** it mentions heading hierarchy (##/###/####)
- **AND** it mentions lossless fidelity (content preserved verbatim except heading escape for document structure)
