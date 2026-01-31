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
