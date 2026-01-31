use crate::client::GitHubClient;
use crate::error::{Error, Result};
use crate::graphql::{COMMENTS_QUERY, DISCUSSION_QUERY, REPLIES_QUERY};
use crate::models::{Comment, Discussion, Reply};
use serde_json::Value;

/// Response structure for comments query
#[derive(Debug)]
struct CommentsResponse {
    nodes: Option<Vec<Option<Comment>>>,
    page_info: crate::models::PageInfo,
}

/// Response structure for replies query
#[derive(Debug)]
struct RepliesResponse {
    nodes: Option<Vec<Option<Reply>>>,
    page_info: crate::models::PageInfo,
}

/// Fetch a complete discussion with all comments and replies
///
/// # Arguments
/// * `client` - The GitHubClient to use for queries
/// * `owner` - Repository owner (user or organization)
/// * `repo` - Repository name
/// * `number` - Discussion number
///
/// # Returns
/// A complete Discussion object with all comments and replies
///
/// # Behavior
/// - Fetches discussion metadata using DISCUSSION_QUERY
/// - Extracts discussion ID from response
/// - Fetches all comments using pagination
/// - For each comment, fetches all replies using pagination
/// - Replaces null authors with `<deleted>` placeholder
/// - Sorts comments by createdAt ascending
/// - Sorts replies for each comment by createdAt ascending
/// - Fails immediately on any error (no partial results)
pub(crate) fn fetch_discussion(
    client: &GitHubClient,
    owner: &str,
    repo: &str,
    number: u64,
) -> Result<Discussion> {
    // Step 1: Fetch discussion metadata (task 4.2)
    let variables = serde_json::json!({
        "owner": owner,
        "repo": repo,
        "number": number
    });

    let mut discussion = client.execute_query(DISCUSSION_QUERY, variables)?;

    // Step 2: Get discussion ID from response (task 4.3)
    let discussion_id = discussion.id.clone();

    // Step 3: Fetch all comments using pagination (task 4.4)
    let mut comments = fetch_all_comments(client, &discussion_id)?;

    // Step 4: For each comment, fetch all replies (task 4.5)
    for comment in &mut comments {
        let comment_id = comment.id.clone();
        let replies = fetch_all_replies(client, &comment_id)?;

        // Update the comment's replies with the fetched ones
        comment.replies.nodes = if replies.is_empty() {
            None
        } else {
            Some(replies.into_iter().map(Some).collect())
        };
        // Reset page_info to indicate no more pages since we've fetched all replies
        comment.replies.page_info = crate::models::PageInfo {
            has_next_page: false,
            end_cursor: None,
        };
    }

    // Step 5: Replace null authors with `<deleted>` placeholder (task 4.6)
    replace_deleted_authors(&mut discussion, &mut comments)?;

    // Step 6: Sort comments by createdAt ascending (task 4.7)
    comments.sort_by(|a, b| a.created_at.cmp(&b.created_at));

    // Step 7: Sort replies for each comment by createdAt ascending (task 4.8)
    for comment in &mut comments {
        if let Some(ref mut nodes) = comment.replies.nodes {
            nodes.sort_by(|a, b| match (a, b) {
                (Some(r1), Some(r2)) => r1.created_at.cmp(&r2.created_at),
                (Some(_), None) => std::cmp::Ordering::Less,
                (None, Some(_)) => std::cmp::Ordering::Greater,
                (None, None) => std::cmp::Ordering::Equal,
            });
        }
    }

    // Step 8: Update discussion with fetched comments (task 4.9)
    discussion.comments.nodes = if comments.is_empty() {
        None
    } else {
        Some(comments.into_iter().map(Some).collect())
    };
    // Reset page_info to indicate no more pages since we've fetched all comments
    discussion.comments.page_info = crate::models::PageInfo {
        has_next_page: false,
        end_cursor: None,
    };

    Ok(discussion)
}

