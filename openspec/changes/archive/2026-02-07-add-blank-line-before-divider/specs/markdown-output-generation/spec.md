## MODIFIED Requirements

### Requirement: Generate original post section
The system SHALL generate the original post section with author and body.

#### Scenario: Original post formatting
- **WHEN** original post section is generated
- **THEN** section starts with `## Original Post` heading
- **AND** author line is `_author: <login> (<ISO8601>)_`
- **AND** body content follows author line
- **AND** blank line precedes `---` separator

#### Scenario: Original post with deleted author
- **WHEN** original post author is `<deleted>`
- **THEN** author line is `_author: <deleted> (<ISO8601>)_`
- **AND** body content formatting rules still apply (verbatim, heading escape, CRLF normalization)
- **AND** section structure remains unchanged

#### Scenario: Body is verbatim
- **WHEN** body content is written
- **THEN** body is written exactly as received from API
- **AND** no HTML escaping is performed
- **AND** no Markdown prettification is performed
- **AND** whitespace is preserved
- **AND** Markdown heading syntax (`#`) at line start is escaped with backslash to preserve document structure
- **AND** CRLF line endings in input are normalized to LF
