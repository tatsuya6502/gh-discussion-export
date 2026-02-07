## Why

The Markdown output currently places `---` dividers immediately after content without a blank line. This causes CommonMark-compliant renderers to misinterpret the preceding text as a heading when `---` is parsed as an underline. Adding blank lines before dividers ensures proper rendering across all Markdown processors.

## What Changes

- Add blank line before `---` separator in header section
- Add blank line before `---` separator in original post section
- Ensure consistent spacing around all dividers in generated Markdown

## Capabilities

### New Capabilities
- None

### Modified Capabilities
- `markdown-output-generation`: The separator formatting requirement is changing to ensure proper rendering across Markdown processors

## Impact

- **Affected code**: `src/output.rs` - specifically `generate_header()` and `generate_original_post()` functions
- **Tests**: Existing unit tests in `output.rs` will need updates to expect blank lines before `---` separators
- **Breaking change**: No - this is purely a formatting improvement that maintains output structure
