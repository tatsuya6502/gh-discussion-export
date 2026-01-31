## 1. Dependencies

- [x] 1.1 Add to `Cargo.toml`: `reqwest` (blocking, rustls-tls), `serde`, `serde_json`
- [x] 1.2 Add `chrono` for DateTime handling if needed

## 2. Data Models

- [x] 2.1 Create `src/models.rs` module
- [x] 2.2 Define `Author` struct with `login` field (optional)
- [x] 2.3 Define `PageInfo` struct with `has_next_page`, `end_cursor` fields
- [x] 2.4 Define `Reply` struct with id, databaseId, author, createdAt, body
- [x] 2.5 Define `Comment` struct with id, databaseId, author, createdAt, body, replies, pageInfo
- [x] 2.6 Define `Discussion` struct with title, number, url, createdAt, body, comments
- [x] 2.7 Derive `Deserialize` on all models with `#[serde(rename)]` for camelCase conversion
- [x] 2.8 Define `GraphQLError` and `ErrorResponse` structs

## 3. GraphQL Query

- [x] 3.1 Create `src/graphql.rs` module
- [x] 3.2 Define DISCUSSION_QUERY as static string with proper GraphQL syntax
- [x] 3.3 Query should fetch: discussion title, number, url, createdAt, body
- [x] 3.4 Query should fetch: comments with id, databaseId, createdAt, body, author, replies
- [x] 3.5 Query should fetch: replies with id, databaseId, createdAt, body, author
- [x] 3.6 Query should include pageInfo (hasNextPage, endCursor) for comments and replies

## 4. HTTP Client

- [x] 4.1 Create `src/client.rs` module
- [x] 4.2 Define `HttpClient` trait with `post(&self, url: &str, body: &str) -> Result<String, Error>` method
- [x] 4.3 Add `#[cfg_attr(test, automock)]` to `HttpClient` trait for automatic mock generation
- [x] 4.4 Implement `ReqwestClient` struct wrapping `reqwest::blocking::Client`
- [x] 4.5 Implement `HttpClient` trait for `ReqwestClient` with proper headers (User-Agent, Authorization)
- [x] 4.6 Implement `GitHubClient` struct with `new(http_client: Box<dyn HttpClient>, token: String) -> Self` constructor
- [x] 4.7 Implement `execute_query(&self, query: &str, variables: serde_json::Value) -> Response` method
- [x] 4.8 Set POST endpoint to `https://api.github.com/graphql`
- [x] 4.9 Handle HTTP errors (non-200 status codes) and return appropriate Error
- [x] 4.10 Check response for `errors` field and return Error if present
- [x] 4.11 Parse response `data` field into Discussion struct using serde_json

## 5. Module Integration

- [x] 5.1 Update `src/lib.rs` to declare `models`, `graphql`, `client` modules with `pub(crate)` visibility
- [x] 5.2 Ensure no compilation errors

## 6. Unit Tests

- [x] 6.1 Add test for GraphQL query string validity
- [x] 6.2 Add test for Discussion deserialization from sample JSON
- [x] 6.3 Add test for Comment deserialization with replies
- [x] 6.4 Add test for null author handling
- [x] 6.5 Add test for GraphQL error response parsing
- [x] 6.6 Add test for HTTP error handling (401, 403) using `MockHttpClient`
- [x] 6.7 Add test for successful query execution with mocked `HttpClient`
- [x] 6.8 Ensure `use mockall::automock;` is guarded by `#[cfg(test)]`
