## Context

GitHub Discussions can have hundreds or thousands of comments and replies. The GraphQL API returns these in paginated form using cursor-based pagination. The graphql-client change provides the basic query execution, but doesn't handle pagination. This change implements the pagination logic to retrieve complete discussion data.

**Constraints:**
- MUST paginate until completion (per `specs/discussion-fetching/spec.md` "Paginate comments" requirement)
- MUST maintain chronological ordering (createdAt ascending)
- MUST handle deleted users (null author → `<deleted>`)
- Discussion number is provided by user; owner/repo from CLI args

## Goals / Non-Goals

**Goals:**
- Implement cursor-based pagination for comments
- Implement cursor-based pagination for replies
- Fetch all comments and replies for a discussion
- Preserve thread structure (which reply belongs to which comment)
- Ensure chronological ordering

**Non-Goals:**
- Markdown output formatting (handled by `markdown-output` change)
- CLI integration (handled by `integration` change)

## Decisions

### Pagination Strategy: Cursor-based loops
Use `while has_next_page` loops with `after: cursor` parameter.

**Rationale:** GitHub GraphQL uses cursor-based pagination (standard GraphQL pattern). Each response includes `pageInfo.hasNextPage` and `pageInfo.endCursor`. We fetch with `after: null` initially, then use the returned cursor for subsequent requests.

**Alternative considered:** Relay-style pagination with first/last - Rejected because we want all items, not fixed page sizes.

### Ordering: Sort during assembly
Sort comments and replies by `createdAt` ascending after fetching all pages.

**Rationale:** While each page may be ordered, combining multiple pages doesn't guarantee global ordering. Sorting after all pages are fetched ensures correct chronological order. This is simpler than requesting sorted pages from the API.

### Thread Structure Preservation
Maintain parent-child relationships in memory using nested vectors.

**Rationale:** The output format requires thread hierarchy (Comment → Replies). Keeping the structure as `Vec<Comment>` where each `Comment` has `replies: Vec<Reply>` preserves this naturally. Flattening would require tracking parent IDs and rebuilding structure later.

### Deleted User Handling
Replace null author with placeholder string `<deleted>` during fetch.

**Rationale:** Per `specs/discussion-fetching/spec.md` "Handle deleted authors" requirement, we should handle missing authors by printing `<deleted>`. Doing this during fetch simplifies the output layer - it doesn't need to handle Option types. This is a data normalization concern, not presentation.

### Error Handling During Pagination
Fail immediately on any pagination error (don't attempt partial results).

**Rationale:** Per `specs/discussion-fetching/spec.md` "Propagate pagination errors" requirement, pagination failure is a hard error. Attempting to return partial data would violate the "lossless" requirement. The user should see a clear error and retry.

### Infinite Loop Protection
Validate that `pageInfo.endCursor` is not null when `pageInfo.hasNextPage` is true.

**Rationale:** GitHub's GraphQL API should theoretically never return `hasNextPage: true` with `endCursor: null`, but API bugs or edge cases could cause this condition. Without validation, the pagination loop would continue infinitely with the same `after: null` parameter. The code now checks: if `has_next_page` is true but `end_cursor` is None, return `Error::ApiInvariant` with a descriptive message.

**Implementation:** `src/fetch.rs:210-214` and `src/fetch.rs:263-267`

### GraphQL Error Handling in Pagination
Check for GraphQL errors in every pagination query response.

**Rationale:** The main `execute_query` function checks for GraphQL errors, but pagination queries use `execute_query_raw` which returns raw JSON. Without explicit error checking, GraphQL errors (like invalid fields, type mismatches, or rate limiting) would be returned as generic JSON parse errors. The `execute_query_raw` helper now checks the `errors` field and returns a descriptive `Error::GraphQL` if any errors are present.

**Implementation:** `src/fetch.rs:280-293`

### DISCUSSION_QUERY Simplification
Fetch only discussion metadata in the initial query, not comments or replies.

**Rationale:** The original `DISCUSSION_QUERY` fetched the first 100 comments and replies along with discussion metadata. However, `fetch_discussion` immediately discards this data and re-fetches everything via `fetch_all_comments` and `fetch_all_replies` to ensure complete pagination. Fetching comments/replies in the initial query was wasteful - it doubled response size and provided no benefit. The simplified query now only fetches metadata (id, title, number, url, createdAt, body, author), relying on pagination queries for all comment/reply data.

**Implementation:** `src/graphql.rs:9-31` (simplified from 54 lines to 23 lines)

### Enhanced Error Messages for Type Mismatches
Provide diagnostic error messages when GraphQL fragment spreads don't match.

**Rationale:** The `COMMENTS_QUERY` uses `... on Discussion` fragment spread. If the provided node ID doesn't match a Discussion type (e.g., a Repository ID or Issue ID), the fragment won't match and the response won't contain a `comments` field. The original error "Response missing 'comments' field" was technically correct but didn't help users understand the root cause. The enhanced error now states: "Response missing 'comments' field - the node ID may not be a Discussion", making it clear that the ID type might be wrong.

**Implementation:** `src/fetch.rs:318-322`

Similarly, when the `node` field itself is null (ID doesn't exist), the error now distinguishes between "missing field" vs "null value": "Node is null - the ID may not be a valid Discussion".

**Implementation:** `src/fetch.rs:312-316` and `src/fetch.rs:342-346`

### Reply Fetching Optimization
Fetch the first page of reply nodes in `COMMENTS_QUERY` to avoid unnecessary API calls for comments without replies.

**Rationale:** `fetch_discussion` calls `fetch_all_replies` for every comment. Without optimization, this results in N API calls for N comments, even if only M comments have replies (where M << N). By fetching the first page of reply nodes (100 items) in the initial `COMMENTS_QUERY`, we can:
1. Check if `comment.replies.nodes` has any data OR `comment.replies.page_info.has_next_page` is true
2. Only call `fetch_all_replies` if replies actually exist
3. Skip the API call for comments with zero replies

This reduces API calls from N (all comments) to M (comments with replies). For a discussion with 50 comments where only 5 have replies, this reduces API calls from 50 to 5.

**Trade-off:** Slightly larger initial query responses, but significant reduction in total API calls for discussions with many reply-less comments.

## Risks / Trade-offs

| Risk | Mitigation |
| ------ | ------------ |
| Rate limiting during large discussions | The fetch will fail with 403; user can retry after quota resets |
| Memory usage for very large discussions | All data is held in memory; acceptable for CLI tool with typical discussion sizes |
| Discussion deleted during fetch | Will fail mid-pagination; acceptable as hard error per spec |

## Migration Plan

Not applicable - this is new functionality.

## Open Questions

1. Should we fetch comments and replies in parallel?
   - **Decision:** No. Sequential fetching is simpler and respects rate limits better. The tool is fast enough with sequential calls.

2. What if a comment is deleted between pagination calls?
   - **Decision:** This will cause a GraphQL error, which we treat as a hard error. The user can retry.
