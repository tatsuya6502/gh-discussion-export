## Why

The Discussion data has been fetched, but needs to be formatted as a lossless Markdown archive file. This change implements the output generation that creates the properly formatted `discussion.md` file according to the exact specification in `specs/markdown-output-generation/spec.md`.

## What Changes

- Create `output.rs` module for Markdown formatting
- Generate header section with discussion metadata
- Format original post with author info
- Format comments and replies in hierarchical structure (##/###/#### headings)
- Write output file with UTF-8 encoding and LF line endings
- Ensure body content is emitted verbatim except for heading escape (prefixing `#` at line start with backslash to preserve document structure)
- Add unit tests for output format validation

## Capabilities

### New Capabilities
- `markdown-output-generation`: Format and write discussion as Markdown

### Modified Capabilities
- None

## Impact

- New module: `output.rs`
- Depends on: graphql-client (for Discussion model)
- No changes to existing code
