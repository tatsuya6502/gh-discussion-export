// Markdown output formatting and file writing
//
// This module generates lossless Markdown archives of GitHub Discussions,
// preserving all content verbatim except for heading escape (to preserve
// document structure).

use crate::error::{Error, Result};
use crate::models::{Comment, Discussion, Reply};
use std::fs;

/// Helper function to extract author login, returning "<deleted>" if null
fn get_author_login(author: Option<&crate::models::Author>) -> &str {
    author
        .and_then(|a| a.login.as_deref())
        .unwrap_or("<deleted>")
}

/// Escape Markdown heading syntax at the start of lines
///
/// Prefixes '#' at the start of any line with a backslash to prevent
/// it from being interpreted as a Markdown heading. This preserves
/// document structure while keeping content readable.
///
/// Preserves trailing newlines to maintain lossless fidelity.
fn escape_headings(body: &str) -> String {
    let ends_with_newline = body.ends_with('\n');
    let mut result = body
        .lines()
        .map(|line| {
            if line.starts_with('#') {
                format!("\\{}", line)
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");

    if ends_with_newline {
        result.push('\n');
    }
    result
}

/// Normalize CRLF line endings to LF
///
/// Replaces \r\n with \n, then replaces any remaining lone \r with \n
/// to ensure pure LF line endings in the output file.
fn normalize_crlf(body: &str) -> String {
    body.replace("\r\n", "\n").replace('\r', "\n")
}

/// Process body content for output
///
/// Applies heading escape and CRLF normalization while preserving
/// all other content verbatim.
fn process_body(body: &str) -> String {
    let normalized = normalize_crlf(body);
    escape_headings(&normalized)
}

/// Generate header section with discussion metadata
///
/// Returns a String containing:
/// - # <title>
/// - Discussion: <owner>/<repo>#<number>
/// - URL: https://github.com/<owner>/<repo>/discussions/<number>
/// - Created at: <ISO8601>
/// - Author: <login>
/// - ---
pub(crate) fn generate_header(discussion: &Discussion, owner: &str, repo: &str) -> String {
    let author = get_author_login(discussion.author.as_ref());
    format!(
        "# {}\nDiscussion: {}/{}#{}\nURL: {}\nCreated at: {}\nAuthor: {}\n---\n",
        discussion.title,
        owner,
        repo,
        discussion.number,
        discussion.url,
        discussion.created_at,
        author
    )
}

/// Generate original post section
///
/// Returns a String containing:
/// - ## Original Post
/// - _author: <login> (<ISO8601>)_
/// - <body content verbatim except heading escape>
/// - ---
pub(crate) fn generate_original_post(discussion: &Discussion) -> String {
    let author = get_author_login(discussion.author.as_ref());
    let body = process_body(&discussion.body);
    format!(
        "## Original Post\n_author: {} ({})_\n{}\n---\n",
        author, discussion.created_at, body
    )
}

/// Generate comments section with all comments and replies
///
/// Returns a String containing:
/// - ## Comments
/// - For each comment: ### Comment <N>
///   - _author: <login> (<ISO8601>)_
///   - <body content verbatim except heading escape>
///   - For each reply: #### Reply <N.M>
///     - _author: <login> (<ISO8601>)_
///     - <body content verbatim except heading escape>
///
/// If there are no comments, still emits the ## Comments heading.
pub(crate) fn generate_comments(discussion: &Discussion) -> String {
    let mut output = String::from("## Comments\n");

    if let Some(ref comments) = discussion.comments.nodes {
        let mut comment_num = 0;
        for comment_opt in comments.iter() {
            if let Some(comment) = comment_opt {
                comment_num += 1;
                let author = get_author_login(comment.author.as_ref());
                let body = process_body(&comment.body);

                output.push_str(&format!(
                    "### Comment {}\n_author: {} ({})_\n{}\n",
                    comment_num, author, comment.created_at, body
                ));

                // Add replies if present
                if let Some(ref replies) = comment.replies.nodes {
                    let mut reply_num = 0;
                    for reply_opt in replies.iter() {
                        if let Some(reply) = reply_opt {
                            reply_num += 1;
                            let reply_author = get_author_login(reply.author.as_ref());
                            let reply_body = process_body(&reply.body);

                            output.push_str(&format!(
                                "#### Reply {}.{}\n_author: {} ({})_\n{}\n",
                                comment_num, reply_num, reply_author, reply.created_at, reply_body
                            ));
                        }
                    }
                }
            }
        }
    }

    output
}

/// Format complete discussion as Markdown
///
/// Concatenates header, original post, and comments sections with
/// proper spacing between sections.
///
/// Returns complete Markdown String ready for file output.
pub(crate) fn format_discussion(discussion: &Discussion, owner: &str, repo: &str) -> String {
    let header = generate_header(discussion, owner, repo);
    let original_post = generate_original_post(discussion);
    let comments = generate_comments(discussion);

    format!("{}\n{}\n{}", header, original_post, comments)
}

/// Write Markdown content to file
///
/// Uses std::fs::write to create file with UTF-8 encoding and LF line endings.
/// Returns Error if I/O operation fails.
pub(crate) fn write_output(markdown: &str, path: &str) -> Result<()> {
    fs::write(path, markdown).map_err(Error::Io)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Author;
    use chrono::{DateTime, Utc};

    fn make_discussion() -> Discussion {
        Discussion {
            id: "test_id".to_string(),
            title: "Test Discussion".to_string(),
            number: 123,
            url: "https://github.com/owner/repo/discussions/123".to_string(),
            created_at: DateTime::parse_from_rfc3339("2024-01-15T10:30:00Z")
                .unwrap()
                .with_timezone(&Utc),
            body: "This is the original post body.".to_string(),
            author: Some(Author {
                login: Some("testuser".to_string()),
            }),
            comments: Default::default(),
        }
    }

    fn make_comment(login: Option<&str>, body: &str) -> Comment {
        Comment {
            id: "comment_id".to_string(),
            database_id: 1,
            author: login.map(|l| Author {
                login: Some(l.to_string()),
            }),
            created_at: DateTime::parse_from_rfc3339("2024-01-15T11:00:00Z")
                .unwrap()
                .with_timezone(&Utc),
            body: body.to_string(),
            replies: crate::models::CommentReplies {
                nodes: Some(vec![]),
                page_info: Default::default(),
            },
        }
    }

    fn make_reply(login: Option<&str>, body: &str) -> Reply {
        Reply {
            id: "reply_id".to_string(),
            database_id: 2,
            author: login.map(|l| Author {
                login: Some(l.to_string()),
            }),
            created_at: DateTime::parse_from_rfc3339("2024-01-15T12:00:00Z")
                .unwrap()
                .with_timezone(&Utc),
            body: body.to_string(),
        }
    }

    #[test]
    fn test_generate_header_with_all_fields() {
        let discussion = make_discussion();
        let header = generate_header(&discussion, "owner", "repo");

        assert!(header.contains("# Test Discussion"));
        assert!(header.contains("Discussion: owner/repo#123"));
        assert!(header.contains("URL: https://github.com/owner/repo/discussions/123"));
        assert!(header.contains("Created at: 2024-01-15 10:30:00 UTC"));
        assert!(header.contains("Author: testuser"));
        assert!(header.ends_with("---\n"));
    }

    #[test]
    fn test_generate_header_with_deleted_author() {
        let mut discussion = make_discussion();
        discussion.author = None;
        let header = generate_header(&discussion, "owner", "repo");

        assert!(header.contains("Author: <deleted>"));
    }

    #[test]
    fn test_generate_original_post() {
        let discussion = make_discussion();
        let post = generate_original_post(&discussion);

        assert!(post.contains("## Original Post"));
        assert!(post.contains("_author: testuser (2024-01-15 10:30:00 UTC)_"));
        assert!(post.contains("This is the original post body."));
        assert!(post.ends_with("---\n"));
    }

    #[test]
    fn test_generate_original_post_with_deleted_author() {
        let mut discussion = make_discussion();
        discussion.author = None;
        let post = generate_original_post(&discussion);

        assert!(post.contains("_author: <deleted>"));
        assert!(post.contains("This is the original post body."));
    }

    #[test]
    fn test_generate_comments_with_multiple_comments() {
        let mut discussion = make_discussion();
        let comment1 = make_comment(Some("user1"), "First comment");
        let comment2 = make_comment(Some("user2"), "Second comment");

        discussion.comments.nodes = Some(vec![Some(comment1), Some(comment2)]);
        let comments = generate_comments(&discussion);

        assert!(comments.contains("## Comments"));
        assert!(comments.contains("### Comment 1"));
        assert!(comments.contains("First comment"));
        assert!(comments.contains("### Comment 2"));
        assert!(comments.contains("Second comment"));
    }

    #[test]
    fn test_generate_comments_with_no_comments() {
        let mut discussion = make_discussion();
        discussion.comments.nodes = Some(vec![]);
        let comments = generate_comments(&discussion);

        assert!(comments.contains("## Comments"));
        // Should not contain any comment or reply headings
        assert!(!comments.contains("### Comment"));
        assert!(!comments.contains("#### Reply"));
    }

    #[test]
    fn test_heading_escape() {
        let input = "## This is a heading\nRegular text\n### Another heading";
        let escaped = escape_headings(input);

        assert_eq!(
            escaped,
            "\\## This is a heading\nRegular text\n\\### Another heading"
        );
    }

    #[test]
    fn test_escape_headings_preserves_trailing_newline() {
        let input = "# Heading\nContent\n";
        let escaped = escape_headings(input);

        assert!(
            escaped.ends_with('\n'),
            "trailing newline should be preserved"
        );
        assert_eq!(escaped, "\\# Heading\nContent\n");
    }

    #[test]
    fn test_crlf_normalization() {
        let input = "Line 1\r\nLine 2\r\nLine 3";
        let normalized = normalize_crlf(input);

        assert_eq!(normalized, "Line 1\nLine 2\nLine 3");
        assert!(!normalized.contains("\r\n"));
    }

    #[test]
    fn test_lone_cr_normalization() {
        let input = "Line 1\rLine 2\r\nLine 3\rLine 4";
        let normalized = normalize_crlf(input);

        assert_eq!(normalized, "Line 1\nLine 2\nLine 3\nLine 4");
        assert!(!normalized.contains('\r'));
    }

    #[test]
    fn test_process_body_verbatim_with_heading_escape() {
        let input = "# Heading in body\nRegular text\n## Another heading";
        let processed = process_body(input);

        // Should escape headings but preserve everything else verbatim
        assert!(processed.contains("\\# Heading in body"));
        assert!(processed.contains("\\## Another heading"));
        assert!(processed.contains("Regular text"));
    }

    #[test]
    fn test_process_body_crlf_normalization() {
        let input = "Line 1\r\nLine 2\r\nLine 3";
        let processed = process_body(input);

        assert!(!processed.contains("\r\n"));
        assert!(processed.contains("Line 1\nLine 2\nLine 3"));
    }

    #[test]
    fn test_format_discussion_complete_output() {
        let discussion = make_discussion();
        let formatted = format_discussion(&discussion, "owner", "repo");

        // Check all sections are present
        assert!(formatted.contains("# Test Discussion"));
        assert!(formatted.contains("## Original Post"));
        assert!(formatted.contains("## Comments"));
        assert!(formatted.contains("---"));
    }

    #[test]
    fn test_heading_hierarchy() {
        let mut discussion = make_discussion();
        let mut comment = make_comment(Some("user1"), "Comment body");
        let reply = Reply {
            id: "reply_id".to_string(),
            database_id: 2,
            author: Some(Author {
                login: Some("replier".to_string()),
            }),
            created_at: DateTime::parse_from_rfc3339("2024-01-15T12:00:00Z")
                .unwrap()
                .with_timezone(&Utc),
            body: "Reply body".to_string(),
        };

        comment.replies.nodes = Some(vec![Some(reply)]);
        discussion.comments.nodes = Some(vec![Some(comment)]);

        let formatted = format_discussion(&discussion, "owner", "repo");

        // Check heading levels
        assert!(formatted.contains("# Test Discussion")); // Level 1
        assert!(formatted.contains("## Original Post")); // Level 2
        assert!(formatted.contains("## Comments")); // Level 2
        assert!(formatted.contains("### Comment 1")); // Level 3
        assert!(formatted.contains("#### Reply 1.1")); // Level 4
    }

    #[test]
    fn test_write_output_creates_file() {
        let temp_dir = std::env::temp_dir();
        let file_path = temp_dir.join("test_output.md");
        let path_str = file_path.to_str().unwrap();

        let markdown = "# Test\n\nContent here";
        let result = write_output(markdown, path_str);

        assert!(result.is_ok());
        assert!(file_path.exists());

        // Clean up
        fs::remove_file(&file_path).unwrap();
    }

    #[test]
    fn test_write_output_handles_io_error() {
        // Use an invalid path (directory that doesn't exist)
        let result = write_output("test", "/nonexistent/dir/file.md");

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::Io(_)));
    }

    #[test]
    fn test_replies_with_proper_nesting() {
        let mut discussion = make_discussion();
        let mut comment1 = make_comment(Some("user1"), "Comment 1");
        let mut comment2 = make_comment(Some("user2"), "Comment 2");

        let reply1_1 = Reply {
            id: "reply_1_1".to_string(),
            database_id: 11,
            author: Some(Author {
                login: Some("replier1".to_string()),
            }),
            created_at: DateTime::parse_from_rfc3339("2024-01-15T12:00:00Z")
                .unwrap()
                .with_timezone(&Utc),
            body: "Reply 1.1".to_string(),
        };

        let reply1_2 = Reply {
            id: "reply_1_2".to_string(),
            database_id: 12,
            author: Some(Author {
                login: Some("replier2".to_string()),
            }),
            created_at: DateTime::parse_from_rfc3339("2024-01-15T12:30:00Z")
                .unwrap()
                .with_timezone(&Utc),
            body: "Reply 1.2".to_string(),
        };

        let reply2_1 = Reply {
            id: "reply_2_1".to_string(),
            database_id: 21,
            author: Some(Author {
                login: Some("replier3".to_string()),
            }),
            created_at: DateTime::parse_from_rfc3339("2024-01-15T13:00:00Z")
                .unwrap()
                .with_timezone(&Utc),
            body: "Reply 2.1".to_string(),
        };

        comment1.replies.nodes = Some(vec![Some(reply1_1), Some(reply1_2)]);
        comment2.replies.nodes = Some(vec![Some(reply2_1)]);
        discussion.comments.nodes = Some(vec![Some(comment1), Some(comment2)]);

        let formatted = format_discussion(&discussion, "owner", "repo");

        // Check proper reply numbering
        assert!(formatted.contains("### Comment 1"));
        assert!(formatted.contains("#### Reply 1.1"));
        assert!(formatted.contains("Reply 1.1"));
        assert!(formatted.contains("#### Reply 1.2"));
        assert!(formatted.contains("Reply 1.2"));
        assert!(formatted.contains("### Comment 2"));
        assert!(formatted.contains("#### Reply 2.1"));
        assert!(formatted.contains("Reply 2.1"));
    }

    #[test]
    fn test_comment_with_no_replies() {
        let mut discussion = make_discussion();
        let comment = make_comment(Some("user1"), "Comment without replies");

        discussion.comments.nodes = Some(vec![Some(comment)]);
        let formatted = format_discussion(&discussion, "owner", "repo");

        assert!(formatted.contains("### Comment 1"));
        assert!(formatted.contains("Comment without replies"));
        // Should not contain reply headings
        assert!(!formatted.contains("#### Reply"));
    }

    #[test]
    fn test_deleted_author_in_comment() {
        let mut discussion = make_discussion();
        let comment = make_comment(None, "Comment from deleted user");

        discussion.comments.nodes = Some(vec![Some(comment)]);
        let formatted = format_discussion(&discussion, "owner", "repo");

        assert!(formatted.contains("_author: <deleted>"));
        assert!(formatted.contains("Comment from deleted user"));
    }

    #[test]
    fn test_deleted_author_in_reply() {
        let mut discussion = make_discussion();
        let mut comment = make_comment(Some("user1"), "Comment");

        let reply = Reply {
            id: "reply_id".to_string(),
            database_id: 2,
            author: None, // Deleted user
            created_at: DateTime::parse_from_rfc3339("2024-01-15T12:00:00Z")
                .unwrap()
                .with_timezone(&Utc),
            body: "Reply from deleted user".to_string(),
        };

        comment.replies.nodes = Some(vec![Some(reply)]);
        discussion.comments.nodes = Some(vec![Some(comment)]);

        let formatted = format_discussion(&discussion, "owner", "repo");

        assert!(formatted.contains("#### Reply 1.1"));
        assert!(formatted.contains("_author: <deleted>"));
        assert!(formatted.contains("Reply from deleted user"));
    }

    #[test]
    fn test_comment_numbering_with_none_entries() {
        let mut discussion = make_discussion();
        let comment1 = make_comment(Some("user1"), "Comment 1");
        let comment2 = make_comment(Some("user2"), "Comment 2");
        let comment3 = make_comment(Some("user3"), "Comment 3");

        // Create comments with None entries interspersed
        discussion.comments.nodes = Some(vec![
            Some(comment1),
            None, // Deleted/missing comment
            Some(comment2),
            None, // Another missing comment
            Some(comment3),
        ]);

        let formatted = format_discussion(&discussion, "owner", "repo");

        // Should number sequentially: Comment 1, Comment 2, Comment 3
        assert!(formatted.contains("### Comment 1"));
        assert!(formatted.contains("Comment 1"));
        assert!(formatted.contains("### Comment 2"));
        assert!(formatted.contains("Comment 2"));
        assert!(formatted.contains("### Comment 3"));
        assert!(formatted.contains("Comment 3"));

        // Should not contain Comment 4 or Comment 5 (only 3 actual comments)
        assert!(!formatted.contains("### Comment 4"));
        assert!(!formatted.contains("### Comment 5"));
    }

    #[test]
    fn test_reply_numbering_with_none_entries() {
        let mut discussion = make_discussion();
        let mut comment1 = make_comment(Some("user1"), "Comment 1");

        let reply1 = Reply {
            id: "reply_1".to_string(),
            database_id: 1,
            author: Some(Author {
                login: Some("replier1".to_string()),
            }),
            created_at: DateTime::parse_from_rfc3339("2024-01-15T12:00:00Z")
                .unwrap()
                .with_timezone(&Utc),
            body: "Reply 1".to_string(),
        };

        let reply2 = Reply {
            id: "reply_2".to_string(),
            database_id: 2,
            author: Some(Author {
                login: Some("replier2".to_string()),
            }),
            created_at: DateTime::parse_from_rfc3339("2024-01-15T12:30:00Z")
                .unwrap()
                .with_timezone(&Utc),
            body: "Reply 2".to_string(),
        };

        // Create replies with None entries interspersed
        comment1.replies.nodes = Some(vec![
            Some(reply1),
            None, // Deleted/missing reply
            Some(reply2),
        ]);

        discussion.comments.nodes = Some(vec![Some(comment1)]);

        let formatted = format_discussion(&discussion, "owner", "repo");

        // Should number sequentially: Reply 1.1, Reply 1.2
        assert!(formatted.contains("#### Reply 1.1"));
        assert!(formatted.contains("Reply 1"));
        assert!(formatted.contains("#### Reply 1.2"));
        assert!(formatted.contains("Reply 2"));

        // Should not contain Reply 1.3 (only 2 actual replies)
        assert!(!formatted.contains("#### Reply 1.3"));
    }
}
