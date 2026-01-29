## ADDED Requirements

### Requirement: Generate Markdown header
The system SHALL generate a header section with discussion metadata.

#### Scenario: Header with all fields
- **WHEN** Markdown output is generated
- **THEN** header contains `# <Discussion Title>` as first line
- **AND** header contains `Discussion: <owner>/<repo>#<number>`
- **AND** header contains `URL: https://github.com/<owner>/<repo>/discussions/<number>`
- **AND** header contains `Created at: <ISO8601 timestamp>`
- **AND** header contains `Author: <login>`
- **AND** header ends with `---` separator

#### Scenario: Header with deleted author
- **WHEN** original post author is `<deleted>`
- **THEN** header contains `Author: <deleted>`

### Requirement: Generate original post section
The system SHALL generate the original post section with author and body.

#### Scenario: Original post formatting
- **WHEN** original post section is generated
- **THEN** section starts with `## Original Post` heading
- **AND** author line is `_author: <login> (<ISO8601>)_`
- **AND** body content follows author line
- **AND** section ends with `---` separator

#### Scenario: Body is verbatim
- **WHEN** body content is written
- **THEN** body is written exactly as received from API
- **AND** no HTML escaping is performed
- **AND** no Markdown prettification is performed
- **AND** whitespace is preserved
- **AND** Markdown heading syntax (`#`) at line start is escaped with backslash to preserve document structure
- **AND** CRLF line endings in input are normalized to LF

### Requirement: Generate comments section
The system SHALL generate a comments section containing all comments with nested replies.

#### Scenario: Comments section heading
- **WHEN** comments section is generated
- **THEN** section starts with `## Comments` heading

#### Scenario: Discussion with no comments
- **WHEN** discussion has zero comments
- **THEN** `## Comments` heading is still emitted
- **AND** no comment or reply subsections follow

#### Scenario: Comment formatting
- **WHEN** a comment is formatted
- **THEN** comment starts with `### Comment <N>` where N is 1-indexed position
- **AND** author line is `_author: <login> (<ISO8601>)_`
- **AND** body content follows author line

#### Scenario: Reply formatting
- **WHEN** a reply is formatted
- **THEN** reply is nested under parent comment
- **AND** reply starts with `#### Reply <N.M>` where N is comment number, M is reply number
- **AND** author line is `_author: <login> (<ISO8601>)_`
- **AND** body content follows author line

#### Scenario: Multiple comments
- **WHEN** multiple comments exist
- **THEN** comments are numbered sequentially (Comment 1, Comment 2, etc.)
- **AND** replies are numbered per comment (Reply 1.1, Reply 1.2, Reply 2.1, etc.)

#### Scenario: Comment with no replies
- **WHEN** a comment has no replies
- **THEN** comment body is written without any reply subsections

### Requirement: Write output file
The system SHALL write the formatted Markdown to a file.

#### Scenario: File encoding
- **WHEN** output file is written
- **THEN** file uses UTF-8 encoding
- **AND** file uses LF line endings
- **AND** file has no BOM

#### Scenario: Specified output path
- **WHEN** user provides `--output` argument
- **THEN** file is written to specified path

#### Scenario: Default output path
- **WHEN** user does not provide `--output` argument
- **THEN** file is written to `<number>-discussion.md` in current directory

### Requirement: Heading hierarchy
The system SHALL use proper Markdown heading levels for content structure.

#### Scenario: Correct heading levels
- **WHEN** Markdown is generated
- **THEN** discussion title uses `#` (level 1)
- **AND** section headers use `##` (level 2)
- **AND** comments use `###` (level 3)
- **AND** replies use `####` (level 4)

#### Scenario: No additional headings
- **WHEN** Markdown is generated
- **THEN** no other headings are introduced beyond the specified structure
