// Asset detection and extraction module

use std::collections::HashSet;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::mpsc;
use std::thread;

/// Extract UUID from a GitHub asset URL.
///
/// GitHub asset URLs have the format: `https://github.com/user-attachments/assets/<uuid>`
/// This function extracts the UUID portion if the URL matches this pattern.
///
/// # Arguments
/// * `url` - The asset URL to parse
///
/// # Returns
/// * `Some(String)` - The extracted UUID if the URL is a valid GitHub asset URL
/// * `None` - If the URL doesn't match the expected pattern
pub fn extract_asset_uuid(url: &str) -> Option<String> {
    // Match github.com/user-attachments/assets/UUID format
    // SECURITY: Use starts_with to ensure we only match actual GitHub URLs
    // prevents matching malicious URLs like https://evil.com/github.com/user-attachments/assets/
    if url.starts_with("https://github.com/user-attachments/assets/") {
        let parts: Vec<&str> = url.split("github.com/user-attachments/assets/").collect();
        if parts.len() > 1 {
            let uuid = parts[1].split('/').next().unwrap_or("");
            if !uuid.is_empty() {
                return Some(uuid.to_string());
            }
        }
    }
    None
}

/// Detect all GitHub asset URLs in HTML content.
///
/// Parses HTML and extracts all src attributes from <img> tags that point to
/// GitHub user-attachments assets.
///
/// # Arguments
/// * `html` - The HTML content to scan
///
/// # Returns
/// A vector of all detected GitHub asset URLs
pub fn detect_asset_urls(html: &str) -> Vec<String> {
    use scraper::{Html, Selector};

    let document = Html::parse_fragment(html);
    let selector = Selector::parse("img").unwrap();

    document
        .select(&selector)
        .filter_map(|el| el.value().attr("src"))
        .filter(|src| extract_asset_uuid(src).is_some())
        .map(|s| s.to_string())
        .collect()
}

/// Detect all GitHub asset URLs in Markdown image syntax.
///
/// Scans Markdown content for image references `![alt](url)` or `![alt](url "title")`
/// and extracts those pointing to GitHub user-attachments assets.
///
/// # Arguments
/// * `text` - The Markdown text to scan
///
/// # Returns
/// A vector of all detected GitHub asset URLs
pub fn detect_markdown_assets(text: &str) -> Vec<String> {
    let mut urls = Vec::new();

    // Match Markdown image syntax: ![alt](url) or ![alt](url "title")
    for line in text.lines() {
        let mut start = 0;
        while let Some(img_start) = line[start..].find("![").and_then(|pos| {
            let after_bracket = &line[start + pos..];
            after_bracket.find("](").map(|end| start + pos + end + 2)
        }) {
            // Find the closing parenthesis
            if let Some(img_end) = line[img_start..].find(')') {
                // Extract full content between ]( and )
                let full_content = &line[img_start..img_start + img_end];
                // Split on first space to separate URL from optional title
                // Format: url or url "title"
                let url = full_content
                    .split_once(' ')
                    .map(|(url_part, _)| url_part)
                    .unwrap_or(full_content);
                if extract_asset_uuid(url).is_some() {
                    urls.push(url.to_string());
                }
                start = img_start + img_end + 1;
            } else {
                break;
            }
        }
    }

    urls
}

/// Deduplicate asset URLs to get unique UUIDs.
///
/// Returns a list of unique asset URLs, preserving order and removing duplicates.
///
/// # Arguments
/// * `urls` - A vector of asset URLs (may contain duplicates)
///
/// # Returns
/// A vector of unique URLs in their original order
pub fn dedupe_asset_urls(urls: Vec<String>) -> Vec<String> {
    let mut seen = HashSet::new();
    let mut unique = Vec::new();

    for url in urls {
        if let Some(uuid) = extract_asset_uuid(&url)
            && !seen.contains(&uuid)
        {
            seen.insert(uuid);
            unique.push(url);
        }
    }

    unique
}

/// Map HTTP Content-Type header to file extension.
///
/// Returns the appropriate file extension for a given Content-Type,
/// defaulting to ".bin" for unknown or binary types.
///
/// # Arguments
/// * `content_type` - The Content-Type header value (e.g., "image/png")
///
/// # Returns
/// File extension including the dot (e.g., ".png", ".jpg", ".bin")
pub fn content_type_to_extension(content_type: &str) -> String {
    let ext = match content_type {
        "image/png" => "png",
        "image/jpeg" => "jpg",
        "image/jpg" => "jpg",
        "image/svg+xml" => "svg",
        "image/gif" => "gif",
        "image/webp" => "webp",
        "image/avif" => "avif",
        "application/octet-stream" | "" => "bin",
        _ => "bin",
    };
    format!(".{}", ext)
}

