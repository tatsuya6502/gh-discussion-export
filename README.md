# GitHub Discussion Export

A CLI tool that downloads GitHub Discussions and saves them as lossless Markdown files. Designed as a source for LLM-based specification synthesis.

## Purpose

Exports a complete GitHub Discussion (original post + all comments + all replies) to a single Markdown file suitable for archival or LLM input.

**Key characteristics:**
- **Lossless**: Content is preserved verbatim (no summarization, reformatting, or interpretation)
- **Complete**: Fetches all comments and replies via cursor-based pagination
- **Structured**: Hierarchical Markdown format (## Original Post → ### Comments → #### Replies)

## Installation

```bash
cargo install gh-discussion-export
```

## Prerequisites

- **GitHub CLI** (`gh`) - must be installed and authenticated:
  ```bash
  gh auth login
  ```

## Usage

```bash
# Basic usage - outputs to <number>-discussion.md
gh-discussion-export \
  --owner rust-lang \
  --repo rust \
  --number 12345

# Custom output path
gh-discussion-export \
  --owner rust-lang \
  --repo rust \
  --number 12345 \
  --output my-discussion.md
```

### Arguments

| Argument | Required | Description |
|----------|----------|-------------|
| `--owner` | Yes | Repository owner (user or org) |
| `--repo` | Yes | Repository name |
| `--number` | Yes | Discussion number |
| `--output` | No | Output file path (default: `<number>-discussion.md`) |

## Output Format

The tool generates a single Markdown file with this structure:

```md
# <Discussion Title>

Discussion: <owner>/<repo>#<number>
URL: https://github.com/<owner>/<repo>/discussions/<number>

Created at: 2024-01-15T10:30:00Z
Author: username

---

## Original Post

_author: username (2024-01-15T10:30:00Z)

<original body content>

---

## Comments

### Comment 1
_author: username (2024-01-15T11:00:00Z)

<comment body>

#### Reply 1.1
_author: username (2024-01-15T11:30:00Z)

<reply body>

### Comment 2
...
```

**Key specifications:**
- UTF-8 encoding, LF line endings
- Body content emitted verbatim (no HTML escaping, Markdown prettification, or whitespace trimming)
- Deleted users shown as `<deleted>`
- All content in chronological order

## Architecture

The project is organized into 7 modular changes (OpenSpec-driven):

1. **foundation** - CLI parsing, GitHub CLI auth, error types
2. **graphql-client** - reqwest HTTP client, GraphQL queries, serde models
3. **discussion-fetch** - Cursor-based pagination for comments/replies
4. **markdown-output** - Format generation, file writing
5. **integration** - Wire all modules in `main.rs`
6. **e2e-tests** - Integration tests against real GitHub API
7. **documentation** - README and contribution guidelines

### Implementation Progress

- [x] 1. **foundation** (25 tasks)
  - CLI argument parsing with clap
  - GitHub CLI authentication
  - Error types with thiserror
  - Project structure setup
- [x] 2. **graphql-client** (37 tasks)
  - reqwest HTTP client setup
  - GraphQL query definitions
  - serde response models
  - API error handling
- [x] 3. **discussion-fetch** (35 tasks)
  - Discussion metadata fetching
  - Cursor-based pagination for comments
  - Cursor-based pagination for replies
  - Chronological sorting
  - Deleted user handling
- [ ] 4. **markdown-output** (51 tasks)
  - Header generation with metadata
  - Original post formatting (##)
  - Comment formatting (###)
  - Reply formatting (####)
  - File writing (UTF-8, LF)
- [ ] 5. **integration** (32 tasks)
  - Wire all modules in main.rs
  - Complete workflow pipeline
  - Error handling and messaging
- [ ] 6. **e2e-tests** (55 tasks)
  - Integration tests with real GitHub API
  - Pagination tests
  - Edge case tests (deleted users, large discussions)
  - Output format validation
- [ ] 7. **documentation** (48 tasks)
  - README.md with installation and usage
  - Contribution guidelines
  - License information

## Development

```bash
# Build
cargo build --release

# Run tests
cargo test

# Run integration tests (requires GitHub auth)
cargo test -- --ignored
```

## License

MIT
