## Context

The Markdown output generator creates section dividers using `---`. The `generate_original_post()` function places the divider immediately after the body content without a preceding blank line. This works in some Markdown renderers but causes issues in CommonMark-compliant renderers, where `---` following text is interpreted as a heading underline rather than a horizontal rule.

**Current behavior in `src/output.rs`:**
- `generate_header()` ends with: `"\n\n---\n"` (line 78) - already has blank line ✓
- `generate_original_post()` ends with: `"\n---\n"` (line 102) - missing blank line ✗

The problem manifests when the content immediately before `---` contains text - that text becomes the heading text, and `---` becomes the underline.

**Constraints:**
- Must maintain lossless fidelity requirement - only formatting changes, no content modification
- Must preserve UTF-8 encoding and LF line endings
- Should not affect body content processing (heading escape, CRLF normalization remain unchanged)

## Goals / Non-Goals

**Goals:**
- Ensure `---` divider in original post section renders consistently as horizontal rule across all Markdown processors
- Add blank line before `---` separator in `generate_original_post()`
- Maintain existing test coverage with updated expectations

**Non-Goals:**
- Changing the overall document structure or heading hierarchy
- Modifying body content processing logic
- Adding configuration options for divider formatting

## Decisions

### Decision 1: Add blank line before `---` separator in `generate_original_post()`

**Choice:** Modify the format string in `generate_original_post()` to include `\n\n` before `---` instead of `\n`

**Rationale:**
- CommonMark spec requires blank lines before thematic breaks (`---`) to prevent misinterpretation as setext headings
- This is the minimal change that ensures correct rendering
- Aligns with Markdown best practices - thematic breaks should be surrounded by blank lines for clarity
- `generate_header()` already has the blank line, so only `generate_original_post()` needs the fix

**Code changes:**
```rust
// Before (line 102):
"## Original Post\n\n_author: {} ({})_\n\n{}\n---\n"

// After:
"## Original Post\n\n_author: {} ({})_\n\n{}\n\n---\n"
```

**Alternatives considered:**
1. **Use different divider syntax** (e.g., `***` or `___`)
   - Rejected: All three variants (`---`, `***`, `___`) have the same CommonMark parsing rules. Blank line is required regardless.

2. **Escape the divider** (e.g., `\---`)
   - Rejected: This would render as literal text `\---` rather than a horizontal rule.

3. **Remove dividers entirely**
   - Rejected: Dividers provide visual structure and separation between sections. Removing them would make the output harder to read.

## Risks / Trade-offs

### Risk: Test failures due to changed expectations

**Risk:** Existing unit tests assert that content ends with `---\n`, but will now end with `\n---\n`
**Mitigation:** Update test assertions to expect the blank line. Tests in `src/output.rs` that check for `ends_with("---\n")` will need to be updated to `ends_with("\n\n---\n")`.

### Risk: Output file size increase

**Risk:** Adding one blank line increases file size
**Impact:** Negligible - 1 additional byte per discussion
**Mitigation:** None needed. This is an acceptable trade-off for correct rendering.

### Trade-off: Blank line in source readability

**Trade-off:** The raw Markdown source will have more vertical spacing
**Acceptance:** This improves readability and follows Markdown best practices. The blank line makes the section break more visually obvious in source code.

## Migration Plan

**Deployment steps:**
1. Update `src/output.rs` format string in `generate_original_post()`
2. Update unit tests in `src/output.rs` to expect blank line before divider in original post
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
