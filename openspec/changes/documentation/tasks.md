## 1. README.md Creation

- [x] 1.1 Create `README.md` in project root
- [x] 1.2 Add project title and description
- [x] 1.3 Explain tool purpose (lossless discussion archive)
- [x] 1.4 Add badges if desired (build status, version, etc.)

## 2. Prerequisites Section

- [x] 2.1 Add "Prerequisites" section to README
- [x] 2.2 Document GitHub CLI (`gh`) requirement
- [x] 2.3 Document `gh auth login` requirement
- [x] 2.4 Document Rust toolchain requirement for building from source
- [x] 2.5 Link to GitHub CLI installation instructions

## 3. Installation Section

- [x] 3.1 Add "Installation" section to README
- [x] 3.2 Document `cargo install gh-discussion-export` command
- [x] 3.3 Document building from source with `cargo build --release`
- [x] 3.4 Document where binary is located after build (`target/release/`)
- [x] 3.5 Note about pre-compiled binaries (future feature)

## 4. Usage Section

- [x] 4.1 Add "Usage" section to README
- [x] 4.2 Document basic command syntax
- [x] 4.3 Document all arguments: `--owner`, `--repo`, `--number`, `--output`
- [x] 4.4 Indicate required vs optional arguments
- [x] 4.5 Document default output behavior (`<number>-discussion.md`)

## 5. Examples Section

- [x] 5.1 Add "Examples" section to README
- [x] 5.2 Add basic example: export discussion from public repo
- [x] 5.3 Add example with custom output path
- [x] 5.4 Add example showing help command
- [x] 5.5 Ensure examples use realistic values (e.g., rust-lang/rust)
- [x] 5.6 Format examples as code blocks with shell syntax

## 6. Output Format Section

- [x] 6.1 Add "Output Format" section to README
- [x] 6.2 Describe Markdown structure
- [x] 6.3 Explain heading hierarchy (##/###/####)
- [x] 6.4 Mention lossless fidelity (verbatim content preservation)
- [x] 6.5 Note about UTF-8 encoding and LF line endings

## 7. Contributing Section

- [x] 7.1 Add "Contributing" section to README
- [x] 7.2 Link to or include CONTRIBUTING.md content
- [x] 7.3 Explain contribution process briefly

## 8. CONTRIBUTING.md Creation

- [x] 8.1 Create `CONTRIBUTING.md` file
- [x] 8.2 Add development setup instructions (clone, build, test)
- [x] 8.3 Document how to run unit tests (`cargo test`)
- [x] 8.4 Document how to run integration tests (`cargo test -- --ignored`)
- [x] 8.5 Explain git workflow (branch, commit, PR)
- [x] 8.6 Add code style guidelines (use `cargo fmt`)
- [x] 8.7 Add note about running clippy (`cargo clippy`)

## 9. License Section

- [x] 9.1 Add "License" section to README
- [x] 9.2 Specify MIT license
- [x] 9.3 Add copyright year and author if known

## 10. Review and Refinement

- [x] 10.1 Proofread README for clarity and correctness
- [x] 10.2 Test all examples to ensure they work
- [x] 10.3 Verify all links are correct (GitHub CLI, etc.)
- [x] 10.4 Ensure formatting is clean (consistent headers, code blocks)
- [x] 10.5 Check for spelling and grammar errors
