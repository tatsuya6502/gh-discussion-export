// Asset detection and extraction module

use std::collections::HashSet;

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
    if url.contains("github.com/user-attachments/assets/") {
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
                let url = full_content.split_once(' ')
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
}