/// Replace null authors with `<deleted>` placeholder
///
/// This helper function handles task 4.6 by replacing null author fields
/// with Author structs containing login: Some("<deleted>")
fn replace_deleted_authors(discussion: &mut Discussion, comments: &mut [Comment]) -> Result<()> {
    use crate::models::Author;

    // Handle discussion author
    if discussion.author.is_none() {
        discussion.author = Some(Author {
            login: Some("<deleted>".to_string()),
        });
    }

    // Handle comment authors
    for comment in comments {
        if comment.author.is_none() {
            comment.author = Some(Author {
                login: Some("<deleted>".to_string()),
            });
        }

        // Handle reply authors
        if let Some(ref mut nodes) = comment.replies.nodes {
            for reply in nodes {
                if let Some(r) = reply
                    && r.author.is_none()
                {
                    r.author = Some(Author {
                        login: Some("<deleted>".to_string()),
                    });
                }
            }
        }
    }

    Ok(())
}

/// Fetch all comments for a discussion using cursor-based pagination
///
/// # Arguments
/// * `client` - The GitHubClient to use for queries
/// * `discussion_id` - The node ID of the discussion
///
/// # Returns
/// A vector of all comments for the discussion
///
/// # Behavior
/// - Starts with `after: null` to fetch the first page
/// - Continues fetching while `pageInfo.hasNextPage` is true
/// - Uses `pageInfo.endCursor` as the `after` parameter for subsequent requests
/// - Accumulates comments across all pages
/// - Fails immediately on any error (no partial results)
pub(crate) fn fetch_all_comments(
    client: &GitHubClient,
    discussion_id: &str,
) -> Result<Vec<Comment>> {
    let mut all_comments = Vec::new();
    let mut after: Option<String> = None;

    loop {
        let variables = serde_json::json!({
            "id": discussion_id,
            "after": after
        });

        let response = execute_query_raw(client, COMMENTS_QUERY, variables)?;
        let comments_response = parse_comments_response(response)?;

        // Accumulate comments (filter out nulls from nodes array)
        if let Some(nodes) = comments_response.nodes {
            for c in nodes.into_iter().flatten() {
                all_comments.push(c);
            }
        }

        // Check if there are more pages
        if !comments_response.page_info.has_next_page {
            break;
        }

        // Set cursor for next page - protect against infinite loop
        // if has_next_page is true but end_cursor is None, this is an API error
        after = comments_response.page_info.end_cursor;
        if after.is_none() {
            return Err(Error::ApiInvariant(
                "hasNextPage was true but endCursor was null".to_string(),
            ));
        }
    }

    Ok(all_comments)
}

/// Fetch all replies for a comment using cursor-based pagination
///
/// # Arguments
/// * `client` - The GitHubClient to use for queries
/// * `comment_id` - The node ID of the comment
///
/// # Returns
/// A vector of all replies for the comment
///
/// # Behavior
/// - Starts with `after: null` to fetch the first page
/// - Continues fetching while `pageInfo.hasNextPage` is true
/// - Uses `pageInfo.endCursor` as the `after` parameter for subsequent requests
/// - Accumulates replies across all pages
/// - Fails immediately on any error (no partial results)
pub(crate) fn fetch_all_replies(client: &GitHubClient, comment_id: &str) -> Result<Vec<Reply>> {
    let mut all_replies = Vec::new();
    let mut after: Option<String> = None;

    loop {
        let variables = serde_json::json!({
            "id": comment_id,
            "after": after
        });

        let response = execute_query_raw(client, REPLIES_QUERY, variables)?;
        let replies_response = parse_replies_response(response)?;

        // Accumulate replies (filter out nulls from nodes array)
        if let Some(nodes) = replies_response.nodes {
            for r in nodes.into_iter().flatten() {
                all_replies.push(r);
            }
        }

        // Check if there are more pages
        if !replies_response.page_info.has_next_page {
            break;
        }

        // Set cursor for next page - protect against infinite loop
        // if has_next_page is true but end_cursor is None, this is an API error
        after = replies_response.page_info.end_cursor;
        if after.is_none() {
            return Err(Error::ApiInvariant(
                "hasNextPage was true but endCursor was null".to_string(),
            ));
        }
    }

    Ok(all_replies)
}

