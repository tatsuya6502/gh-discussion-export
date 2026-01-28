## Context

Unit tests mock the GitHub API, which is good for isolation but doesn't prove the tool works with real data. End-to-end tests call the actual GitHub API to validate the complete pipeline. This change focuses on integration testing against real discussions.

**Constraints:**
- Tests must be runnable by any developer with GitHub CLI auth
- Tests should use public discussions to avoid repository access issues
- Tests should be idempotent (can run multiple times safely)
- Tests must handle rate limiting gracefully

## Goals / Non-Goals

**Goals:**
- Create integration tests that call real GitHub API
- Test complete workflow from CLI to output file
- Test edge cases with real data
- Ensure output format matches specification

**Non-Goals:**
- Unit tests (already covered in previous changes)
- Performance/benchmark testing
- Load testing

## Decisions

### Test Framework: Rust's built-in `cargo test`
Use standard `#[test]` attribute in `tests/` directory.

**Rationale:** Rust's integrated test framework is sufficient for integration tests. Alternative (proptest, rstest) would add dependencies without significant benefit for this use case.

### Test Data: Public GitHub Discussions
Use well-known public discussions for testing.

**Rationale:** Public discussions are accessible to anyone with `gh auth login`. Using private repos would require all developers to have access to specific repositories. We'll use discussions from a popular open-source repository (e.g., rust-lang/rust).

### Test Organization: Separate files for test categories
Create `tests/basic_e2e_test.rs`, `tests/edge_cases_test.rs`, etc.

**Rationale:** Grouping related tests makes the suite more organized. Running `cargo test` will execute all of them.

### Fixture Discussions: Hardcoded discussion numbers
Use known discussion IDs in test code.

**Rationale:** Simplicity. We'll pick discussions with specific characteristics (small, large, with deleted users, etc.) and hardcode their numbers. This is fragile if discussions are deleted, but acceptable for a CLI tool where tests can be updated.

### Rate Limit Handling: Mark tests as `ignore` if needed
Use `#[ignore]` attribute for tests that hit rate limits.

**Rationale:** GitHub has rate limits (5000 points/hour for authenticated). We'll mark some tests as ignored so developers can run them explicitly with `cargo test -- --ignored` when needed.

## Risks / Trade-offs

| Risk | Mitigation |
|------|------------|
| Test discussions get deleted | Choose discussions from stable, popular repos |
| Rate limits prevent test runs | Use `#[ignore]` for heavy tests, document in README |
| Tests are flaky due to network | Use retry logic for transient failures |

## Migration Plan

Not applicable - this is test infrastructure only.

## Open Questions

1. Which repository should we use for test discussions?
   - **Decision:** Use rust-lang/rust (very active, unlikely to delete discussions, public)

2. Should we mock the API in CI and run real tests locally only?
   - **Decision:** For now, all tests are real. Mocking can be added later if CI becomes a concern. The tool is early-stage and manual testing is acceptable.
