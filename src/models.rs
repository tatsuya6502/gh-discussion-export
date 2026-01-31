use chrono::{DateTime, Utc};
use serde::Deserialize;

/// Represents a GitHub user (author of comments/replies)
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Author {
    pub login: Option<String>,
}

/// Pagination information for GraphQL connections
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PageInfo {
    pub has_next_page: bool,
    pub end_cursor: Option<String>,
}

/// A reply to a comment
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Reply {
    pub id: String,
    pub database_id: i64,
    pub author: Option<Author>,
    pub created_at: DateTime<Utc>,
    pub body: String,
}

/// A comment on a discussion
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    pub id: String,
    pub database_id: i64,
    pub author: Option<Author>,
    pub created_at: DateTime<Utc>,
    pub body: String,
    pub replies: CommentReplies,
}

/// Replies connection with pagination info
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CommentReplies {
    pub nodes: Option<Vec<Option<Reply>>>,
    pub page_info: PageInfo,
}

/// A GitHub discussion
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Discussion {
    pub id: String,
    pub title: String,
    pub number: u64,
    pub url: String,
    pub created_at: DateTime<Utc>,
    pub body: String,
    pub author: Option<Author>,
    pub comments: DiscussionComments,
}

/// Comments connection with pagination info
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DiscussionComments {
    pub nodes: Option<Vec<Option<Comment>>>,
    pub page_info: PageInfo,
}

/// GraphQL error response structure
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct GraphQLError {
    pub message: String,
    pub path: Option<Vec<serde_json::Value>>,
    pub extensions: Option<serde_json::Value>,
}

/// Wrapper for GraphQL error responses
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ErrorResponse {
    pub errors: Option<Vec<GraphQLError>>,
}

/// GraphQL response wrapper
///
/// GraphQL can return both data and errors in the same response (partial success).
/// Using optional fields ensures we capture both when present.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct GraphQLResponse {
    pub data: Option<serde_json::Value>,
    pub errors: Option<Vec<GraphQLError>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_discussion_deserialization() {
        let json_data = json!({
            "id": "discussion_123",
            "title": "Test Discussion",
            "number": 123,
            "url": "https://github.com/test/repo/discussions/123",
            "createdAt": "2024-01-15T10:30:00Z",
            "body": "This is a test discussion",
            "author": {"login": "testuser"},
            "comments": {
                "nodes": [
                    {
                        "id": "comment_1",
                        "databaseId": 456,
                        "author": {"login": "testuser"},
                        "createdAt": "2024-01-15T11:00:00Z",
                        "body": "Test comment",
                        "replies": {
                            "nodes": [],
                            "pageInfo": {"hasNextPage": false, "endCursor": null}
                        }
                    }
                ],
                "pageInfo": {"hasNextPage": false, "endCursor": null}
            }
        });

        let discussion: Discussion = serde_json::from_value(json_data).unwrap();
        assert_eq!(discussion.id, "discussion_123");
        assert_eq!(discussion.title, "Test Discussion");
        assert_eq!(discussion.number, 123);
        assert_eq!(
            discussion.url,
            "https://github.com/test/repo/discussions/123"
        );
        assert_eq!(discussion.body, "This is a test discussion");
        assert!(discussion.author.is_some());
        assert_eq!(
            discussion.author.unwrap().login,
            Some("testuser".to_string())
        );
        assert!(discussion.comments.nodes.is_some());
    }

    #[test]
    fn test_comment_deserialization_with_replies() {
        let json_data = json!({
            "id": "comment_1",
            "databaseId": 456,
            "author": {"login": "testuser"},
            "createdAt": "2024-01-15T11:00:00Z",
            "body": "Test comment",
            "replies": {
                "nodes": [
                    {
                        "id": "reply_1",
                        "databaseId": 789,
                        "author": {"login": "replier"},
                        "createdAt": "2024-01-15T12:00:00Z",
                        "body": "Test reply"
                    }
                ],
                "pageInfo": {"hasNextPage": false, "endCursor": "cursor123"}
            }
        });

        let comment: Comment = serde_json::from_value(json_data).unwrap();
        assert_eq!(comment.id, "comment_1");
        assert_eq!(comment.database_id, 456);
        assert_eq!(comment.body, "Test comment");
        assert!(comment.author.is_some());
        assert_eq!(comment.author.unwrap().login, Some("testuser".to_string()));
        assert!(comment.replies.nodes.is_some());
        let replies = comment.replies.nodes.unwrap();
        assert_eq!(replies.len(), 1);
        assert!(replies[0].is_some());
    }

    #[test]
    fn test_null_author_handling() {
        let json_data = json!({
            "id": "comment_1",
            "databaseId": 456,
            "author": null,
            "createdAt": "2024-01-15T11:00:00Z",
            "body": "Test comment",
            "replies": {
                "nodes": [],
                "pageInfo": {"hasNextPage": false, "endCursor": null}
            }
        });

        let comment: Comment = serde_json::from_value(json_data).unwrap();
        assert!(comment.author.is_none());
    }

    #[test]
    fn test_page_info_deserialization() {
        let json_data = json!({
            "hasNextPage": true,
            "endCursor": "cursor_abc123"
        });

        let page_info: PageInfo = serde_json::from_value(json_data).unwrap();
        assert!(page_info.has_next_page);
        assert_eq!(page_info.end_cursor, Some("cursor_abc123".to_string()));
    }

    #[test]
    fn test_page_info_with_null_cursor() {
        let json_data = json!({
            "hasNextPage": false,
            "endCursor": null
        });

        let page_info: PageInfo = serde_json::from_value(json_data).unwrap();
        assert!(!page_info.has_next_page);
        assert!(page_info.end_cursor.is_none());
    }

    #[test]
    fn test_graphql_error_deserialization() {
        let json_data = json!({
            "message": "Field 'invalid' doesn't exist on type 'Query'",
            "path": ["repository", "discussion", "invalid"],
            "extensions": {"code": "GRAPHQL_VALIDATION_FAILED"}
        });

        let error: GraphQLError = serde_json::from_value(json_data).unwrap();
        assert!(error.message.contains("invalid"));
        assert!(error.path.is_some());
        assert!(error.extensions.is_some());
    }

    #[test]
    fn test_discussion_with_null_author() {
        let json_data = json!({
            "id": "discussion_1",
            "title": "Test",
            "number": 1,
            "url": "https://github.com/test/repo/discussions/1",
            "createdAt": "2024-01-15T10:30:00Z",
            "body": "Test body",
            "author": null,  // Deleted discussion author
            "comments": {
                "nodes": [
                    {
                        "id": "comment_1",
                        "databaseId": 456,
                        "author": null,  // Deleted user
                        "createdAt": "2024-01-15T11:00:00Z",
                        "body": "Comment from deleted user",
                        "replies": {
                            "nodes": [],
                            "pageInfo": {"hasNextPage": false, "endCursor": null}
                        }
                    }
                ],
                "pageInfo": {"hasNextPage": false, "endCursor": null}
            }
        });

        let discussion: Discussion = serde_json::from_value(json_data).unwrap();
        assert!(discussion.author.is_none()); // Discussion author is null
        let comments = discussion.comments.nodes.unwrap();
        assert!(comments[0].as_ref().unwrap().author.is_none()); // Comment author is null
    }
}
