## 1. Dependencies

- [ ] 1.1 Add to `Cargo.toml`: `reqwest` (blocking, rustls-tls), `serde`, `serde_json`
- [ ] 1.2 Add `chrono` for DateTime handling if needed

## 2. Data Models

- [ ] 2.1 Create `src/models.rs` module
- [ ] 2.2 Define `Author` struct with `login` field (optional)
- [ ] 2.3 Define `PageInfo` struct with `has_next_page`, `end_cursor` fields
- [ ] 2.4 Define `Reply` struct with id, databaseId, author, createdAt, body
- [ ] 2.5 Define `Comment` struct with id, databaseId, author, createdAt, body, replies, pageInfo
- [ ] 2.6 Define `Discussion` struct with title, number, url, createdAt, body, comments
- [ ] 2.7 Derive `Deserialize` on all models with `#[serde(rename)]` for camelCase conversion
- [ ] 2.8 Define `GraphQLError` and `ErrorResponse` structs

## 3. GraphQL Query

- [ ] 3.1 Create `src/graphql.rs` module
- [ ] 3.2 Define DISCUSSION_QUERY as static string with proper GraphQL syntax
- [ ] 3.3 Query should fetch: discussion title, number, url, createdAt, body
- [ ] 3.4 Query should fetch: comments with id, databaseId, createdAt, body, author, replies
- [ ] 3.5 Query should fetch: replies with id, databaseId, createdAt, body, author
- [ ] 3.6 Query should include pageInfo (hasNextPage, endCursor) for comments and replies

## 4. HTTP Client

- [ ] 4.1 Create `src/client.rs` module
- [ ] 4.2 Define `HttpClient` trait with `post(&self, url: &str, body: &str) -> Result<String, Error>` method
- [ ] 4.3 Add `#[cfg_attr(test, automock)]` to `HttpClient` trait for automatic mock generation
- [ ] 4.4 Implement `ReqwestClient` struct wrapping `reqwest::blocking::Client`
- [ ] 4.5 Implement `HttpClient` trait for `ReqwestClient` with proper headers (User-Agent, Authorization)
- [ ] 4.6 Implement `GitHubClient` struct with `new(http_client: Box<dyn HttpClient>, token: String) -> Self` constructor
- [ ] 4.7 Implement `execute_query(&self, query: &str, variables: serde_json::Value) -> Response` method
- [ ] 4.8 Set POST endpoint to `https://api.github.com/graphql`
- [ ] 4.9 Handle HTTP errors (non-200 status codes) and return appropriate Error
- [ ] 4.10 Check response for `errors` field and return Error if present
- [ ] 4.11 Parse response `data` field into Discussion struct using serde_json

## 5. Module Integration

- [ ] 5.1 Update `src/lib.rs` to declare `models`, `graphql`, `client` modules with `pub(crate)` visibility
- [ ] 5.2 Ensure no compilation errors

## 6. Unit Tests

- [ ] 6.1 Add test for GraphQL query string validity
- [ ] 6.2 Add test for Discussion deserialization from sample JSON
- [ ] 6.3 Add test for Comment deserialization with replies
- [ ] 6.4 Add test for null author handling
- [ ] 6.5 Add test for GraphQL error response parsing
- [ ] 6.6 Add test for HTTP error handling (401, 403) using `MockHttpClient`
- [ ] 6.7 Add test for successful query execution with mocked `HttpClient`
- [ ] 6.8 Ensure `use mockall::automock;` is guarded by `#[cfg(test)]`
