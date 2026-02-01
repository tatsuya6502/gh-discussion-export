# GitHub Discussion Export - Context for AI Reviewers

## Project Overview

This is a **specification-first** Rust CLI tool built using the OpenSpec workflow. The implementation is organized into 7 sequential changes, each with complete artifacts (proposal, design, specs, tasks).

## The Original Specification

The tool is built from a detailed specification originally captured in `specs.md` (not committed). Key requirements from that spec:

### What This Tool Does
- Fetches **one** GitHub Discussion by number
- Retrieves original post, **all** comments, and **all** replies
- Outputs a **single** Markdown file

### What This Tool Does NOT Do (Explicit Non-Goals)
- ❌ Any form of summarization or normalization
- ❌ Extracting decisions or proposals
- ❌ Editing, trimming, or reformatting Markdown content
- ❌ Multiple output formats or HTML output
- ❌ OpenSpec conversion or LLM integration

### Critical Requirements

1. **Lossless Fidelity**
   - Body content MUST be emitted verbatim
   - **Exception:** Markdown heading syntax (`#`) at line start is escaped with backslash to preserve document structure (see `openspec/changes/markdown-output/design.md` for rationale)
   - No HTML rendering or other escaping
   - No trimming of whitespace or trailing newlines
   - UTF-8 encoding, LF line endings

2. **Complete Data Retrieval**
   - All comments MUST be fetched
   - All replies MUST be fetched
   - Pagination MUST continue until completion
   - Ordering MUST be chronological (`createdAt` ascending)

3. **Fixed Output Format**
   - Heading depth: `##` (sections), `###` (comments), `####` (replies)
   - Author format: `_author: <login> (<timestamp>)_`
   - No additional sections may be introduced

4. **Authentication**
   - MUST use GitHub CLI (`gh auth token`)
   - No Personal Access Token management
   - Exit with clear error if `gh` not installed/authenticated

5. **API**
   - GitHub GraphQL API only (REST explicitly disallowed)
   - Endpoint: `https://api.github.com/graphql`

## Code Organization

### Module Structure

```
src/
├── main.rs          # Integration point (all modules wired together)
├── cli.rs           # Argument parsing with clap
├── auth.rs          # GitHub CLI token retrieval
├── error.rs         # Error types with thiserror
├── client.rs        # reqwest HTTP client for GraphQL
├── graphql.rs       # GraphQL query definitions
├── models.rs        # serde response models
├── fetch.rs         # Pagination logic for comments/replies
└── output.rs        # Markdown formatting and file writing
```

### Dependencies

- **clap** v4 - CLI argument parsing (derive API)
- **thiserror** - Error type definitions
- **reqwest** - HTTP client (blocking, rustls-tls)
- **serde / serde_json** - JSON serialization
- **tokio** not used - blocking client for simplicity

## What Reviewers Should Look For

### Critical Paths
1. **Pagination loops** in `fetch.rs` - must use `pageInfo.hasNextPage` and `pageInfo.endCursor`
2. **Body content handling** in `output.rs` - must be verbatim except for heading escape (prefixing `#` at line start with backslash)
3. **Deleted user handling** - null authors should become `<deleted>` string
4. **Authentication error handling** - clear messages directing users to `gh auth login`

### Common Pitfalls
- ⚠️ Don't add HTML or other escaping to body content (violates lossless requirement; heading escape is the only exception)
- ⚠️ Don't stop pagination early (spec requires "MUST paginate until completion")
- ⚠️ Don't use the REST API (GraphQL is required)
- ⚠️ Don't manage PATs directly (must use `gh auth token`)

### Test Coverage
- Unit tests for each module (co-located with implementation)
- Integration tests in `tests/` directory against real GitHub API
- Mark heavy tests as `#[ignore]` to avoid rate limits

## OpenSpec Artifacts

Each change in `openspec/changes/<name>/` contains:
- `proposal.md` - Why this change exists, what it adds
- `design.md` - Technical decisions, trade-offs, alternatives
- `specs/**/*.md` - Detailed requirements with scenarios (WHEN/THEN format)
- `tasks.md` - Implementation checklist (checkbox format)

Reviewers can reference these artifacts to understand intent and verify completeness.

## Intended Usage

This tool is designed as the **first stage** of a multi-step specification workflow:
1. Export discussion to Markdown (this tool)
2. Feed Markdown to LLM (Claude, etc.) for specification synthesis
3. Review and refine generated specifications

The output is meant to be a **primary source document** - interpretation is explicitly deferred to downstream tools.

## Development Workflow

After making any code changes, run the following commands to verify the changes:

```bash
# 1. Check compilation
cargo check

# 2. Run linter
cargo clippy --lib --tests --all-features --all-targets

# 3. Format code
cargo fmt --all

# 4. Run tests
cargo test
```

All commands must pass before committing changes. This ensures:
- Code compiles without errors
- No clippy warnings (except expected dead_code warnings for modules not yet integrated)
- Code is properly formatted
- All tests pass

## Git Workflow

**Important**: Git commits and pushes are performed manually by the user. Do not automatically commit or push changes.

The user will:
- Review the changes made by the AI
- Create commits manually using their preferred method
- Push to GitHub when ready

The AI assistant should:
- Make code changes as requested
- Run verification commands (cargo check, clippy, fmt, test)
- Provide summaries of changes
- **NOT** execute git commit or git push commands

## PR Review Workflow

When working on pull requests, use this command to view unresolved review comments:

```bash
# Get unresolved review comments for a PR
gh pr-review review view --repo tatsuya6502/gh-discussion-export --pr <PR_NUMBER> --unresolved
```

This requires the `gh pr-review` extension to be installed.

<details>
<summary>📦 Installation instructions</summary>

```bash
gh extension install agynio/gh-pr-review
```

</details>
