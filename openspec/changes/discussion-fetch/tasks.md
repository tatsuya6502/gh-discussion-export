## 1. Module Setup

- [ ] 1.1 Create `src/fetch.rs` module
- [ ] 1.2 Update `src/lib.rs` to declare `fetch` module with `pub(crate)` visibility

## 2. Comment Pagination

- [ ] 2.1 Implement `fetch_all_comments(client, discussion_id)` function
- [ ] 2.2 Initial query with `after: null`
- [ ] 2.3 Loop while `page_info.has_next_page` is true
- [ ] 2.4 Subsequent queries use `after: page_info.end_cursor`
- [ ] 2.5 Accumulate comments across pages
- [ ] 2.6 Return when `has_next_page` is false

## 3. Reply Pagination

- [ ] 3.1 Implement `fetch_all_replies(client, comment_id)` function
- [ ] 3.2 Initial query with `after: null`
- [ ] 3.3 Loop while `page_info.has_next_page` is true
- [ ] 3.4 Subsequent queries use `after: page_info.end_cursor`
- [ ] 3.5 Accumulate replies across pages
- [ ] 3.6 Return when `has_next_page` is false

## 4. Discussion Fetching

- [ ] 4.1 Implement `fetch_discussion(client, owner, repo, number)` function
- [ ] 4.2 Call GraphQL client to fetch discussion metadata
- [ ] 4.3 Get discussion ID from response
- [ ] 4.4 Call `fetch_all_comments` with discussion ID
- [ ] 4.5 For each comment, call `fetch_all_replies`
- [ ] 4.6 Replace null authors with `<deleted>` placeholder
- [ ] 4.7 Sort comments by `createdAt` ascending
- [ ] 4.8 Sort replies for each comment by `createdAt` ascending
- [ ] 4.9 Return complete Discussion object

## 5. Unit Tests

- [ ] 5.1 Add test for single page of comments (no pagination)
- [ ] 5.2 Add test for multiple pages of comments (pagination loop)
- [ ] 5.3 Add test for single page of replies
- [ ] 5.4 Add test for multiple pages of replies
- [ ] 5.5 Add test for deleted author handling
- [ ] 5.6 Add test for chronological sorting
- [ ] 5.7 Add test for error propagation during pagination
- [ ] 5.8 Mock GraphQL client responses for testing

## 6. Integration with GraphQL Client

- [ ] 6.1 Import GitHubClient from client module
- [ ] 6.2 Import model types (Discussion, Comment, Reply)
- [ ] 6.3 Ensure Error types are compatible
- [ ] 6.4 Verify compilation with all dependencies
