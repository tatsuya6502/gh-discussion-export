## Why

Current CLI requires separate `--owner` and `--repo` flags, making the command verbose and inconsistent with GitHub's standard `OWNER/REPO` format used across GitHub CLI (`gh`), repository URLs, and documentation. Users frequently work within Git repositories where the owner/repo can be automatically detected, yet they must still manually specify these values. This creates friction and reduces the developer experience.

## What Changes

- **BREAKING**: Replace separate `--owner` and `--repo` arguments with unified `--repo <OWNER/REPO>` format
- Convert `--number` from required flag to positional argument (first positional argument)
- Add `-o` short form for `--output` flag
- Add automatic repository detection: when `--repo` is omitted, infer owner/repo from current Git repository using `gh repo view`
- Update help text and usage examples to reflect new interface

## Capabilities

### New Capabilities
- `git-repo-detection`: Automatic detection of GitHub owner and repository name from current Git repository

### Modified Capabilities
- `cli-parsing`: Argument structure is changing significantly
  - Separate `--owner`/`--repo` flags → unified `--repo <OWNER/REPO>`
  - `--number` flag → positional argument
  - New `-o` short form for `--output`
  - New optional behavior: `--repo` can be omitted for automatic detection

## Impact

- **Affected code**: `src/cli.rs` (complete rewrite of argument structure), `src/main.rs` (integration changes)
- **Dependencies**: No new dependencies (reuses existing `gh` CLI for repo detection)
- **Breaking changes**: Existing scripts/calls using `--owner` and `--repo` flags will need to be updated to new format
- **Tests**: All CLI parsing tests will need to be updated to reflect new argument structure
- **Documentation**: README, usage examples, and help text will need updates
