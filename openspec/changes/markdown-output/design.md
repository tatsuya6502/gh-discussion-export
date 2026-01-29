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

### Body Handling: Verbatim copy with heading escape
Write body content exactly as received from API with minimal processing.

**Processing rule:** Escape Markdown heading syntax (`#`) at the start of body lines by prefixing with backslash (e.g., `##` → `\##`).

**Rationale:** The tool is designed for "lossless" archival. Body content should be preserved verbatim for human readability and LLM processing. However, unescaped headings in body content would break the document structure (e.g., a user's `## My Section` inside a reply would be interpreted as a new section heading). Backslash escaping is the minimal change needed to preserve document structure while keeping content readable when rendered. This is a structural necessity, not content modification.

### File Writing: Direct std::fs::write
Use `std::fs::write` for file output.

**Rationale:** Simple and sufficient for this use case. `std::fs::write` is a convenience wrapper around `File::create` and `write_all`. It does not provide atomic replacement semantics. For true atomic writes, the canonical pattern would be: write to a temporary file in the same directory, optionally call `File::sync_all()` on the temp (if crash-durability is required), then use `std::fs::rename(temp, dest)` to atomically replace the target. The `rename` operation provides atomic semantics on both POSIX and Windows. For this CLI tool, `std::fs::write` is acceptable since partial writes are unlikely and the file can be regenerated if needed. UTF-8 is Rust's default string encoding, so no special handling needed.

### Line Endings: Explicit LF
Ensure LF line endings by using `\n` in format strings (not `\r\n`).

**Rationale:** Per `specs/markdown-output-generation/spec.md` "File encoding" scenario, line endings must be LF. `std::fs::write` writes bytes exactly as-is without conversion. While our format strings use `\n`, the real risk is that input body content from the API might already contain CRLF sequences. To ensure pure LF output, we must normalize any CRLF (`\r\n`) in input content to LF (`\n`) before writing.

## Risks / Trade-offs

| Risk | Mitigation |
| --- | --- |
| Discussion title contains Markdown special characters | Title is in heading context; this is valid Markdown |
| Body content contains Markdown that breaks structure | Body is plain text; not an issue in practice |
| Large file size with many comments | File size is proportional to discussion size; acceptable for CLI tool |

## Migration Plan

Not applicable - this is new functionality.

## Open Questions

1. Should we validate that body content doesn't contain malicious HTML/script injection?
   - **Decision:** No. The output is plain Markdown for archival. If the original discussion contains script tags, they're preserved. This is a feature, not a bug - lossless archival.
