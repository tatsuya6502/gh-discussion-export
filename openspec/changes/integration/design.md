## Context

The previous changes created modular components, but the application doesn't do anything yet. This change connects all the pieces into a working pipeline: CLI args → authentication → GraphQL client → data fetching → Markdown output.

**Constraints:**
- Must use all modules created in previous changes
- Must provide clear error messages to users
- Must exit with appropriate status codes (0 for success, non-zero for errors)

## Goals / Non-Goals

**Goals:**
- Implement complete workflow in main()
- Connect all module calls in correct sequence
- Handle errors gracefully with user-friendly messages
- Verify executable works end-to-end

**Non-Goals:**
- Adding new functionality beyond wiring existing modules
- End-to-end tests (handled by `e2e-tests` change)
- Documentation (handled by `documentation` change)

## Decisions

### Error Handling: Early return with message
Use pattern matching on Results and print error messages, then exit.

**Rationale:** CLI tools should fail fast with clear messages. Using `match` on Results and printing errors before `std::process::exit(1)` provides good UX. Alternative (panic!) would show ugly backtraces.

### Main Flow: Linear pipeline
Execute steps sequentially: parse args → get token → fetch discussion → format → write.

**Rationale:** The workflow is inherently linear. Each step depends on the previous. Async or parallel execution would add complexity without benefit for a CLI tool.

### Output Path Defaulting
Handle in main() after parsing CLI args.

**Rationale:** The default value logic (`<number>-discussion.md`) is application-specific, not a CLI parsing concern. Doing it in main keeps the CLI module simple.

## Risks / Trade-offs

| Risk | Mitigation |
|------|------------|
| Module interfaces don't match | Minor adjustments to module signatures expected during integration |
| Error messages not user-friendly | Review all error paths and ensure clear messages |

## Migration Plan

Not applicable - this is the initial working implementation.

## Open Questions

None - the integration flow is straightforward based on the pipeline design.
