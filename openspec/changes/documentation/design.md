## Context

The tool is complete and functional, but lacks user-facing documentation. Users need to know:
- What the tool does
- How to install it
- How to use it (prerequisites, commands, examples)
- How to contribute

This change creates comprehensive documentation for users and contributors.

**Constraints:**
- Documentation should be clear and concise
- Examples should be copy-pasteable
- Prerequisites should be explicitly stated
- License information should match project intent

## Goals / Non-Goals

**Goals:**
- Create README with installation and usage instructions
- Provide clear examples
- Document prerequisites
- Add contribution guidelines
- Ensure documentation is accurate and complete

**Non-Goals:**
- API documentation (code is self-documenting or can use rustdoc later)
- Tutorial content (keep it reference-style)

## Decisions

### README Structure: Standard sections
Use sections: Description, Installation, Prerequisites, Usage, Examples, Contributing, License.

**Rationale:** Follows README best practices. Users expect this structure in open-source projects.

### Installation Instructions: Cargo and binary
Document both `cargo install` and pre-compiled binary options.

**Rationale:** `cargo install` is simplest for Rust users. Pre-compiled binaries are friendlier for non-Rust users. We'll document both even though binaries aren't built yet (future work).

### Prerequisites: Explicit about GitHub CLI
Clearly state that GitHub CLI (`gh`) must be installed and authenticated.

**Rationale:** This is the #1 support issue users will encounter. Being explicit upfront reduces confusion.

### Examples: Real-world scenarios
Include examples for common use cases.

**Rationale:** Abstract documentation is hard to follow. Real examples (e.g., "export a discussion from rust-lang/rust") make it concrete.

### Contribution Guidelines: Lightweight
Keep CONTRIBUTING.md simple with basic setup and workflow.

**Rationale:** This is a small tool, not a large project. Extensive contribution guidelines would be overkill. Basic "fork, branch, PR" workflow is sufficient.

### README Content Requirements
The README.md must include the following sections with specific content:

**Project Description:**
- Explains what the tool does (exports GitHub Discussions to Markdown)
- Mentions the output format (single Markdown file)
- References the fidelity goal (lossless archive, content preserved verbatim except heading escape for document structure)

**Installation Instructions:**
- `cargo install` command for Rust users
- Building from source instructions (`cargo build --release`)
- Pre-compiled binary option (if available in future)

**Prerequisites:**
- Explicitly lists GitHub CLI (`gh`) requirement
- Explains `gh auth login` requirement
- Mentions Rust toolchain requirement for building from source

**Usage Documentation:**
- Shows all command-line arguments with explanations
- Indicates which arguments are required vs optional
- Mentions `--help` flag for built-in help
- Includes copy-pasteable examples:
  - Basic usage example with real repo, owner, and discussion number
  - Output customization example showing `--output` argument

**Output Format Description:**
- Explains the Markdown structure
- Mentions heading hierarchy (# for discussion title, ## for sections, ### for comments, #### for replies)
- States lossless fidelity (content preserved verbatim except heading escape for document structure)

**Contribution Guidelines:**
- Links to or summarizes CONTRIBUTING.md
- Explains development setup (clone, build, test)

**Architecture Section:**
- Explains how the tool works (GraphQL API, cursor-based pagination, Markdown formatting)
- Describes OpenSpec-driven development methodology
- Links to specification at `openspec/specs`
- Lists module organization based on specifications

**License Information:**
- Specifies the license (MIT)
- Includes copyright notice if applicable

**CONTRIBUTING.md Content:**
- Development setup instructions (including OpenSpec CLI)
- OpenSpec-Driven Development explanation
- AI-Generated Code policy
- Testing, code style, and workflow guidelines

**Rationale:** These content requirements ensure the README is comprehensive and user-friendly, addressing common questions upfront and reducing support burden. The Architecture section provides transparency about the tool's internal workings and development approach, while CONTRIBUTING.md sets clear expectations for contributors.

## Risks / Trade-offs

| Risk | Mitigation |
| --- | --- |
| Documentation becomes outdated | Keep examples simple and focused on core functionality that won't change |
| Installation instructions may be wrong | Test instructions on fresh system before finalizing |

## Migration Plan

Not applicable - this is new documentation.

## Open Questions

1. What license should we use?
   - **Decision:** MIT or Apache-2.0 (standard for Rust tools). We'll document MIT for simplicity.

2. Should we include a "Roadmap" section?
   - **Decision:** No. The tool is feature-complete per specs.md. Future enhancements can be discussed in issues.
