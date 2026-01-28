## Why

Unit tests provide coverage for individual modules, but we need to validate that the complete application works correctly with real GitHub API calls. This change adds end-to-end integration tests that verify the tool against actual GitHub Discussions.

## What Changes

- Add `cargo` test configuration for integration tests
- Create `tests/` directory with integration test files
- Implement tests against real GitHub Discussions
- Add tests for edge cases (deleted users, large discussions, etc.)
- Set up test fixtures (known discussion IDs, test repositories)

## Capabilities

### New Capabilities
- None (testing capability)

### Modified Capabilities
- None

## Impact

- New `tests/` directory
- Test configuration in `Cargo.toml`
- Dependencies: Integration change must be complete
