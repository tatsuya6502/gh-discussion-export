## 1. README.md Creation

- [ ] 1.1 Create `README.md` in project root
- [ ] 1.2 Add project title and description
- [ ] 1.3 Explain tool purpose (lossless discussion archive)
- [ ] 1.4 Add badges if desired (build status, version, etc.)

## 2. Prerequisites Section

- [ ] 2.1 Add "Prerequisites" section to README
- [ ] 2.2 Document GitHub CLI (`gh`) requirement
- [ ] 2.3 Document `gh auth login` requirement
- [ ] 2.4 Document Rust toolchain requirement for building from source
- [ ] 2.5 Link to GitHub CLI installation instructions

## 3. Installation Section

- [ ] 3.1 Add "Installation" section to README
- [ ] 3.2 Document `cargo install gh-discussion-export` command
- [ ] 3.3 Document building from source with `cargo build --release`
- [ ] 3.4 Document where binary is located after build (`target/release/`)
- [ ] 3.5 Note about pre-compiled binaries (future feature)

## 4. Usage Section

- [ ] 4.1 Add "Usage" section to README
- [ ] 4.2 Document basic command syntax
- [ ] 4.3 Document all arguments: `--owner`, `--repo`, `--number`, `--output`
- [ ] 4.4 Indicate required vs optional arguments
- [ ] 4.5 Document default output behavior (`<number>-discussion.md`)

## 5. Examples Section

- [ ] 5.1 Add "Examples" section to README
- [ ] 5.2 Add basic example: export discussion from public repo
- [ ] 5.3 Add example with custom output path
- [ ] 5.4 Add example showing help command
- [ ] 5.5 Ensure examples use realistic values (e.g., rust-lang/rust)
- [ ] 5.6 Format examples as code blocks with shell syntax

## 6. Output Format Section

- [ ] 6.1 Add "Output Format" section to README
- [ ] 6.2 Describe Markdown structure
- [ ] 6.3 Explain heading hierarchy (##/###/####)
- [ ] 6.4 Mention lossless fidelity (verbatim content preservation)
- [ ] 6.5 Note about UTF-8 encoding and LF line endings

## 7. Contributing Section

- [ ] 7.1 Add "Contributing" section to README
- [ ] 7.2 Link to or include CONTRIBUTING.md content
- [ ] 7.3 Explain contribution process briefly

## 8. CONTRIBUTING.md Creation

- [ ] 8.1 Create `CONTRIBUTING.md` file
- [ ] 8.2 Add development setup instructions (clone, build, test)
- [ ] 8.3 Document how to run unit tests (`cargo test`)
- [ ] 8.4 Document how to run integration tests (`cargo test -- --ignored`)
- [ ] 8.5 Explain git workflow (branch, commit, PR)
- [ ] 8.6 Add code style guidelines (use `cargo fmt`)
- [ ] 8.7 Add note about running clippy (`cargo clippy`)

## 9. License Section

- [ ] 9.1 Add "License" section to README
- [ ] 9.2 Specify MIT license
- [ ] 9.3 Add copyright year and author if known

## 10. Review and Refinement

- [ ] 10.1 Proofread README for clarity and correctness
- [ ] 10.2 Test all examples to ensure they work
- [ ] 10.3 Verify all links are correct (GitHub CLI, etc.)
- [ ] 10.4 Ensure formatting is clean (consistent headers, code blocks)
- [ ] 10.5 Check for spelling and grammar errors