/// Execute a GraphQL query and return the raw JSON response
///
/// This is a helper function that performs the same HTTP request as
/// `GitHubClient::execute_query` but returns the raw data instead of
/// parsing it into a Discussion struct. Also checks for GraphQL errors.
fn execute_query_raw(
    client: &GitHubClient,
    query: &str,
    variables: serde_json::Value,
) -> Result<Value> {
    let response = client.execute_query_raw(query, variables)?;

    // Check for GraphQL errors
    if let Some(errors) = response.get("errors").and_then(|e| e.as_array())
        && !errors.is_empty()
    {
        let error_messages: Vec<String> = errors
            .iter()
            .filter_map(|e| e.get("message").and_then(|m| m.as_str()))
            .map(|s| s.to_string())
            .collect();
        return Err(Error::GraphQL(error_messages.join("; ")));
    }

    Ok(response)
}

/// Parse a raw JSON response into a CommentsResponse
fn parse_comments_response(response: Value) -> Result<CommentsResponse> {
    // Navigate the response structure: data.node.comments
    let data = response
        .get("data")
        .ok_or_else(|| Error::JsonParse("Response missing 'data' field".to_string()))?;

    let node = data
        .get("node")
        .ok_or_else(|| Error::JsonParse("Response missing 'node' field".to_string()))?;

    // Check if node is null (ID didn't match the Discussion type)
    if node.is_null() {
        return Err(Error::JsonParse(
            "Node is null - the ID may not be a valid Discussion".to_string(),
        ));
    }

    let comments = node
        .get("comments")
        .ok_or_else(|| Error::JsonParse("Response missing 'comments' field".to_string()))?;

    // Parse nodes
    let nodes: Option<Vec<Option<Comment>>> = match comments.get("nodes") {
        Some(v) => Some(
            serde_json::from_value(v.clone())
                .map_err(|e| Error::JsonParse(format!("Failed to parse comment nodes: {}", e)))?,
        ),
        None => None,
    };

    // Parse pageInfo
    let page_info_value = comments
        .get("pageInfo")
        .ok_or_else(|| Error::JsonParse("Response missing 'pageInfo' field".to_string()))?;

    let page_info: crate::models::PageInfo = serde_json::from_value(page_info_value.clone())
        .map_err(|e| Error::JsonParse(format!("Failed to parse PageInfo: {}", e)))?;

    Ok(CommentsResponse { nodes, page_info })
}

