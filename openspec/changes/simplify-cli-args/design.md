## Context

The current CLI interface (`--owner`, `--repo`, `--number`) is inconsistent with GitHub's standard `OWNER/REPO` format used across GitHub CLI, documentation, and URLs. The tool already depends on `gh` CLI for authentication via `gh auth token`, so extending this dependency for repository detection is natural and consistent.

Current implementation in `src/cli.rs` uses clap derive API with separate `owner` and `repo` fields, both marked as required. The `--number` flag is also a required argument. Validation functions ensure non-empty strings and positive numbers.

**Constraints**:
- Must maintain compatibility with existing authentication flow (`gh auth token`)
- Must preserve all current validation logic (type checking, empty string detection)
- Breaking change is acceptable (this is v0.x pre-1.0 tool)
- No new Rust dependencies allowed

## Goals / Non-Goals

**Goals:**
- Simplify CLI interface to match GitHub's `OWNER/REPO` convention
- Reduce typing for users working within Git repositories
- Provide short form `-o` for frequently used `--output` flag
- Maintain clear error messages when repository cannot be auto-detected
- Reuse existing `gh` CLI dependency (already used for authentication)

**Non-Goals:**
- Supporting multiple GitHub hosts (Enterprise Server) via repo detection
- Caching detected repository information
- Supporting custom git remote selection logic (delegate entirely to `gh`)
- Backward compatibility with legacy `--owner`/`--repo` flags

## Decisions

### 1. Use `gh repo view` for repository detection

**Decision**: Delegate repository detection entirely to `gh repo view --json owner,name --jq '.owner.login + "/" + .name'`

**Rationale**:
- `gh` CLI already has sophisticated remote resolution logic (handles `origin`, `upstream`, GitHub Enterprise, SSH vs HTTPS)
- Reusing `gh` ensures consistent behavior with other `gh` commands
- No need to re-implement complex Git remote parsing in Rust
- User already has `gh` installed (required for authentication)

**Alternatives considered**:
- **Parse `git remote -v` output in Rust**: Would require handling SCP syntax (`git@github.com:owner/repo.git`), HTTPS URLs, SSH URLs, and multiple remotes. More complex and error-prone.
- **Use `git2` crate**: Adds a new native dependency, increases binary size, and re-implements logic `gh` already has.

### 2. Defer repo detection until after CLI parsing

**Decision**: Make `--repo` optional in clap, then call `gh repo view` only when omitted

**Rationale**:
- Clean separation between argument parsing and repository detection
- Allows clap to handle all structural validation before executing external command
- Error messages can be contextual (knowing `--repo` was omitted)

**Implementation flow**:
```
1. clap parses CLI arguments (repo: Option<String>)
2. If repo is None, execute gh repo view
3. Parse output or surface gh's error directly
4. Combine with validation results
```

### 3. Parse OWNER/REPO with custom validator

**Decision**: Use clap's custom `value_parser` to validate `--repo` format

**Rationale**:
- Provides immediate feedback on invalid format (e.g., "owner" or "owner/repo/extra")
- Splits OWNER/REPO during validation, avoiding later parsing logic
- Consistent with existing validation pattern (`validate_non_empty_string`, `validate_positive_number`)

**Validation rules**:
- Must contain exactly one `/` separator
- Both owner and repo parts must be non-empty
- Trailing `.git` suffix is allowed (common in copy-paste from URLs)

### 4. Positional argument for discussion number

**Decision**: Use clap's positional argument syntax: `#[arg(value_name = "NUMBER")]`

**Rationale**:
- Reduces visual clutter (primary input comes first)
- Matches common CLI patterns (e.g., `gh pr view 123`)
- Number is always required and is the primary operation target

### 5. Module organization

**Decision**: Keep repository detection logic in `src/cli.rs` alongside argument parsing

**Rationale**:
- Tightly coupled to CLI argument structure (knows when `--repo` is omitted)
- No other modules need this capability
- Avoids premature abstraction

**Code structure**:
```rust
// src/cli.rs
pub struct CliArgs {
    pub number: u64,           // positional
    pub repo: Option<String>,  // optional --repo flag
    pub output: Option<String>, // -o, --output
}

impl CliArgs {
    pub fn repo_owner(&self) -> Result<String, Error> { ... }
    pub fn repo_name(&self) -> Result<String, Error> { ... }
    fn detect_from_git() -> Result<String, Error> { ... }
    fn parse_repo_flag(s: &str) -> Result<(String, String), Error> { ... }
}
```

## Risks / Trade-offs

### Risk: `gh` command fails or returns unexpected output

**Mitigation**: Surface `gh` errors directly to user with suggestion to specify `--repo` explicitly. Add explicit check for `gh` availability with clear error message.

### Risk: Breaking change for existing users

**Mitigation**: This is pre-1.0 software. Migration guide in REMOVED requirements section of spec. Clear error messages if old flags are used (clap will automatically reject unknown flags).

### Risk: Performance cost of spawning `gh` process

**Mitigation**: Only executes when `--repo` is omitted (opt-in performance trade-off). For explicit `--repo`, no external process is spawned. Acceptable cost for improved UX.

### Risk: `gh` CLI version incompatibilities

**Mitigation**: `gh repo view --json` is stable API. If `gh` is too old, error message will suggest upgrading or using `--repo` explicitly.

## Migration Plan

### For users upgrading from previous version

1. Update command invocation:
   ```bash
   # Old
   gh-discussion-export --owner rust-lang --repo rust --number 123

   # New
   gh-discussion-export --repo rust-lang/rust 123
   ```

2. For Git repositories, omit `--repo`:
   ```bash
   # Inside git clone of rust-lang/rust
   gh-discussion-export 123
   ```

3. Update CI/CD scripts:
   - Replace `--owner`/`--repo` with `--repo OWNER/REPO`
   - Move `--number` value to positional argument
   - Or run inside Git repository and omit `--repo` entirely

### Rollback strategy

If critical issues are found:
1. Revert to previous CLI structure
2. Users can continue using old syntax
3. Auto-detection feature removed

### Testing strategy

- Unit tests for argument parsing (valid/invalid repo formats)
- Unit tests for `OWNER/REPO` parsing logic
- Integration tests for `gh repo view` invocation (mockable or real)
- Update all existing CLI tests to use new argument structure

## Open Questions

None. The approach is straightforward given existing `gh` dependency and well-defined `gh repo view` API.
