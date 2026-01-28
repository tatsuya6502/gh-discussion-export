## Why

The GitHub Discussion Exporter needs to communicate with GitHub's GraphQL API to fetch discussion data. This change implements the GraphQL client layer including HTTP client setup, query definitions, and response models.

## What Changes

- Add `reqwest` dependency for HTTP requests
- Add `serde` and `serde_json` dependencies for JSON serialization
- Create GraphQL query module with discussion fetch query
- Define response model structs matching GitHub GraphQL schema
- Implement client module that executes queries with authentication
- Add unit tests for query construction and response parsing

## Capabilities

### New Capabilities
- `graphql-client`: GraphQL query execution and response parsing
- `github-api-models`: Data structures matching GitHub's GraphQL schema

### Modified Capabilities
- None

## Impact

- New dependencies: `reqwest`, `serde`, `serde_json`
- New modules: `graphql.rs` (queries), `client.rs` (HTTP client), `models.rs` (response types)
- No changes to existing code
- Foundation change must be completed first (for authentication and error types)
