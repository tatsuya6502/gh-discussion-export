## Why

The GraphQL client provides the capability to execute queries, but the full discussion data requires handling pagination for both comments and replies. This change implements the data fetching logic that retrieves complete discussion threads by paginating through all comments and replies.

## What Changes

- Create `fetch.rs` module with discussion retrieval logic
- Implement pagination loop for comments using cursor-based pagination
- Implement pagination loop for replies within each comment
- Fetch discussion by number with all comments and replies
- Add unit tests for pagination logic
- Ensure chronological ordering (createdAt ascending)

## Capabilities

### New Capabilities
- `discussion-fetching`: Complete discussion retrieval with pagination

### Modified Capabilities
- None

## Impact

- New module: `fetch.rs`
- Depends on: foundation (auth, errors), graphql-client (client, models)
- No changes to existing code
