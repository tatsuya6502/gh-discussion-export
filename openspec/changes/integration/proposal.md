## Why

All the individual modules have been built (CLI parsing, auth, GraphQL client, fetching, output), but they haven't been wired together into a working application. This change integrates all components into the main.rs file to create a functional CLI tool.

## What Changes

- Wire all modules together in `main.rs`
- Connect CLI args → auth → client → fetch → output pipeline
- Implement error handling and user-facing error messages
- Ensure executable works end-to-end

## Capabilities

### New Capabilities
- None (integration of existing capabilities)

### Modified Capabilities
- None

## Impact

- Updates `main.rs` to implement complete workflow
- Depends on: foundation, graphql-client, discussion-fetch, markdown-output
- Creates working CLI executable
