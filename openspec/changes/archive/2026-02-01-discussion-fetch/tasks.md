## 1. Module Setup

- [x] 1.1 Create `src/fetch.rs` module
- [x] 1.2 Update `src/lib.rs` to declare `fetch` module with `pub(crate)` visibility

## 2. Comment Pagination

- [x] 2.1 Implement `fetch_all_comments(client, discussion_id)` function
- [x] 2.2 Initial query with `after: null`
- [x] 2.3 Loop while `page_info.has_next_page` is true
- [x] 2.4 Subsequent queries use `after: page_info.end_cursor`
- [x] 2.5 Accumulate comments across pages
- [x] 2.6 Return when `has_next_page` is false

## 3. Reply Pagination

- [x] 3.1 Implement `fetch_all_replies(client, comment_id)` function
- [x] 3.2 Initial query with `after: null`
- [x] 3.3 Loop while `page_info.has_next_page` is true
- [x] 3.4 Subsequent queries use `after: page_info.end_cursor`
- [x] 3.5 Accumulate replies across pages
- [x] 3.6 Return when `has_next_page` is false

## 4. Discussion Fetching

- [x] 4.1 Implement `fetch_discussion(client, owner, repo, number)` function
- [x] 4.2 Call GraphQL client to fetch discussion metadata
- [x] 4.3 Get discussion ID from response
- [x] 4.4 Call `fetch_all_comments` with discussion ID
- [x] 4.5 For each comment, call `fetch_all_replies`
- [x] 4.6 Replace null authors with `<deleted>` placeholder
- [x] 4.7 Sort comments by `createdAt` ascending
- [x] 4.8 Sort replies for each comment by `createdAt` ascending
- [x] 4.9 Return complete Discussion object

## 5. Unit Tests

- [x] 5.1 Add test for single page of comments (no pagination)
- [x] 5.2 Add test for multiple pages of comments (pagination loop)
- [x] 5.3 Add test for single page of replies
- [x] 5.4 Add test for multiple pages of replies
- [x] 5.5 Add test for deleted author handling
- [x] 5.6 Add test for chronological sorting
- [x] 5.7 Add test for error propagation during pagination
- [x] 5.8 Mock GraphQL client responses for testing

## 6. Integration with GraphQL Client

- [x] 6.1 Import GitHubClient from client module
- [x] 6.2 Import model types (Discussion, Comment, Reply)
- [x] 6.3 Ensure Error types are compatible
- [x] 6.4 Verify compilation with all dependencies
