/// GraphQL query to fetch discussion metadata
///
/// This query fetches only discussion metadata:
/// - Discussion ID (node ID for pagination queries)
/// - Discussion metadata (title, number, URL, created at, body, author)
///
/// Note: Comments and replies are fetched separately using pagination queries
/// (COMMENTS_QUERY and REPLIES_QUERY) to ensure complete data retrieval.
pub const DISCUSSION_QUERY: &str = r#"
query ($owner: String!, $repo: String!, $number: Int!) {
    repository(owner: $owner, name: $repo) {
        discussion(number: $number) {
            id
            title
            number
            url
            createdAt
            body
            author {
                login
            }
        }
    }
}
"#;

/// GraphQL query to fetch comments for a discussion with pagination
///
/// This query fetches:
/// - Comment nodes with id, databaseId, author, createdAt, body
/// - Replies connection with pageInfo (for determining if replies need pagination)
/// - PageInfo for comment pagination
///
/// Variables:
/// - $id: ID! - The discussion node ID
/// - $after: String - Cursor for pagination (null for first page)
pub const COMMENTS_QUERY: &str = r#"
query ($id: ID!, $after: String) {
    node(id: $id) {
        ... on Discussion {
            comments(first: 100, after: $after) {
                nodes {
                    id
                    databaseId
                    author {
                        login
                    }
                    createdAt
                    body
                    replies(first: 100) {
                        pageInfo {
                            hasNextPage
                            endCursor
                        }
                    }
                }
                pageInfo {
                    hasNextPage
                    endCursor
                }
            }
        }
    }
}
"#;

/// GraphQL query to fetch replies for a comment with pagination
///
/// This query fetches:
/// - Reply nodes with id, databaseId, author, createdAt, body
/// - PageInfo for reply pagination
///
/// Variables:
/// - $id: ID! - The comment node ID
/// - $after: String - Cursor for pagination (null for first page)
pub const REPLIES_QUERY: &str = r#"
query ($id: ID!, $after: String) {
    node(id: $id) {
        ... on DiscussionComment {
            replies(first: 100, after: $after) {
                nodes {
                    id
                    databaseId
                    author {
                        login
                    }
                    createdAt
                    body
                }
                pageInfo {
                    hasNextPage
                    endCursor
                }
            }
        }
    }
}
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_contains_discussion_fields() {
        assert!(DISCUSSION_QUERY.contains("title"));
        assert!(DISCUSSION_QUERY.contains("number"));
        assert!(DISCUSSION_QUERY.contains("url"));
        assert!(DISCUSSION_QUERY.contains("createdAt"));
        assert!(DISCUSSION_QUERY.contains("body"));
        assert!(DISCUSSION_QUERY.contains("author"));
    }

    #[test]
    fn test_query_contains_comment_fields() {
        // COMMENTS_QUERY contains comment fields
        assert!(COMMENTS_QUERY.contains("comments"));
        assert!(COMMENTS_QUERY.contains("databaseId"));
        assert!(COMMENTS_QUERY.contains("author"));
        assert!(COMMENTS_QUERY.contains("login"));
        assert!(COMMENTS_QUERY.contains("replies"));
    }

    #[test]
    fn test_query_contains_page_info() {
        // COMMENTS_QUERY and REPLIES_QUERY contain pagination info
        assert!(COMMENTS_QUERY.contains("pageInfo"));
        assert!(COMMENTS_QUERY.contains("hasNextPage"));
        assert!(COMMENTS_QUERY.contains("endCursor"));
        assert!(REPLIES_QUERY.contains("pageInfo"));
    }

    #[test]
    fn test_query_variables() {
        assert!(DISCUSSION_QUERY.contains("$owner: String!"));
        assert!(DISCUSSION_QUERY.contains("$repo: String!"));
        assert!(DISCUSSION_QUERY.contains("$number: Int!"));
    }

    #[test]
    fn test_query_syntax_basic() {
        // Basic GraphQL syntax checks
        let trimmed = DISCUSSION_QUERY.trim();
        assert!(trimmed.starts_with("query"));
        assert!(DISCUSSION_QUERY.contains("{"));
        assert!(DISCUSSION_QUERY.contains("}"));
        assert_eq!(
            DISCUSSION_QUERY.matches("{").count(),
            DISCUSSION_QUERY.matches("}").count()
        );
    }
}
