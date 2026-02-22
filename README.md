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
gh-discussion-export [--repo <OWNER/REPO>] [--output <PATH>] <NUMBER>
```

### Positional Argument

| Argument | Description |
|:-------- |:----------- |
| `<NUMBER>` | Discussion number (required, first positional argument) |

### Optional Arguments

| Argument | Description | Default |
|:-------- |:----------- |:------- |
| `--repo <OWNER/REPO>` | GitHub repository in OWNER/REPO format | Auto-detected from Git repository |
| `-o <PATH>, --output <PATH>` | Output file path | `<number>-discussion.md` |
| `--no-assets` | Skip downloading embedded assets (images) | Download assets |
| `-j <N>, --parallel <N>` | Number of parallel asset downloads | 4 |

### Help

```bash
gh-discussion-export --help
```

## Automatic Repository Detection

When you omit the `--repo` argument, the tool automatically detects the repository from your current Git directory using the GitHub CLI:

```bash
# Inside a Git repository
gh-discussion-export 123
```

This is equivalent to:

```bash
gh-discussion-export --repo $(gh repo view --json owner,name --jq '.owner.login + "/" + .name') 123
```

## Examples

### Export a discussion from a public repository

```bash
gh-discussion-export --repo rust-lang/rust 12345
```

This creates a file named `12345-discussion.md` in the current directory.

### Export with automatic repository detection

```bash
# Inside your Git repository
gh-discussion-export 993
```

### Export with custom output path

```bash
gh-discussion-export --repo cli/cli 993 -o my-discussion-archive.md
```

### Export a discussion with embedded assets

When a discussion contains embedded images (from GitHub's user-attachments assets), the tool automatically downloads them and transforms the URLs to reference local files:

```bash
gh-discussion-export --repo owner/repo 123
```

This creates:
- `123-discussion.md` - Markdown file with local asset references
- `123-discussion-assets/` - Directory containing downloaded images

### Export with custom parallelism

Control the number of concurrent asset downloads:

```bash
gh-discussion-export --repo owner/repo 123 -j 8
```

### Skip asset download

Export without downloading embedded assets:

```bash
gh-discussion-export --repo owner/repo 123 --no-assets
```

## Asset Download Behavior

When downloading assets, the tool:

1. **Detects** all GitHub asset URLs from HTML `<img>` tags and Markdown image syntax
2. **Deduplicates** assets by UUID (same asset referenced multiple times is only downloaded once)
3. **Downloads** assets in parallel using the GitHub authentication token
4. **Transforms** URLs to reference local paths while preserving original URLs

### Asset Directory Naming

Downloaded assets are stored in a directory named `<discussion-number>-discussion-assets/` next to the Markdown file.

Each asset is named using its UUID with the appropriate file extension (determined from the Content-Type header):
- `6c72b402-4a5c-45cc-9b0a-50717f8a09a7.png`
- `7d83c513-5b6d-46dd-a01b-61728e8b0a8b.jpg`

### URL Transformation

GitHub asset URLs in the exported Markdown are transformed to reference local paths:

**Markdown images:**
```md
<!-- Before -->
![Diagram](https://github.com/user-attachments/assets/6c72b402-4a5c-45cc-9b0a-50717f8a09a7)

<!-- After -->
![Diagram](123-discussion-assets/6c72b402-4a5c-45cc-9b0a-50717f8a09a7.png)<!-- https://github.com/user-attachments/assets/6c72b402-4a5c-45cc-9b0a-50717f8a09a7 -->
```

**HTML images:**
```html
<!-- Before -->
<img src="https://github.com/user-attachments/assets/6c72b402-4a5c-45cc-9b0a-50717f8a09a7" alt="Diagram" />

<!-- After -->
<img src="123-discussion-assets/6c72b402-4a5c-45cc-9b0a-50717f8a09a7.png" alt="Diagram" data-original-url="https://github.com/user-attachments/assets/6c72b402-4a5c-45cc-9b0a-50717f8a09a7" />
```

The original URL is preserved for reference:
- **Markdown**: In an HTML comment after the image
- **HTML**: In the `data-original-url` attribute

### Authentication

Assets from private repositories require authentication, using the same GitHub token as discussion access (retrieved via `gh auth token`).

**Error messages:**
- **401 Unauthorized**: Invalid token - run `gh auth login` to re-authenticate
- **403 Forbidden**: Authentication failed or access denied - ensure you have access to the repository

### Offline Viewing

Once assets are downloaded, the exported Markdown can be viewed offline. All images reference local files, so no network connection is required.

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