/// Download a single asset to local path with authentication.
///
/// Downloads an asset from GitHub using bearer authentication and saves it
/// to the specified local path.
///
/// # Arguments
/// * `client` - HTTP client for making requests
/// * `token` - GitHub authentication token
/// * `url` - Asset URL to download
/// * `path` - Local file path where asset should be saved
///
/// # Returns
/// * `Ok(())` if download succeeded
/// * `Err(Error)` if download failed (network error, IO error, etc.)
///
/// # Behavior
/// - Skips download if file already exists (task 11.2)
/// - Handles 401 (authentication), 403 (permission), 404 (not found) with specific errors
/// - Handles network timeout with descriptive error message
/// - Handles permission/disk space errors when writing files
pub fn download_asset(
    client: &reqwest::blocking::Client,
    token: &str,
    url: &str,
    path: &Path,
) -> crate::error::Result<()> {
    // Task 11.2: Skip re-download if file already exists
    if path.exists() {
        return Ok(());
    }

    // Download with bearer authentication (task 5.4 requirement)
    let response = client.get(url).bearer_auth(token).send().map_err(|e| {
        // Task 11.3: Detect timeout and provide clear message
        if e.is_timeout() {
            crate::error::Error::Http(format!("Network timeout while downloading asset: {}", url))
        } else if e.is_connect() {
            crate::error::Error::Http(format!(
                "Failed to connect to server while downloading asset: {}",
                url
            ))
        } else {
            crate::error::Error::Http(format!("Failed to download asset: {}", e))
        }
    })?;

    // Check HTTP status
    let status = response.status();
    if status.as_u16() == 401 {
        // Task 11.6: Handle 401 with clear message
        return Err(crate::error::Error::Authentication);
    } else if status.as_u16() == 404 {
        // Task 11.4: Asset not found
        return Err(crate::error::Error::Http(format!(
            "Asset not found (HTTP 404): {}",
            url
        )));
    } else if status.as_u16() == 403 {
        // Task 11.5: Authentication failed or access denied
        return Err(crate::error::Error::PermissionDenied(format!(
            "Authentication failed or access denied (HTTP 403): {}",
            url
        )));
    } else if !status.is_success() {
        return Err(crate::error::Error::Http(format!(
            "Failed to download asset: HTTP {}",
            status.as_u16()
        )));
    }

    // Read response body
    let bytes = response
        .bytes()
        .map_err(|e| crate::error::Error::Http(format!("Failed to read response body: {}", e)))?;

    // Write to file (create parent directories if needed)
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| {
            // Task 11.7 & 11.8: Better error messages for IO errors
            categorize_io_error(e, "create directory")
        })?;
    }

    let mut file = File::create(path).map_err(|e| categorize_io_error(e, "create file"))?;

    file.write_all(&bytes)
        .map_err(|e| categorize_io_error(e, "write file"))?;

    Ok(())
}

/// Categorize IO errors into more specific error types.
///
/// Helps distinguish between permission errors, disk space errors, and other IO issues.
fn categorize_io_error(e: std::io::Error, operation: &str) -> crate::error::Error {
    use std::io::ErrorKind;
    match e.kind() {
        ErrorKind::PermissionDenied => crate::error::Error::Io(std::io::Error::new(
            ErrorKind::PermissionDenied,
            format!(
                "Permission denied: failed to {} for asset: {}",
                operation, e
            ),
        )),
        ErrorKind::StorageFull | ErrorKind::WriteZero => {
            crate::error::Error::Io(std::io::Error::new(
                ErrorKind::StorageFull,
                "Disk full or storage quota exceeded",
            ))
        }
        _ => crate::error::Error::Io(e),
    }
}

