## Why

The Markdown output in the original post section places the `---` divider immediately after the body content without a blank line. This causes CommonMark-compliant renderers to misinterpret the preceding text as a heading when `---` is parsed as an underline. Adding a blank line before the divider ensures proper rendering across all Markdown processors.

## What Changes

- Add blank line before `---` separator in original post section

## Capabilities

### New Capabilities
- None

### Modified Capabilities
- `markdown-output-generation`: The separator formatting requirement is changing to ensure proper rendering across Markdown processors

## Impact

- **Affected code**: `src/output.rs` - specifically `generate_original_post()` function
- **Tests**: Existing unit tests in `output.rs` will need updates to expect blank line before `---` separator in original post
- **Breaking change**: No - this is purely a formatting improvement that maintains output structure
