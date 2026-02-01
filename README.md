# GitHub Discussion Export

A simple CLI tool that downloads GitHub Discussions and saves them as lossless Markdown files.

## Purpose

Exports a complete GitHub Discussion (original post + all comments + all replies) to a single Markdown file suitable for archival or LLM input.

**Key characteristics:**
- **Lossless**: Content is preserved verbatim (no summarization, reformatting, or interpretation).
- **Complete**: Fetches all comments and replies via cursor-based pagination.
- **Structured**: Hierarchical Markdown format (`## Original Post` / `## Comments` → `### Comment N` → `#### Reply N`).

## Prerequisites

- **Rust toolchain** (latest stable) - Install from [rustup.rs](https://rustup.rs/)
- **GitHub CLI** (`gh`) - Install from [cli.github.com](https://cli.github.com/), then authenticate:
  ```bash
  gh auth login
  ```

## Installation

### From GitHub (recommended)

```bash
cargo install --git https://github.com/tatsuya6502/gh-discussion-export.git
```

### From source

```bash
# Clone the repository
git clone https://github.com/tatsuya6502/gh-discussion-export.git
cd gh-discussion-export

# Build and install
cargo install --path .
```

## Usage

### Basic Command Syntax

```bash
gh-discussion-export --owner <OWNER> --repo <REPO> --number <NUMBER> [--output <PATH>]
```

### Required Arguments

| Argument | Description |
|:-------- |:----------- |
| `--owner` | Repository owner (user or organization) |
| `--repo` | Repository name |
| `--number` | Discussion number |

### Optional Arguments

| Argument | Description | Default |
|:-------- |:----------- |:------- |
| `--output` | Output file path | `<number>-discussion.md` |

### Help

```bash
gh-discussion-export --help
```

## Examples

### Export a discussion from a public repository

```bash
gh-discussion-export \
  --owner rust-lang \
  --repo rust \
  --number 12345
```

This creates a file named `12345-discussion.md` in the current directory.

### Export with custom output path

```bash
gh-discussion-export \
  --owner cli \
  --repo cli \
  --number 993 \
  --output my-discussion-archive.md
```

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

_author: username (2024-01-15T10:30:00Z)_

<original body content>

---

## Comments

### Comment 1

_author: username (2024-01-15T11:00:00Z)_

<comment body>

#### Reply 1.1

_author: username (2024-01-15T11:30:00Z)_

<reply body>

### Comment 2
...
```

**Key specifications:**
- **Encoding**: UTF-8 with LF line endings.
- **Fidelity**: Body content emitted verbatim (no HTML escaping, Markdown prettification, or whitespace trimming).
- **Heading escape**: If comment or reply bodies contain Markdown headings (lines starting with `#`), they are escaped with backslash (e.g., `\#`) to preserve document structure.
- **Deleted users**: Shown as `<deleted>`.
- **Ordering**: All content in chronological order (`createdAt` ascending).

## Architecture

### How It Works

The tool fetches discussion data from GitHub's GraphQL API and formats it as Markdown:

1. **Authentication**: Retrieves GitHub token via `gh auth token` (requires GitHub CLI).
2. **GraphQL Queries**: Queries GitHub's GraphQL API for discussion metadata.
3. **Pagination**: Uses cursor-based pagination to fetch all comments and replies.
4. **Formatting**: Generates structured Markdown with hierarchical headings.
5. **Output**: Writes to a single UTF-8 encoded file with LF line endings.

### OpenSpec-Driven Development

This project follows the OpenSpec methodology &mdash; a specification-first approach where all changes are designed and documented before implementation.

The specification is located in [`openspec/specs`](openspec/specs) and includes detailed requirements, design decisions, and implementation tasks for each modular change.

#### Module Organization

The codebase is organized into the following specifications:

- **cli-parsing** &mdash; Command-line argument parsing with clap.
- **gh-auth** &mdash; GitHub CLI authentication and token retrieval.
- **error-handling** &mdash; Error types with thiserror.
- **github-api-models** &mdash; serde models for GraphQL API responses.
- **graphql-client** &mdash; HTTP client and GraphQL query execution.
- **discussion-fetching** &mdash; Cursor-based pagination for discussions, comments, and replies.
- **markdown-output-generation** &mdash; Markdown formatting and file writing.

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines on setting up the development environment, running tests, and submitting pull requests.

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.