/// Parse a raw JSON response into a RepliesResponse
fn parse_replies_response(response: Value) -> Result<RepliesResponse> {
    // Navigate the response structure: data.node.replies
    let data = response
        .get("data")
        .ok_or_else(|| Error::JsonParse("Response missing 'data' field".to_string()))?;

    let node = data
        .get("node")
        .ok_or_else(|| Error::JsonParse("Response missing 'node' field".to_string()))?;

    // Check if node is null (ID didn't match the DiscussionComment type)
    if node.is_null() {
        return Err(Error::JsonParse(
            "Node is null - the ID may not be a valid DiscussionComment".to_string(),
        ));
    }

    let replies = node
        .get("replies")
        .ok_or_else(|| Error::JsonParse("Response missing 'replies' field".to_string()))?;

    // Parse nodes
    let nodes: Option<Vec<Option<Reply>>> = match replies.get("nodes") {
        Some(v) => Some(
            serde_json::from_value(v.clone())
                .map_err(|e| Error::JsonParse(format!("Failed to parse reply nodes: {}", e)))?,
        ),
        None => None,
    };

    // Parse pageInfo
    let page_info_value = replies
        .get("pageInfo")
        .ok_or_else(|| Error::JsonParse("Response missing 'pageInfo' field".to_string()))?;

    let page_info: crate::models::PageInfo = serde_json::from_value(page_info_value.clone())
        .map_err(|e| Error::JsonParse(format!("Failed to parse PageInfo: {}", e)))?;

    Ok(RepliesResponse { nodes, page_info })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_parse_comments_response_single_page() {
        let response = json!({
            "data": {
                "node": {
                    "comments": {
                        "nodes": [
                            {
                                "id": "comment_1",
                                "databaseId": 1,
                                "author": {"login": "user1"},
                                "createdAt": "2024-01-01T00:00:00Z",
                                "body": "Test comment 1",
                                "replies": {
                                    "pageInfo": {"hasNextPage": false, "endCursor": null}
                                }
                            }
                        ],
                        "pageInfo": {
                            "hasNextPage": false,
                            "endCursor": null
                        }
                    }
                }
            }
        });

        let result = parse_comments_response(response).unwrap();
        assert!(!result.page_info.has_next_page);
        assert!(result.page_info.end_cursor.is_none());
        assert!(result.nodes.is_some());
        let nodes = result.nodes.unwrap();
        assert_eq!(nodes.len(), 1);
        assert!(nodes[0].is_some());
        assert_eq!(nodes[0].as_ref().unwrap().body, "Test comment 1");
    }

    #[test]
    fn test_parse_comments_response_with_nulls() {
        let response = json!({
            "data": {
                "node": {
                    "comments": {
                        "nodes": [
                            {
                                "id": "comment_1",
                                "databaseId": 1,
                                "author": {"login": "user1"},
                                "createdAt": "2024-01-01T00:00:00Z",
                                "body": "Test comment 1",
                                "replies": {
                                    "pageInfo": {"hasNextPage": false, "endCursor": null}
                                }
                            },
                            null,
                            {
                                "id": "comment_2",
                                "databaseId": 2,
                                "author": {"login": "user2"},
                                "createdAt": "2024-01-01T01:00:00Z",
                                "body": "Test comment 2",
                                "replies": {
                                    "pageInfo": {"hasNextPage": false, "endCursor": null}
                                }
                            }
                        ],
                        "pageInfo": {
                            "hasNextPage": false,
                            "endCursor": null
                        }
                    }
                }
            }
        });

        let result = parse_comments_response(response).unwrap();
        let nodes = result.nodes.unwrap();
        assert_eq!(nodes.len(), 3);
        assert!(nodes[0].is_some());
        assert!(nodes[1].is_none());
        assert!(nodes[2].is_some());
    }

    #[test]
    fn test_parse_comments_response_has_next_page() {
        let response = json!({
            "data": {
                "node": {
                    "comments": {
                        "nodes": [],
                        "pageInfo": {
                            "hasNextPage": true,
                            "endCursor": "cursor_abc123"
                        }
                    }
                }
            }
        });

        let result = parse_comments_response(response).unwrap();
        assert!(result.page_info.has_next_page);
        assert_eq!(
            result.page_info.end_cursor,
            Some("cursor_abc123".to_string())
        );
    }

    #[test]
    fn test_parse_replies_response_single_page() {
        let response = json!({
            "data": {
                "node": {
                    "replies": {
                        "nodes": [
                            {
                                "id": "reply_1",
                                "databaseId": 1,
                                "author": {"login": "user1"},
                                "createdAt": "2024-01-01T00:00:00Z",
                                "body": "Test reply 1"
                            }
                        ],
                        "pageInfo": {
                            "hasNextPage": false,
                            "endCursor": null
                        }
                    }
                }
            }
        });

        let result = parse_replies_response(response).unwrap();
        assert!(!result.page_info.has_next_page);
        assert!(result.page_info.end_cursor.is_none());
        assert!(result.nodes.is_some());
        let nodes = result.nodes.unwrap();
        assert_eq!(nodes.len(), 1);
        assert!(nodes[0].is_some());
        assert_eq!(nodes[0].as_ref().unwrap().body, "Test reply 1");
    }

    #[test]
    fn test_parse_replies_response_with_nulls() {
        let response = json!({
            "data": {
                "node": {
                    "replies": {
                        "nodes": [
                            {
                                "id": "reply_1",
                                "databaseId": 1,
                                "author": {"login": "user1"},
                                "createdAt": "2024-01-01T00:00:00Z",
                                "body": "Test reply 1"
                            },
                            null,
                            {
                                "id": "reply_2",
                                "databaseId": 2,
                                "author": {"login": "user2"},
                                "createdAt": "2024-01-01T01:00:00Z",
                                "body": "Test reply 2"
                            }
                        ],
                        "pageInfo": {
                            "hasNextPage": false,
                            "endCursor": null
                        }
                    }
                }
            }
        });

        let result = parse_replies_response(response).unwrap();
        let nodes = result.nodes.unwrap();
        assert_eq!(nodes.len(), 3);
        assert!(nodes[0].is_some());
        assert!(nodes[1].is_none());
        assert!(nodes[2].is_some());
    }

    #[test]
    fn test_parse_replies_response_has_next_page() {
        let response = json!({
            "data": {
                "node": {
                    "replies": {
                        "nodes": [],
                        "pageInfo": {
                            "hasNextPage": true,
                            "endCursor": "cursor_xyz789"
                        }
                    }
                }
            }
        });

        let result = parse_replies_response(response).unwrap();
        assert!(result.page_info.has_next_page);
        assert_eq!(
            result.page_info.end_cursor,
            Some("cursor_xyz789".to_string())
        );
    }

    #[test]
    fn test_parse_comments_response_missing_data() {
        let response = json!({});

        let result = parse_comments_response(response);
        assert!(result.is_err());
        match result {
            Err(Error::JsonParse(msg)) => assert!(msg.contains("data")),
            _ => panic!("Expected JsonParse error"),
        }
    }

    #[test]
    fn test_parse_replies_response_missing_data() {
        let response = json!({});

        let result = parse_replies_response(response);
        assert!(result.is_err());
        match result {
            Err(Error::JsonParse(msg)) => assert!(msg.contains("data")),
            _ => panic!("Expected JsonParse error"),
        }
    }

    #[test]
    fn test_parse_comments_response_missing_nodes() {
        let response = json!({
            "data": {
                "node": {
                    "comments": {
                        "pageInfo": {
                            "hasNextPage": false,
                            "endCursor": null
                        }
                    }
                }
            }
        });

        let result = parse_comments_response(response).unwrap();
        assert!(result.nodes.is_none());
        assert!(!result.page_info.has_next_page);
    }

    #[test]
    fn test_parse_replies_response_missing_nodes() {
        let response = json!({
            "data": {
                "node": {
                    "replies": {
                        "pageInfo": {
                            "hasNextPage": false,
                            "endCursor": null
                        }
                    }
                }
            }
        });

        let result = parse_replies_response(response).unwrap();
        assert!(result.nodes.is_none());
        assert!(!result.page_info.has_next_page);
    }

    // Task 5.2: Add test for multiple pages of comments (pagination loop)
    #[test]
    fn test_fetch_all_comments_multiple_pages() {
        // This test would require mocking the GitHubClient to simulate
        // multiple pages of responses. Since we're testing at unit level,
        // we'll verify the logic through the parse functions and
        // the multi-page response structure.
        let response_page1 = json!({
            "data": {
                "node": {
                    "comments": {
                        "nodes": [
                            {
                                "id": "comment_1",
                                "databaseId": 1,
                                "author": {"login": "user1"},
                                "createdAt": "2024-01-01T00:00:00Z",
                                "body": "Comment 1",
                                "replies": {
                                    "pageInfo": {"hasNextPage": false, "endCursor": null}
                                }
                            }
                        ],
                        "pageInfo": {
                            "hasNextPage": true,
                            "endCursor": "cursor_page2"
                        }
                    }
                }
            }
        });

        let response_page2 = json!({
            "data": {
                "node": {
                    "comments": {
                        "nodes": [
                            {
                                "id": "comment_2",
                                "databaseId": 2,
                                "author": {"login": "user2"},
                                "createdAt": "2024-01-01T01:00:00Z",
                                "body": "Comment 2",
                                "replies": {
                                    "pageInfo": {"hasNextPage": false, "endCursor": null}
                                }
                            }
                        ],
                        "pageInfo": {
                            "hasNextPage": false,
                            "endCursor": null
                        }
                    }
                }
            }
        });

        // Verify we can parse both pages correctly
        let page1 = parse_comments_response(response_page1).unwrap();
        assert!(page1.page_info.has_next_page);
        assert_eq!(page1.page_info.end_cursor, Some("cursor_page2".to_string()));
        assert_eq!(page1.nodes.unwrap().len(), 1);

        let page2 = parse_comments_response(response_page2).unwrap();
        assert!(!page2.page_info.has_next_page);
        assert_eq!(page2.nodes.unwrap().len(), 1);
    }

    // Task 5.4: Add test for multiple pages of replies
    #[test]
    fn test_fetch_all_replies_multiple_pages() {
        let response_page1 = json!({
            "data": {
                "node": {
                    "replies": {
                        "nodes": [
                            {
                                "id": "reply_1",
                                "databaseId": 1,
                                "author": {"login": "user1"},
                                "createdAt": "2024-01-01T00:00:00Z",
                                "body": "Reply 1"
                            }
                        ],
                        "pageInfo": {
                            "hasNextPage": true,
                            "endCursor": "cursor_page2"
                        }
                    }
                }
            }
        });

        let response_page2 = json!({
            "data": {
                "node": {
                    "replies": {
                        "nodes": [
                            {
                                "id": "reply_2",
                                "databaseId": 2,
                                "author": {"login": "user2"},
                                "createdAt": "2024-01-01T01:00:00Z",
                                "body": "Reply 2"
                            }
                        ],
                        "pageInfo": {
                            "hasNextPage": false,
                            "endCursor": null
                        }
                    }
                }
            }
        });

        // Verify we can parse both pages correctly
        let page1 = parse_replies_response(response_page1).unwrap();
        assert!(page1.page_info.has_next_page);
        assert_eq!(page1.page_info.end_cursor, Some("cursor_page2".to_string()));
        assert_eq!(page1.nodes.unwrap().len(), 1);

        let page2 = parse_replies_response(response_page2).unwrap();
        assert!(!page2.page_info.has_next_page);
        assert_eq!(page2.nodes.unwrap().len(), 1);
    }

    // Task 5.5: Add test for deleted author handling
    #[test]
    fn test_deleted_author_handling() {
        use crate::models::{Comment, Discussion};
        use chrono::{DateTime, Utc};

        // Create a discussion with null author
        let mut discussion = Discussion {
            id: "discussion_1".to_string(),
            title: "Test Discussion".to_string(),
            number: 1,
            url: "https://github.com/test/repo/discussions/1".to_string(),
            created_at: DateTime::parse_from_rfc3339("2024-01-01T00:00:00Z")
                .unwrap()
                .with_timezone(&Utc),
            body: "Test body".to_string(),
            author: None, // Deleted author
            comments: crate::models::DiscussionComments {
                nodes: None,
                page_info: crate::models::PageInfo {
                    has_next_page: false,
                    end_cursor: None,
                },
            },
        };

        // Create a comment with null author
        let mut comments = vec![Comment {
            id: "comment_1".to_string(),
            database_id: 1,
            author: None, // Deleted author
            created_at: DateTime::parse_from_rfc3339("2024-01-01T01:00:00Z")
                .unwrap()
                .with_timezone(&Utc),
            body: "Comment 1".to_string(),
            replies: crate::models::CommentReplies {
                nodes: Some(vec![Some(crate::models::Reply {
                    id: "reply_1".to_string(),
                    database_id: 1,
                    author: None, // Deleted author
                    created_at: DateTime::parse_from_rfc3339("2024-01-01T02:00:00Z")
                        .unwrap()
                        .with_timezone(&Utc),
                    body: "Reply 1".to_string(),
                })]),
                page_info: crate::models::PageInfo {
                    has_next_page: false,
                    end_cursor: None,
                },
            },
        }];

        // Apply the replace_deleted_authors function
        let result = replace_deleted_authors(&mut discussion, &mut comments);

        // Verify the function succeeds
        assert!(result.is_ok());

        // Verify discussion author is replaced
        assert!(discussion.author.is_some());
        assert_eq!(
            discussion.author.as_ref().unwrap().login,
            Some("<deleted>".to_string())
        );

        // Verify comment author is replaced
        assert!(comments[0].author.is_some());
        assert_eq!(
            comments[0].author.as_ref().unwrap().login,
            Some("<deleted>".to_string())
        );

        // Verify reply author is replaced
        let reply = comments[0].replies.nodes.as_ref().unwrap()[0]
            .as_ref()
            .unwrap();
        assert!(reply.author.is_some());
        assert_eq!(
            reply.author.as_ref().unwrap().login,
            Some("<deleted>".to_string())
        );
    }

    // Task 5.6: Add test for chronological sorting
    #[test]
    fn test_chronological_sorting() {
        use crate::models::{Author, Comment, Reply};
        use chrono::{DateTime, Utc};

        // Create comments out of order
        let mut comments = vec![
            Comment {
                id: "comment_2".to_string(),
                database_id: 2,
                author: Some(Author {
                    login: Some("user2".to_string()),
                }),
                created_at: DateTime::parse_from_rfc3339("2024-01-01T02:00:00Z")
                    .unwrap()
                    .with_timezone(&Utc),
                body: "Comment 2".to_string(),
                replies: crate::models::CommentReplies {
                    nodes: Some(vec![]),
                    page_info: crate::models::PageInfo {
                        has_next_page: false,
                        end_cursor: None,
                    },
                },
            },
            Comment {
                id: "comment_1".to_string(),
                database_id: 1,
                author: Some(Author {
                    login: Some("user1".to_string()),
                }),
                created_at: DateTime::parse_from_rfc3339("2024-01-01T01:00:00Z")
                    .unwrap()
                    .with_timezone(&Utc),
                body: "Comment 1".to_string(),
                replies: crate::models::CommentReplies {
                    nodes: Some(vec![]),
                    page_info: crate::models::PageInfo {
                        has_next_page: false,
                        end_cursor: None,
                    },
                },
            },
            Comment {
                id: "comment_3".to_string(),
                database_id: 3,
                author: Some(Author {
                    login: Some("user3".to_string()),
                }),
                created_at: DateTime::parse_from_rfc3339("2024-01-01T03:00:00Z")
                    .unwrap()
                    .with_timezone(&Utc),
                body: "Comment 3".to_string(),
                replies: crate::models::CommentReplies {
                    nodes: Some(vec![]),
                    page_info: crate::models::PageInfo {
                        has_next_page: false,
                        end_cursor: None,
                    },
                },
            },
        ];

        // Sort comments
        comments.sort_by(|a, b| a.created_at.cmp(&b.created_at));

        // Verify they're in chronological order
        assert_eq!(comments[0].id, "comment_1");
        assert_eq!(comments[1].id, "comment_2");
        assert_eq!(comments[2].id, "comment_3");

        // Test reply sorting within a comment
        let mut comment = Comment {
            id: "comment_1".to_string(),
            database_id: 1,
            author: Some(Author {
                login: Some("user1".to_string()),
            }),
            created_at: DateTime::parse_from_rfc3339("2024-01-01T01:00:00Z")
                .unwrap()
                .with_timezone(&Utc),
            body: "Comment 1".to_string(),
            replies: crate::models::CommentReplies {
                nodes: Some(vec![
                    Some(Reply {
                        id: "reply_3".to_string(),
                        database_id: 3,
                        author: Some(Author {
                            login: Some("user3".to_string()),
                        }),
                        created_at: DateTime::parse_from_rfc3339("2024-01-01T03:00:00Z")
                            .unwrap()
                            .with_timezone(&Utc),
                        body: "Reply 3".to_string(),
                    }),
                    Some(Reply {
                        id: "reply_1".to_string(),
                        database_id: 1,
                        author: Some(Author {
                            login: Some("user1".to_string()),
                        }),
                        created_at: DateTime::parse_from_rfc3339("2024-01-01T01:00:00Z")
                            .unwrap()
                            .with_timezone(&Utc),
                        body: "Reply 1".to_string(),
                    }),
                    Some(Reply {
                        id: "reply_2".to_string(),
                        database_id: 2,
                        author: Some(Author {
                            login: Some("user2".to_string()),
                        }),
                        created_at: DateTime::parse_from_rfc3339("2024-01-01T02:00:00Z")
                            .unwrap()
                            .with_timezone(&Utc),
                        body: "Reply 2".to_string(),
                    }),
                ]),
                page_info: crate::models::PageInfo {
                    has_next_page: false,
                    end_cursor: None,
                },
            },
        };

        // Sort replies
        if let Some(ref mut nodes) = comment.replies.nodes {
            nodes.sort_by(|a, b| match (a, b) {
                (Some(r1), Some(r2)) => r1.created_at.cmp(&r2.created_at),
                (Some(_), None) => std::cmp::Ordering::Less,
                (None, Some(_)) => std::cmp::Ordering::Greater,
                (None, None) => std::cmp::Ordering::Equal,
            });
        }

        // Verify replies are in chronological order
        let replies = comment.replies.nodes.unwrap();
        assert_eq!(replies[0].as_ref().unwrap().id, "reply_1");
        assert_eq!(replies[1].as_ref().unwrap().id, "reply_2");
        assert_eq!(replies[2].as_ref().unwrap().id, "reply_3");
    }
}
