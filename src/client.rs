use crate::error::{Error, Result};
use crate::models::Discussion;
#[cfg(test)]
use mockall::automock;

const GITHUB_GRAPHQL_URL: &str = "https://api.github.com/graphql";

/// HTTP client trait for making POST requests
///
/// This trait allows mocking HTTP requests in tests without starting a real server.
#[cfg_attr(test, automock)]
pub trait HttpClient: Send + Sync {
    /// Send a POST request with a JSON body
    fn post(&self, url: &str, body: &str) -> Result<String>;
}

/// Production HTTP client using reqwest
pub struct ReqwestClient {
    client: reqwest::blocking::Client,
    token: String,
}

impl ReqwestClient {
    /// Create a new ReqwestClient with the given GitHub token
    pub fn new(token: String) -> Result<Self> {
        let client = reqwest::blocking::Client::builder()
            .user_agent("gh-discussion-export")
            .build()
            .map_err(|e| Error::Http(format!("Failed to create HTTP client: {}", e)))?;

        Ok(Self { client, token })
    }
}

impl HttpClient for ReqwestClient {
    fn post(&self, url: &str, body: &str) -> Result<String> {
        let response = self
            .client
            .post(url)
            .bearer_auth(&self.token)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .body(body.to_string())
            .send()
            .map_err(|e| Error::Http(format!("Request failed: {}", e)))?;

        let status = response.status();

        // Try to extract rate limit information from headers before consuming response
        let is_rate_limit = status.as_u16() == 429
            || (status.as_u16() == 403
                && response
                    .headers()
                    .get("X-RateLimit-Remaining")
                    .and_then(|v| v.to_str().ok())
                    .map(|v| v == "0")
                    .unwrap_or(false));

        let response_text = response
            .text()
            .map_err(|e| Error::Http(format!("Failed to read response: {}", e)))?;

        // Handle HTTP error status codes
        if status.as_u16() == 401 {
            return Err(Error::Authentication);
        } else if is_rate_limit {
            return Err(Error::RateLimit);
        } else if status.as_u16() == 403 {
            return Err(Error::PermissionDenied(format!(
                "Access denied: {}",
                response_text
            )));
        } else if !status.is_success() {
            return Err(Error::Http(format!(
                "HTTP error {}: {}",
                status.as_u16(),
                response_text
            )));
        }

        Ok(response_text)
    }
}

/// GraphQL client for GitHub's API
pub struct GitHubClient {
    http_client: Box<dyn HttpClient>,
}

impl GitHubClient {
    /// Create a new GitHubClient with the given HTTP client
    pub fn new(http_client: Box<dyn HttpClient>) -> Self {
        Self { http_client }
    }

