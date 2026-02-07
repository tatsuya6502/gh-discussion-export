# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Blank line before `---` separators to ensure proper rendering in CommonMark-compliant Markdown renderers

## [0.1.0] &mdash; 2026-02-02

### Added
- Foundation
  - CLI argument parsing with `clap`
  - GitHub CLI authentication integration (`gh auth token`)
  - Error handling with `thiserror`
- GraphQL client
  - GraphQL client for GitHub API communication
  - Response models matching GitHub GraphQL schema
- Discussion fetching
  - Cursor-based pagination for comments and replies
  - Complete discussion retrieval (original post + all comments + all replies)
- Markdown output
  - Markdown output generation with hierarchical structure
  - UTF-8 encoded output files with LF line endings
  - Markdown heading escape (`#` → `\#`) to preserve document structure
- Integration
  - Wired all modules together into functional CLI tool
- Documentation
  - `README.md` with installation and usage documentation
  - `CONTRIBUTING.md` with development guidelines
- Git repository detection
  - Automatic repository detection from current Git directory
  - Unified `--repo <OWNER/REPO>` argument format
  - Positional argument for discussion number
  - Short form `-o` for `--output` flag

### Fixed
- None

[Unreleased]: https://github.com/tatsuya6502/gh-discussion-export/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/tatsuya6502/gh-discussion-export/releases/tag/v0.1.0
