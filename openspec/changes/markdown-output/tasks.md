## 1. Module Setup

- [ ] 1.1 Create `src/output.rs` module
- [ ] 1.2 Update `src/main.rs` to declare `output` module

## 2. Header Generation

- [ ] 2.1 Implement `generate_header(discussion, owner, repo)` function
- [ ] 2.2 Return String with title line: `# <title>`
- [ ] 2.3 Include discussion ID line: `Discussion: <owner>/<repo>#<number>`
- [ ] 2.4 Include URL line: `URL: https://github.com/<owner>/<repo>/discussions/<number>`
- [ ] 2.5 Include created at line: `Created at: <ISO8601>`
- [ ] 2.6 Include author line: `Author: <login>`
- [ ] 2.7 End with `---` separator
- [ ] 2.8 Use `\n` for all line breaks (LF)

## 3. Original Post Section

- [ ] 3.1 Implement `generate_original_post(discussion)` function
- [ ] 3.2 Add `## Original Post` heading
- [ ] 3.3 Add author line: `_author: <login> (<ISO8601>)_`
- [ ] 3.4 Add body content verbatim
- [ ] 3.5 End with `---` separator

## 4. Comments Section

- [ ] 4.1 Implement `generate_comments(discussion)` function
- [ ] 4.2 Add `## Comments` heading
- [ ] 4.3 Iterate through comments with 1-based index
- [ ] 4.4 For each comment, add `### Comment <N>` heading
- [ ] 4.5 Add author line: `_author: <login> (<ISO8601>)_`
- [ ] 4.6 Add comment body verbatim
- [ ] 4.7 For each reply, add `#### Reply <N.M>` heading
- [ ] 4.8 Add reply author line: `_author: <login> (<ISO8601>)_`
- [ ] 4.9 Add reply body verbatim

## 5. Output Formatting

- [ ] 5.1 Implement `format_discussion(discussion, owner, repo)` function
- [ ] 5.2 Concatenate header + original post + comments
- [ ] 5.3 Ensure proper spacing between sections
- [ ] 5.4 Return complete Markdown String

## 6. File Writing

- [ ] 6.1 Implement `write_output(markdown, path)` function
- [ ] 6.2 Use `std::fs::write` to write file
- [ ] 6.3 Ensure UTF-8 encoding (Rust default)
- [ ] 6.4 Handle I/O errors and return appropriate Error

## 7. Unit Tests

- [ ] 7.1 Add test for header generation with all fields
- [ ] 7.2 Add test for header with deleted author
- [ ] 7.3 Add test for original post section formatting
- [ ] 7.4 Add test for comments section with multiple comments
- [ ] 7.5 Add test for replies with proper nesting
- [ ] 7.6 Add test for heading hierarchy (##/###/####)
- [ ] 7.7 Add test for body verbatim handling (no escaping)
- [ ] 7.8 Add test for file writing with specified path
- [ ] 7.9 Add test for default output path

## 8. Integration with Models

- [ ] 8.1 Import Discussion, Comment, Reply types
- [ ] 8.2 Verify field access (title, number, url, createdAt, body, etc.)
- [ ] 8.3 Ensure compilation with model types
