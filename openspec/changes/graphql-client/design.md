## Context

The GitHub Discussion Exporter uses GitHub's GraphQL API as the data source (per specs.md). GraphQL is required because GitHub Discussions are only fully accessible via GraphQL - the REST API doesn't provide complete discussion data. This change builds the GraphQL client layer that will be used by the discussion-fetch change.

**Constraints:**
- Must use GraphQL API at `https://api.github.com/graphql`
- Must handle authentication via Bearer token (from foundation change)
- Must handle GraphQL-specific errors (syntax errors, validation errors)
- Response models must match GitHub's GraphQL schema

## Goals / Non-Goals

**Goals:**
- Set up reqwest HTTP client with proper headers
- Define GraphQL query for fetching discussion with comments and replies
- Create serde-compatible response models
- Implement query execution with error handling
- Enable unit testing for query construction and response parsing

**Non-Goals:**
- Pagination logic (handled by `discussion-fetch` change)
- Data fetching orchestration (handled by `discussion-fetch` change)
- Discussion data processing beyond basic deserialization

## Decisions

### HTTP Client: `reqwest`
Use `reqwest` with `rustls-tls` for HTTPS requests.

**Rationale:** `reqwest` is the standard HTTP client for Rust. It provides a clean API, proper connection pooling, and TLS support. The `rustls-tls` feature avoids OpenSSL dependency for better cross-platform compilation. Alternative `surf` or `attohttpc` are less mature.

### JSON Handling: `serde` and `serde_json`
Use `serde` derive macros for response models.

**Rationale:** `serde` is the de-facto standard for Rust serialization. The derive API provides compile-time verification of field names and types. `serde_json` is used for constructing GraphQL query variables and parsing responses.

### GraphQL Query Strategy
Embed GraphQL query as a static string in the code.

**Rationale:** The query is fixed and doesn't need to be dynamic. Embedding as a string avoids dependency on GraphQL query builders. Alternative (graphql_client crate) generates types from `.graphql` files but adds complexity and may not align with our exact needs.

**Alternative considered:** Using a dedicated GraphQL client library like `graphql_client` - Rejected because it requires code generation and adds complexity for a single fixed query.

### Query Structure
Fetch discussion, comments, and replies in a single query with nested structure.

**Rationale:** GitHub's GraphQL API supports nested queries. Fetching everything in one query minimizes API calls and provides a natural data structure. Pagination cursors will be handled by the `discussion-fetch` change.

### Response Model Organization
Create separate structs for each GraphQL type: `Discussion`, `Comment`, `Reply`, `Author`.

**Rationale:** Mirrors the GitHub GraphQL schema structure and provides type-safe access. Using flattened structs would lose semantic clarity.

### Error Handling
Define GraphQL-specific error type that wraps HTTP errors and parse errors.

**Rationale:** GraphQL errors come in two forms: HTTP-level errors (401, 429, etc.) and GraphQL errors within a 200 response (validation errors, syntax errors). Both need to be surfaced clearly to users.

## Risks / Trade-offs

| Risk | Mitigation |
|------|------------|
| GitHub GraphQL schema changes may break our models | Use `#[serde(rename)]` and optional fields to tolerate schema changes; monitor for deprecations |
| Large discussions may cause memory issues | Response is streamed; models use String not Vec<u8> to avoid extra allocation |
| Rate limiting may cause failures | Client will propagate 403 errors; calling code can implement retry logic |

## Migration Plan

Not applicable - this is new functionality with no existing users.

## Open Questions

1. Should the query be split into multiple smaller queries or keep as one large nested query?
   - **Decision:** Keep as one query for now. The `discussion-fetch` change may need to split it if pagination proves complex.

2. Should we use async reqwest or blocking?
   - **Decision:** Use blocking client for simplicity. CLI tool doesn't need concurrency.