    /// Execute a GraphQL query and return the Discussion data
    ///
    /// # Arguments
    /// * `query` - GraphQL query string
    /// * `variables` - Query variables as a JSON value
    ///
    /// # Returns
    /// The Discussion object from the response
    pub fn execute_query(&self, query: &str, variables: serde_json::Value) -> Result<Discussion> {
        // Build the request body
        let request_body = serde_json::json!({
            "query": query,
            "variables": variables
        });

        let body_str = request_body.to_string();

        // Send the request
        let response_text = self.http_client.post(GITHUB_GRAPHQL_URL, &body_str)?;

        // Parse the response
        let response: serde_json::Value = serde_json::from_str(&response_text)
            .map_err(|e| Error::JsonParse(format!("Failed to parse JSON: {}", e)))?;

        // Check for GraphQL errors
        if let Some(errors) = response.get("errors").and_then(|e| e.as_array())
            && !errors.is_empty() {
            let error_messages: Vec<String> = errors
                .iter()
                .filter_map(|e| e.get("message").and_then(|m| m.as_str()))
                .map(|s| s.to_string())
                .collect();
            return Err(Error::GraphQL(error_messages.join("; ")));
        }

        // Extract the data
        let data = response
            .get("data")
            .ok_or_else(|| Error::JsonParse("Response missing 'data' field".to_string()))?;

        let repository = data
            .get("repository")
            .ok_or_else(|| Error::JsonParse("Response missing 'repository' field".to_string()))?;

        let discussion_value = repository
            .get("discussion")
            .ok_or_else(|| Error::JsonParse("Response missing 'discussion' field".to_string()))?;

        // Parse the Discussion object
        let discussion: Discussion = serde_json::from_value(discussion_value.clone())
            .map_err(|e| Error::JsonParse(format!("Failed to parse Discussion: {}", e)))?;

        Ok(discussion)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{
        Author, Comment, CommentReplies, Discussion, DiscussionComments, PageInfo,
    };
    use chrono::{DateTime, Utc};

    fn create_mock_discussion() -> Discussion {
        Discussion {
            title: "Test Discussion".to_string(),
            number: 1,
            url: "https://github.com/test/repo/discussions/1".to_string(),
            created_at: DateTime::parse_from_rfc3339("2024-01-01T00:00:00Z")
                .unwrap()
                .with_timezone(&Utc),
            body: "Test body".to_string(),
            comments: DiscussionComments {
                nodes: Some(vec![Some(Comment {
                    id: "1".to_string(),
                    database_id: 1,
                    author: Some(Author {
                        login: Some("testuser".to_string()),
                    }),
                    created_at: DateTime::parse_from_rfc3339("2024-01-01T00:00:00Z")
                        .unwrap()
                        .with_timezone(&Utc),
                    body: "Test comment".to_string(),
                    replies: CommentReplies {
                        nodes: Some(vec![]),
                        page_info: PageInfo {
                            has_next_page: false,
                            end_cursor: None,
                        },
                    },
                })]),
                page_info: PageInfo {
                    has_next_page: false,
                    end_cursor: None,
                },
            },
        }
    }

    #[test]
    fn test_reqwest_client_creation() {
        let client = ReqwestClient::new("test_token".to_string());
        assert!(client.is_ok());
    }

    #[test]
    fn test_github_client_creation() {
        let mock_http = Box::new(MockHttpClient::new());
        let _client = GitHubClient::new(mock_http);
        // Test passes if we can create a GitHubClient with a mock
    }

    #[test]
    fn test_successful_query_execution() {
        let mut mock_http = MockHttpClient::new();
        mock_http.expect_post().times(1).returning(|_url, _body| {
            Ok(serde_json::json!({
                "data": {
                    "repository": {
                        "discussion": {
                            "title": "Test Discussion",
                            "number": 1,
                            "url": "https://github.com/test/repo/discussions/1",
                            "createdAt": "2024-01-01T00:00:00Z",
                            "body": "Test body",
                            "comments": {
                                "nodes": [
                                    {
                                        "id": "1",
                                        "databaseId": 1,
                                        "author": {"login": "testuser"},
                                        "createdAt": "2024-01-01T00:00:00Z",
                                        "body": "Test comment",
                                        "replies": {
                                            "nodes": [],
                                            "pageInfo": {"hasNextPage": false, "endCursor": null}
                                        }
                                    }
                                ],
                                "pageInfo": {"hasNextPage": false, "endCursor": null}
                            }
                        }
                    }
                }
            })
            .to_string())
        });

        let client = GitHubClient::new(Box::new(mock_http));
        let result = client.execute_query("query {}", serde_json::json!({}));
        assert!(result.is_ok());
        let discussion = result.unwrap();
        assert_eq!(discussion.title, "Test Discussion");
        assert_eq!(discussion.number, 1);
    }

    #[test]
    fn test_graphql_error_response() {
        let mut mock_http = MockHttpClient::new();
        mock_http.expect_post().times(1).returning(|_url, _body| {
            Ok(serde_json::json!({
                "data": null,
                "errors": [
                    {
                        "message": "Field 'invalid' doesn't exist on type 'Query'",
                        "path": ["repository", "discussion", "invalid"]
                    }
                ]
            })
            .to_string())
        });

        let client = GitHubClient::new(Box::new(mock_http));
        let result = client.execute_query("query {}", serde_json::json!({}));
        assert!(result.is_err());
        match result {
            Err(Error::GraphQL(msg)) => assert!(msg.contains("invalid")),
            _ => panic!("Expected GraphQL error"),
        }
    }

    #[test]
    fn test_http_401_error() {
        let mut mock_http = MockHttpClient::new();
        mock_http
            .expect_post()
            .times(1)
            .returning(|_url, _body| Err(Error::Authentication));

        let client = GitHubClient::new(Box::new(mock_http));
        let result = client.execute_query("query {}", serde_json::json!({}));
        assert!(result.is_err());
        match result {
            Err(Error::Authentication) => {}
            _ => panic!("Expected Authentication error"),
        }
    }

    #[test]
    fn test_http_403_rate_limit_error() {
        let mut mock_http = MockHttpClient::new();
        mock_http
            .expect_post()
            .times(1)
            .returning(|_url, _body| Err(Error::RateLimit));

        let client = GitHubClient::new(Box::new(mock_http));
        let result = client.execute_query("query {}", serde_json::json!({}));
        assert!(result.is_err());
        match result {
            Err(Error::RateLimit) => {}
            _ => panic!("Expected RateLimit error"),
        }
    }

    #[test]
    fn test_missing_data_field() {
        let mut mock_http = MockHttpClient::new();
        mock_http
            .expect_post()
            .times(1)
            .returning(|_url, _body| Ok(serde_json::json!({}).to_string()));

        let client = GitHubClient::new(Box::new(mock_http));
        let result = client.execute_query("query {}", serde_json::json!({}));
        assert!(result.is_err());
        match result {
            Err(Error::JsonParse(msg)) => assert!(msg.contains("data")),
            _ => panic!("Expected JsonParse error"),
        }
    }

    #[test]
    fn test_missing_repository_field() {
        let mut mock_http = MockHttpClient::new();
        mock_http.expect_post().times(1).returning(|_url, _body| {
            Ok(serde_json::json!({
                "data": {}
            })
            .to_string())
        });

        let client = GitHubClient::new(Box::new(mock_http));
        let result = client.execute_query("query {}", serde_json::json!({}));
        assert!(result.is_err());
        match result {
            Err(Error::JsonParse(msg)) => assert!(msg.contains("repository")),
            _ => panic!("Expected JsonParse error"),
        }
    }

    #[test]
    fn test_http_403_permission_denied_error() {
        let mut mock_http = MockHttpClient::new();
        mock_http
            .expect_post()
            .times(1)
            .returning(|_url, _body| Err(Error::PermissionDenied("Access denied".to_string())));

        let client = GitHubClient::new(Box::new(mock_http));
        let result = client.execute_query("query {}", serde_json::json!({}));
        assert!(result.is_err());
        match result {
            Err(Error::PermissionDenied(msg)) => assert!(msg.contains("Access denied")),
            _ => panic!("Expected PermissionDenied error"),
        }
    }
}
