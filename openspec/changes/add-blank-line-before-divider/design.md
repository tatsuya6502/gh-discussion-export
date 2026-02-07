## Context

The Markdown output generator currently creates section dividers using `---` immediately after content, without a preceding blank line. This works in some Markdown renderers but causes issues in CommonMark-compliant renderers, where `---` following text is interpreted as a heading underline rather than a horizontal rule.

**Current behavior in `src/output.rs`:**
- `generate_header()` ends with: `"\n---\n"` (line 78)
- `generate_original_post()` ends with: `"\n---\n"` (line 102)

The problem manifests when the content immediately before `---` contains text - that text becomes the heading text, and `---` becomes the underline.

**Constraints:**
- Must maintain lossless fidelity requirement - only formatting changes, no content modification
- Must preserve UTF-8 encoding and LF line endings
- Should not affect body content processing (heading escape, CRLF normalization remain unchanged)

## Goals / Non-Goals

**Goals:**
- Ensure `---` dividers render consistently as horizontal rules across all Markdown processors
- Add blank lines before all `---` separators in generated Markdown
- Maintain existing test coverage with updated expectations

**Non-Goals:**
- Changing the overall document structure or heading hierarchy
- Modifying body content processing logic
- Adding configuration options for divider formatting

## Decisions

### Decision 1: Add blank line before all `---` separators

**Choice:** Modify format strings to include `\n\n` before `---` instead of `\n`

**Rationale:**
- CommonMark spec requires blank lines before thematic breaks (`---`) to prevent misinterpretation as setext headings
- This is the minimal change that ensures correct rendering
- Aligns with Markdown best practices - thematic breaks should be surrounded by blank lines for clarity

**Code changes:**
```rust
// Before:
format!("...\n---\n")

// After:
format!("...\n\n---\n")
```

**Alternatives considered:**
1. **Use different divider syntax** (e.g., `***` or `___`)
   - Rejected: All three variants (`---`, `***`, `___`) have the same CommonMark parsing rules. Blank line is required regardless.

2. **Escape the divider** (e.g., `\---`)
   - Rejected: This would render as literal text `\---` rather than a horizontal rule.

3. **Remove dividers entirely**
   - Rejected: Dividers provide visual structure and separation between sections. Removing them would make the output harder to read.

### Decision 2: Modify both `generate_header()` and `generate_original_post()`

**Choice:** Apply the fix to both functions for consistency

**Rationale:**
- Both functions use the same pattern of ending with `---`
- Even though the header divider doesn't currently cause issues (last line is "Author: <login>" which doesn't look like heading text), ensuring consistency prevents future issues
- Makes the code more maintainable - same pattern everywhere

## Risks / Trade-offs

### Risk: Test failures due to changed expectations

**Risk:** Existing unit tests assert that content ends with `---\n`, but will now end with `\n---\n`
**Mitigation:** Update test assertions to expect the blank line. Tests in `src/output.rs` that check for `ends_with("---\n")` will need to be updated to `ends_with("\n\n---\n")`.

### Risk: Output file size increase

**Risk:** Adding two blank lines (one per divider) increases file size
**Impact:** Negligible - 2 additional bytes per discussion
**Mitigation:** None needed. This is an acceptable trade-off for correct rendering.

### Trade-off: Blank line in source readability

**Trade-off:** The raw Markdown source will have more vertical spacing
**Acceptance:** This improves readability and follows Markdown best practices. The blank line makes the section break more visually obvious in source code.

## Migration Plan

**Deployment steps:**
1. Update `src/output.rs` format strings in `generate_header()` and `generate_original_post()`
2. Update unit tests in `src/output.rs` to expect blank lines before dividers
3. Run `cargo test` to verify all tests pass
4. Run `cargo clippy` and `cargo fmt` to maintain code quality standards

**Rollback strategy:**
- Simple revert of format string changes if unexpected issues arise
- No data migration required - this only affects output format

**Validation:**
- Run existing test suite to ensure no regressions
- Manually inspect generated Markdown to verify blank lines appear before dividers
- Test rendering in multiple Markdown viewers (GitHub, VS Code preview, etc.)

## Open Questions

None - the change is straightforward and well-scoped.