/// Download multiple assets in parallel with configurable parallelism.
///
/// Downloads assets concurrently using multiple threads, returning results
/// for each URL indicating success or failure.
///
/// # Arguments
/// * `client` - HTTP client for making requests (cloned for each thread)
/// * `token` - GitHub authentication token
/// * `urls` - Asset URLs to download
/// * `asset_dir` - Directory where assets should be saved
/// * `parallel` - Maximum number of concurrent downloads
///
/// # Returns
/// * Vector of Results, one per URL (Ok means downloaded, Err means failed)
pub fn download_assets_parallel(
    client: &reqwest::blocking::Client,
    token: &str,
    urls: Vec<String>,
    asset_dir: &Path,
    parallel: usize,
) -> Vec<crate::error::Result<()>> {
    let token = Arc::new(token.to_string());
    let (sender, receiver) = mpsc::channel();
    let asset_dir = PathBuf::from(asset_dir);

    // Process URLs in chunks to limit parallelism
    for chunk in urls.chunks(parallel) {
        let mut handles = Vec::new();

        for url in chunk {
            let client = client.clone();
            let token = Arc::clone(&token);
            let sender = sender.clone();
            let url = url.clone(); // Clone URL to move into thread

            // Extract UUID for filename
            if let Some(uuid) = extract_asset_uuid(&url) {
                let filename = format!("{}{}", uuid, ".bin"); // Default extension
                let path = asset_dir.join(&filename);

                let handle = thread::spawn(move || {
                    let result = download_asset(&client, &token, &url, &path);
                    sender.send(result).unwrap();
                });

                handles.push(handle);
            }
        }

        // Wait for this chunk to complete before starting next chunk
        for handle in handles {
            handle.join().unwrap();
        }
    }

    // Collect all results from channel
    receiver.iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_uuid_valid_url() {
        let url = "https://github.com/user-attachments/assets/6c72b402-4a5c-45cc-9b0a-50717f8a09a7";
        assert_eq!(
            extract_asset_uuid(url),
            Some("6c72b402-4a5c-45cc-9b0a-50717f8a09a7".to_string())
        );
    }

    #[test]
    fn test_extract_uuid_non_github_url() {
        let url = "https://example.com/image.png";
        assert_eq!(extract_asset_uuid(url), None);
    }

    #[test]
    fn test_extract_uuid_empty_string() {
        assert_eq!(extract_asset_uuid(""), None);
    }

    #[test]
    fn test_extract_uuid_rejects_malicious_domain() {
        // SECURITY: Must reject URLs from non-GitHub domains
        // even if they contain the github.com path pattern
        let malicious_url = "https://evil.com/github.com/user-attachments/assets/steal-token";
        assert_eq!(extract_asset_uuid(malicious_url), None);
    }

    #[test]
    fn test_extract_uuid_rejects_http_not_https() {
        // SECURITY: GitHub uses HTTPS only
        let http_url =
            "http://github.com/user-attachments/assets/6c72b402-4a5c-45cc-9b0a-50717f8a09a7";
        assert_eq!(extract_asset_uuid(http_url), None);
    }

    #[test]
    fn test_extract_uuid_rejects_subdomain_attack() {
        // SECURITY: Must be exact domain, not subdomain
        let subdomain_url =
            "https://fake-github.com/user-attachments/assets/6c72b402-4a5c-45cc-9b0a-50717f8a09a7";
        assert_eq!(extract_asset_uuid(subdomain_url), None);
    }

    #[test]
    fn test_detect_asset_urls_html_img_tag() {
        let html = r#"<img src="https://github.com/user-attachments/assets/6c72b402-4a5c-45cc-9b0a-50717f8a09a7" alt="Diagram" />"#;
        let urls = detect_asset_urls(html);
        assert_eq!(urls.len(), 1);
        assert_eq!(
            urls[0],
            "https://github.com/user-attachments/assets/6c72b402-4a5c-45cc-9b0a-50717f8a09a7"
        );
    }

    #[test]
    fn test_detect_asset_urls_no_assets() {
        let html = r#"<p>This is a paragraph without images.</p>"#;
        let urls = detect_asset_urls(html);
        assert_eq!(urls.len(), 0);
    }

    #[test]
    fn test_detect_markdown_assets_single_image() {
        let text = "![Diagram](https://github.com/user-attachments/assets/6c72b402-4a5c-45cc-9b0a-50717f8a09a7)";
        let urls = detect_markdown_assets(text);
        assert_eq!(urls.len(), 1);
        assert_eq!(
            urls[0],
            "https://github.com/user-attachments/assets/6c72b402-4a5c-45cc-9b0a-50717f8a09a7"
        );
    }

    #[test]
    fn test_detect_markdown_assets_multiple_images() {
        let text = r#"
![Image1](https://github.com/user-attachments/assets/6c72b402-4a5c-45cc-9b0a-50717f8a09a7)
![Image2](https://github.com/user-attachments/assets/7d83c513-5b6d-46dd-a01b-61728e8b0a8b)
"#;
        let urls = detect_markdown_assets(text);
        assert_eq!(urls.len(), 2);
    }

    #[test]
    fn test_detect_markdown_assets_no_images() {
        let text = "This is plain text without images.";
        let urls = detect_markdown_assets(text);
        assert_eq!(urls.len(), 0);
    }

    #[test]
    fn test_detect_markdown_assets_ignores_external_images() {
        let text = "![External](https://example.com/image.png)";
        let urls = detect_markdown_assets(text);
        assert_eq!(urls.len(), 0);
    }

    #[test]
    fn test_dedupe_asset_urls_removes_duplicates() {
        let urls = vec![
            "https://github.com/user-attachments/assets/6c72b402-4a5c-45cc-9b0a-50717f8a09a7"
                .to_string(),
            "https://github.com/user-attachments/assets/6c72b402-4a5c-45cc-9b0a-50717f8a09a7"
                .to_string(),
            "https://github.com/user-attachments/assets/7d83c513-5b6d-46dd-a01b-61728e8b0a8b"
                .to_string(),
        ];
        let unique = dedupe_asset_urls(urls);
        assert_eq!(unique.len(), 2);
    }

    #[test]
    fn test_dedupe_asset_urls_preserves_order() {
        let urls = vec![
            "https://github.com/user-attachments/assets/uuid1".to_string(),
            "https://github.com/user-attachments/assets/uuid2".to_string(),
            "https://github.com/user-attachments/assets/uuid1".to_string(),
        ];
        let unique = dedupe_asset_urls(urls);
        assert_eq!(unique.len(), 2);
        assert_eq!(
            unique[0],
            "https://github.com/user-attachments/assets/uuid1"
        );
        assert_eq!(
            unique[1],
            "https://github.com/user-attachments/assets/uuid2"
        );
    }

    #[test]
    fn test_dedupe_asset_urls_empty_vector() {
        let urls: Vec<String> = vec![];
        let unique = dedupe_asset_urls(urls);
        assert_eq!(unique.len(), 0);
    }

    #[test]
    fn test_content_type_to_extension_png() {
        assert_eq!(content_type_to_extension("image/png"), ".png");
    }

    #[test]
    fn test_content_type_to_extension_jpeg() {
        assert_eq!(content_type_to_extension("image/jpeg"), ".jpg");
    }

    #[test]
    fn test_content_type_to_extension_jpg() {
        assert_eq!(content_type_to_extension("image/jpg"), ".jpg");
    }

    #[test]
    fn test_content_type_to_extension_svg() {
        assert_eq!(content_type_to_extension("image/svg+xml"), ".svg");
    }

    #[test]
    fn test_content_type_to_extension_octet_stream() {
        assert_eq!(
            content_type_to_extension("application/octet-stream"),
            ".bin"
        );
    }

    #[test]
    fn test_content_type_to_extension_unknown() {
        assert_eq!(content_type_to_extension("application/unknown"), ".bin");
    }

    #[test]
    fn test_content_type_to_extension_empty_string() {
        assert_eq!(content_type_to_extension(""), ".bin");
    }

    #[test]
    fn test_content_type_to_extension_gif() {
        assert_eq!(content_type_to_extension("image/gif"), ".gif");
    }

    #[test]
    fn test_content_type_to_extension_webp() {
        assert_eq!(content_type_to_extension("image/webp"), ".webp");
    }

    #[test]
    fn test_content_type_to_extension_avif() {
        assert_eq!(content_type_to_extension("image/avif"), ".avif");
    }

    // Task 11.9: Unit tests for error categorization
    #[test]
    fn test_categorize_io_error_permission_denied() {
        use std::io::{self, ErrorKind};
        let error = io::Error::new(ErrorKind::PermissionDenied, "access denied");
        let result = categorize_io_error(error, "test");
        // Should be an Io error with permission denied message
        assert!(matches!(result, crate::error::Error::Io(_)));
    }

    #[test]
    fn test_categorize_io_error_other() {
        use std::io;
        let error = io::Error::other("some error");
        let result = categorize_io_error(error, "test");
        // Should be an Io error
        assert!(matches!(result, crate::error::Error::Io(_)));
    }

    #[test]
    fn test_skip_existing_file() {
        // Task 11.2: Verify that download_asset skips existing files
        use std::io::Write;
        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("test_existing_asset.bin");

        // Create a test file
        let mut file = File::create(&test_file).unwrap();
        file.write_all(b"test content").unwrap();
        drop(file);

        // Create a client (won't be used since file exists)
        let client = reqwest::blocking::Client::new();

        // Try to download to existing path - should succeed without network call
        let result = download_asset(
            &client,
            "fake_token",
            "https://github.com/user-attachments/assets/test-uuid",
            &test_file,
        );

        // Should succeed (file exists, download skipped)
        assert!(result.is_ok());

        // Clean up
        std::fs::remove_file(&test_file).ok();
    }

    // Integration tests for download functions (Tasks 5.7, 5.8)
    // Note: Full integration tests with real HTTP requests are manual tests.
    // See tasks 10.7, 10.8, 10.9 for end-to-end testing with real GitHub assets.
    // The download functions are tested via manual E2E testing to avoid:
    // - Requiring real GitHub authentication in CI/CD
    // - Network dependency in automated tests
    // - Flaky tests due to external service availability
    // Manual testing ensures real-world validation of asset download behavior.
}
