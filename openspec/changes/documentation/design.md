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

## Risks / Trade-offs

| Risk | Mitigation |
|------|------------|
| Documentation becomes outdated | Keep examples simple and focused on core functionality that won't change |
| Installation instructions may be wrong | Test instructions on fresh system before finalizing |

## Migration Plan

Not applicable - this is new documentation.

## Open Questions

1. What license should we use?
   - **Decision:** MIT or Apache-2.0 (standard for Rust tools). We'll document MIT for simplicity.

2. Should we include a "Roadmap" section?
   - **Decision:** No. The tool is feature-complete per specs.md. Future enhancements can be discussed in issues.
