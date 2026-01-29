## Context

Per `specs/markdown-output-generation/spec.md` "Heading hierarchy" requirement, the output format is strictly defined with a fixed structure. The Markdown must preserve the discussion content losslessly for use as LLM input. The heading hierarchy reflects thread structure: original post (##), comments (###), replies (####). This change implements that formatting.

**Constraints:**
- MUST follow exact format from `specs/markdown-output-generation/spec.md`
- MUST use UTF-8 encoding, LF line endings
- MUST emit body verbatim (no HTML escaping, no Markdown prettification)
- Heading hierarchy: ## (original post), ### (comment), #### (reply)

## Goals / Non-Goals

**Goals:**
- Generate header with title, discussion ID, URL, created at, author
- Format original post section
- Format all comments with replies
- Use proper heading hierarchy (##/###/####)
- Write file with correct encoding and line endings
- Preserve body content exactly as received

**Non-Goals:**
- Content processing or summarization
- Markdown reformatting or prettification
- HTML rendering or conversion

## Decisions

### Output Construction: String concatenation
Build output as a single String using `format!` macros and string concatenation.

**Rationale:** The output format is straightforward and doesn't require complex template rendering. String concatenation is simple and has no dependencies. Alternative (template engine like `tera`) would be overkill.

### Author Format
Use `_author: <login> (<timestamp>)_` format (italic with underscore).

**Rationale:** This format is specified in `specs/markdown-output-generation/spec.md` "Comment formatting" scenario. The underscore creates italic Markdown which makes author info visually distinct from content. The timestamp is ISO8601 for machine parsing.

### Header Information
Include: title, discussion ID (owner/repo#number), URL, created at, author.

**Rationale:** Per `specs/markdown-output-generation/spec.md`, these fields provide provenance and traceability. The URL allows cross-referencing back to the source. The discussion ID uniquely identifies the discussion.

### Body Handling: Verbatim copy
Write body content exactly as received from API with no processing.

**Rationale:** The tool is designed for "lossless" archival. Any processing (HTML escaping, Markdown normalization, whitespace trimming) would lose information. The LLM downstream can handle raw content.

### File Writing: Direct std::fs::write
Use `std::fs::write` for file output.

**Rationale:** Simple and ensures atomic write. Alternative (buffered writer) is unnecessary for the typical file sizes involved. UTF-8 is Rust's default string encoding, so no special handling needed.

### Line Endings: Explicit LF
Ensure LF line endings by using `\n` in format strings (not `\r\n`).

**Rationale:** Per `specs/markdown-output-generation/spec.md` "File encoding" scenario, line endings must be LF. On Windows, `\n` in Rust strings converts to CRLF in some cases, so we must ensure we're writing actual LF bytes.

## Risks / Trade-offs

| Risk | Mitigation |
|------|------------|
| Discussion title contains Markdown special characters | Title is in heading context; this is valid Markdown |
| Body content contains Markdown that breaks structure | Body is plain text; not an issue in practice |
| Large file size with many comments | File size is proportional to discussion size; acceptable for CLI tool |

## Migration Plan

Not applicable - this is new functionality.

## Open Questions

1. Should we validate that body content doesn't contain malicious HTML/script injection?
   - **Decision:** No. The output is plain Markdown for archival. If the original discussion contains script tags, they're preserved. This is a feature, not a bug - lossless archival.
