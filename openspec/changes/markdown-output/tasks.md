## 1. Module Setup

- [x] 1.1 Create `src/output.rs` module
- [x] 1.2 Update `src/lib.rs` to declare `output` module with `pub(crate)` visibility

## 2. Header Generation

- [x] 2.1 Implement `generate_header(discussion, owner, repo)` function
- [x] 2.2 Return String with title line: `# <title>`
- [x] 2.3 Include discussion ID line: `Discussion: <owner>/<repo>#<number>`
- [x] 2.4 Include URL line: `URL: https://github.com/<owner>/<repo>/discussions/<number>`
- [x] 2.5 Include created at line: `Created at: <ISO8601>`
- [x] 2.6 Include author line: `Author: <login>`
- [x] 2.7 End with `---` separator
- [x] 2.8 Use `\n` for all line breaks (LF)

## 3. Original Post Section

- [x] 3.1 Implement `generate_original_post(discussion)` function
- [x] 3.2 Add `## Original Post` heading
- [x] 3.3 Add author line: `_author: <login> (<ISO8601>)_`
- [x] 3.4 Add body content verbatim (except heading escape)
- [x] 3.5 Implement heading escape: prefix `#` at line start with backslash
- [x] 3.6 Implement CRLF normalization: replace `\r\n` with `\n` in body content
- [x] 3.7 End with `---` separator

## 4. Comments Section

- [x] 4.1 Implement `generate_comments(discussion)` function
- [x] 4.2 Add `## Comments` heading
- [x] 4.3 Iterate through comments with 1-based index
- [x] 4.4 For each comment, add `### Comment <N>` heading
- [x] 4.5 Add author line: `_author: <login> (<ISO8601>)_`
- [x] 4.6 Add comment body verbatim (except heading escape)
- [x] 4.7 Apply heading escape to comment body
- [x] 4.8 Apply CRLF normalization to comment body
- [x] 4.9 For each reply, add `#### Reply <N.M>` heading
- [x] 4.10 Add reply author line: `_author: <login> (<ISO8601>)_`
- [x] 4.11 Add reply body verbatim (except heading escape)
- [x] 4.12 Apply heading escape to reply body
- [x] 4.13 Apply CRLF normalization to reply body

## 5. Output Formatting

- [x] 5.1 Implement `format_discussion(discussion, owner, repo)` function
- [x] 5.2 Concatenate header + original post + comments
- [x] 5.3 Ensure proper spacing between sections
- [x] 5.4 Return complete Markdown String

## 6. File Writing

- [x] 6.1 Implement `write_output(markdown, path)` function
- [x] 6.2 Use `std::fs::write` to write file
- [x] 6.3 Ensure UTF-8 encoding (Rust default)
- [x] 6.4 Handle I/O errors and return appropriate Error

## 7. Unit Tests

- [x] 7.1 Add test for header generation with all fields
- [x] 7.2 Add test for header with deleted author
- [x] 7.3 Add test for original post section formatting
- [x] 7.4 Add test for comments section with multiple comments
- [x] 7.5 Add test for replies with proper nesting
- [x] 7.6 Add test for heading hierarchy (##/###/####)
- [x] 7.7 Add test for body verbatim handling with heading escape (escaping `#` at line start)
- [x] 7.8 Add test for CRLF normalization (input with `\r\n` becomes `\n`)
- [x] 7.9 Add test for file writing with specified path
- [~] 7.10 Add test for default output path → **MOVED to `integration` change (task 10.4)**

## 8. Integration with Models

- [x] 8.1 Import Discussion, Comment, Reply types
- [x] 8.2 Verify field access (title, number, url, createdAt, body, etc.)
- [x] 8.3 Ensure compilation with model types
