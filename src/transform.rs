// Markdown transformation module for asset URL replacement
//
// This module transforms GitHub asset URLs in HTML and Markdown content
// to reference local paths while preserving original URLs for reference.

use crate::assets::extract_asset_uuid;
use std::collections::HashMap;

/// Transform HTML `<img>` tags to use local asset paths.
///
/// Replaces GitHub asset URLs in `src` attributes with local paths,
/// and adds `data-original-url` attribute to preserve the original URL.
/// All other attributes are preserved verbatim.
///
/// # Arguments
/// * `html` - The HTML content to transform
/// * `asset_map` - Mapping from UUID to local file path (e.g., {"uuid": "1041-discussion-assets/uuid.png"})
///
/// # Returns
/// Transformed HTML with local asset paths and preserved original URLs
pub fn transform_html_img_tags(html: &str, asset_map: &HashMap<String, String>) -> String {
    let mut result = html.to_string();
    let mut pos = 0;

    // Find all <img tags
    while let Some(img_start) = result[pos..].find("<img") {
        let absolute_img_start = pos + img_start;

        // Find the end of this img tag (>)
        if let Some(tag_end) = result[absolute_img_start..].find('>') {
            let absolute_tag_end = absolute_img_start + tag_end;
            let img_tag = &result[absolute_img_start..=absolute_tag_end];

            // Extract the src attribute value
            if let Some(src_value) = extract_src_attribute(img_tag) {
                // Check if this is a GitHub asset URL
                if let Some(uuid) = extract_asset_uuid(&src_value)
                    && let Some(local_path) = asset_map.get(&uuid)
                {
                    // Transform the img tag
                    let transformed =
                        transform_img_tag(img_tag, &src_value, local_path, &src_value);

                    // Replace in result
                    result = format!(
                        "{}{}{}",
                        &result[..absolute_img_start],
                        transformed,
                        &result[absolute_tag_end + 1..]
                    );

                    // Update position to continue after this tag
                    pos = absolute_img_start + transformed.len();
                    continue;
                }
            }

            // Move past this tag
            pos = absolute_tag_end + 1;
        } else {
            break;
        }
    }

    result
}

/// Transform Markdown image syntax to use local asset paths.
///
/// Replaces GitHub asset URLs in `![alt](url)` or `![alt](url "title")`
/// with local paths, and adds HTML comment after with original URL.
///
/// # Arguments
/// * `text` - The Markdown text to transform
/// * `asset_map` - Mapping from UUID to local file path
///
/// # Returns
/// Transformed Markdown with local asset paths and preserved original URLs
pub fn transform_markdown_images(text: &str, asset_map: &HashMap<String, String>) -> String {
    let mut result = String::new();

    for line in text.lines() {
        let mut transformed_line = line.to_string();
        let mut start = 0;

        // Find each ![alt](url) or ![alt](url "title") pattern
        while let Some(img_start) = find_image_syntax(&transformed_line[start..]) {
            let absolute_img_start = start + img_start;

            // Find the closing ]
            if let Some(bracket_end) = transformed_line[absolute_img_start..].find(']') {
                let absolute_bracket_end = absolute_img_start + bracket_end;

                // Check for opening ( immediately after ]
                if transformed_line[absolute_bracket_end..].starts_with("](") {
                    // Find the closing )
                    if let Some(paren_end) = transformed_line[absolute_bracket_end + 2..].find(')')
                    {
                        let absolute_paren_end = absolute_bracket_end + 2 + paren_end;

                        // Extract the URL part (between ]( and ))
                        let url_part =
                            &transformed_line[absolute_bracket_end + 2..absolute_paren_end];

                        // Split on space to separate URL from optional title
                        // Format: url or url "title"
                        let (url, title) = if let Some(space_pos) = url_part.find(' ') {
                            let title_with_quotes = &url_part[space_pos + 1..];
                            // Strip surrounding quotes if present (either single or double)
                            let title = if (title_with_quotes.starts_with('"')
                                && title_with_quotes.ends_with('"'))
                                || (title_with_quotes.starts_with('\'')
                                    && title_with_quotes.ends_with('\''))
                            {
                                &title_with_quotes[1..title_with_quotes.len() - 1]
                            } else {
                                title_with_quotes
                            };
                            (&url_part[..space_pos], Some(title))
                        } else {
                            (url_part, None)
                        };

                        // Check if this is a GitHub asset URL
                        if let Some(_uuid) = extract_asset_uuid(url)
                            && let Some(local_path) = asset_map.get(&_uuid.to_string())
                        {
                            // Build replacement string
                            let before = &transformed_line[..absolute_bracket_end + 2]; // ![alt](
                            let after = &transformed_line[absolute_paren_end + 1..]; // Everything after )

                            let replacement = match title {
                                Some(t) => {
                                    // ![alt](local-path "title")after
                                    format!("{}{} \"{}\"){}", before, local_path, t, after)
                                }
                                None => {
                                    // ![alt](local-path)after
                                    let mut s = String::from(before);
                                    s.push_str(local_path);
                                    s.push(')');
                                    s.push_str(after);
                                    s
                                }
                            };

                            // Add HTML comment with original URL
                            let with_comment = format!("{}<!-- {} -->", replacement, url);

                            // Replace the entire image reference
                            transformed_line = format!(
                                "{}{}{}",
                                &transformed_line[..absolute_img_start],
                                with_comment,
                                &transformed_line[absolute_paren_end + 1..]
                            );

                            // Update position to continue after this replacement
                            start = absolute_img_start + with_comment.len();
                            continue;
                        }
                    }
                }
            }

            // Move past this position if no transformation occurred
            start = absolute_img_start + 1;
        }

        result.push_str(&transformed_line);
        result.push('\n');
    }

    // Preserve trailing newline behavior
    if text.ends_with('\n') {
        result
    } else if !result.is_empty() {
        // Remove the extra newline we added
        result.pop();
        result
    } else {
        result
    }
}

/// Transform discussion body with asset URL replacements.
///
/// Applies both HTML and Markdown transformations to discussion body content.
///
/// # Arguments
/// * `body` - The discussion body content
/// * `asset_map` - Mapping from UUID to local file path
///
/// # Returns
/// Transformed body content with local asset paths
pub fn transform_discussion_body(body: &str, asset_map: &HashMap<String, String>) -> String {
    let transformed_html = transform_html_img_tags(body, asset_map);
    transform_markdown_images(&transformed_html, asset_map)
}

/// Transform comment body with asset URL replacements.
///
/// Applies both HTML and Markdown transformations to comment body content.
///
/// # Arguments
/// * `body` - The comment body content
/// * `asset_map` - Mapping from UUID to local file path
///
/// # Returns
/// Transformed body content with local asset paths
pub fn transform_comment_body(body: &str, asset_map: &HashMap<String, String>) -> String {
    transform_discussion_body(body, asset_map)
}

/// Transform reply body with asset URL replacements.
///
/// Applies both HTML and Markdown transformations to reply body content.
///
/// # Arguments
/// * `body` - The reply body content
/// * `asset_map` - Mapping from UUID to local file path
///
/// # Returns
/// Transformed body content with local asset paths
pub fn transform_reply_body(body: &str, asset_map: &HashMap<String, String>) -> String {
    transform_discussion_body(body, asset_map)
}

/// Extract src attribute value from an HTML img tag.
///
/// # Arguments
/// * `img_tag` - The HTML img tag string
///
/// # Returns
/// * `Some(String)` - The src attribute value if found
/// * `None` - If src attribute is not found
fn extract_src_attribute(img_tag: &str) -> Option<String> {
    // Find src="..." or src='...'
    if let Some(src_start) = img_tag.find("src=\"") {
        let after_src = &img_tag[src_start + 5..];
        if let Some(value_end) = after_src.find('"') {
            return Some(after_src[..value_end].to_string());
        }
    }

    // Try single quotes
    if let Some(src_start) = img_tag.find("src='") {
        let after_src = &img_tag[src_start + 5..];
        if let Some(value_end) = after_src.find('\'') {
            return Some(after_src[..value_end].to_string());
        }
    }

    None
}

/// Transform a single img tag with local path and data-original-url.
///
/// # Arguments
/// * `img_tag` - The original img tag HTML
/// * `old_src` - The original src value (for finding and replacing)
/// * `new_src` - The new local path to use
/// * `original_url` - The original URL for data-original-url attribute
///
/// # Returns
/// Transformed img tag HTML
fn transform_img_tag(img_tag: &str, old_src: &str, new_src: &str, original_url: &str) -> String {
    let mut result = img_tag.to_string();

    // Replace src attribute value
    result = result.replace(
        &format!("src=\"{}\"", old_src),
        &format!("src=\"{}\"", new_src),
    );
    result = result.replace(&format!("src='{}'", old_src), &format!("src='{}'", new_src));

    // Add data-original-url attribute before the closing >
    if !result.contains("data-original-url")
        && let Some(tag_end) = result.find('>')
    {
        let before = &result[..tag_end];
        let after = &result[tag_end..];
        result = format!("{} data-original-url=\"{}\"{}", before, original_url, after);
    }

    result
}

/// Find image syntax ![alt](url) starting at position.
///
/// # Arguments
/// * `text` - The text to search
///
/// # Returns
/// * `Some(usize)` - Position of ![ if found
/// * `None` - If no image syntax found
fn find_image_syntax(text: &str) -> Option<usize> {
    let bytes = text.as_bytes();
    let mut pos = 0;

    while pos < bytes.len() {
        if bytes[pos] == b'!' && pos + 1 < bytes.len() && bytes[pos + 1] == b'[' {
            return Some(pos);
        }
        pos += 1;
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_html_img_tag_with_local_path() {
        let html = r#"<img src="https://github.com/user-attachments/assets/6c72b402-4a5c-45cc-9b0a-50717f8a09a7" alt="Diagram" />"#;
        let mut asset_map = HashMap::new();
        asset_map.insert(
            "6c72b402-4a5c-45cc-9b0a-50717f8a09a7".to_string(),
            "1041-discussion-assets/6c72b402-4a5c-45cc-9b0a-50717f8a09a7.png".to_string(),
        );

        let result = transform_html_img_tags(html, &asset_map);

        assert!(
            result.contains(
                "src=\"1041-discussion-assets/6c72b402-4a5c-45cc-9b0a-50717f8a09a7.png\""
            )
        );
        assert!(result.contains("data-original-url=\"https://github.com/user-attachments/assets/6c72b402-4a5c-45cc-9b0a-50717f8a09a7\""));
        assert!(result.contains("alt=\"Diagram\""));
    }

    #[test]
    fn test_transform_html_img_tag_preserves_all_attributes() {
        let html = r#"<img src="https://github.com/user-attachments/assets/6c72b402-4a5c-45cc-9b0a-50717f8a09a7" alt="ER図" width="1192" height="861" loading="lazy" />"#;
        let mut asset_map = HashMap::new();
        asset_map.insert(
            "6c72b402-4a5c-45cc-9b0a-50717f8a09a7".to_string(),
            "1041-discussion-assets/6c72b402-4a5c-45cc-9b0a-50717f8a09a7.png".to_string(),
        );

        let result = transform_html_img_tags(html, &asset_map);

        assert!(
            result.contains(
                "src=\"1041-discussion-assets/6c72b402-4a5c-45cc-9b0a-50717f8a09a7.png\""
            )
        );
        assert!(result.contains("alt=\"ER図\""));
        assert!(result.contains("width=\"1192\""));
        assert!(result.contains("height=\"861\""));
        assert!(result.contains("loading=\"lazy\""));
        assert!(result.contains("data-original-url"));
    }

    #[test]
    fn test_transform_html_img_tag_no_github_assets() {
        let html = r#"<img src="https://example.com/image.png" alt="External" />"#;
        let asset_map = HashMap::new();

        let result = transform_html_img_tags(html, &asset_map);

        // Should remain unchanged
        assert_eq!(result, html);
    }

    #[test]
    fn test_transform_html_img_tag_multiple_images() {
        let html = r#"
<img src="https://github.com/user-attachments/assets/6c72b402-4a5c-45cc-9b0a-50717f8a09a7" alt="First" />
<img src="https://github.com/user-attachments/assets/7d83c513-5b6d-46dd-a01b-61728e8b0a8b" alt="Second" />
"#;
        let mut asset_map = HashMap::new();
        asset_map.insert(
            "6c72b402-4a5c-45cc-9b0a-50717f8a09a7".to_string(),
            "1041-discussion-assets/6c72b402-4a5c-45cc-9b0a-50717f8a09a7.png".to_string(),
        );
        asset_map.insert(
            "7d83c513-5b6d-46dd-a01b-61728e8b0a8b".to_string(),
            "1041-discussion-assets/7d83c513-5b6d-46dd-a01b-61728e8b0a8b.jpg".to_string(),
        );

        let result = transform_html_img_tags(html, &asset_map);

        assert!(
            result.contains(
                "src=\"1041-discussion-assets/6c72b402-4a5c-45cc-9b0a-50717f8a09a7.png\""
            )
        );
        assert!(
            result.contains(
                "src=\"1041-discussion-assets/7d83c513-5b6d-46dd-a01b-61728e8b0a8b.jpg\""
            )
        );
        assert!(result.contains("alt=\"First\""));
        assert!(result.contains("alt=\"Second\""));
    }

    #[test]
    fn test_transform_markdown_image_with_local_path() {
        let text = "![ER図](https://github.com/user-attachments/assets/6c72b402-4a5c-45cc-9b0a-50717f8a09a7)";
        let mut asset_map = HashMap::new();
        asset_map.insert(
            "6c72b402-4a5c-45cc-9b0a-50717f8a09a7".to_string(),
            "1041-discussion-assets/6c72b402-4a5c-45cc-9b0a-50717f8a09a7.png".to_string(),
        );

        let result = transform_markdown_images(text, &asset_map);

        assert!(
            result.contains("](1041-discussion-assets/6c72b402-4a5c-45cc-9b0a-50717f8a09a7.png)")
        );
        assert!(result.contains("<!-- https://github.com/user-attachments/assets/6c72b402-4a5c-45cc-9b0a-50717f8a09a7 -->"));
    }

    #[test]
    fn test_transform_markdown_image_with_existing_title() {
        let text = "![ER図](https://github.com/user-attachments/assets/6c72b402-4a5c-45cc-9b0a-50717f8a09a7 \"Existing title\")";
        let mut asset_map = HashMap::new();
        asset_map.insert(
            "6c72b402-4a5c-45cc-9b0a-50717f8a09a7".to_string(),
            "1041-discussion-assets/6c72b402-4a5c-45cc-9b0a-50717f8a09a7.png".to_string(),
        );

        let result = transform_markdown_images(text, &asset_map);

        assert!(result.contains(
            "](1041-discussion-assets/6c72b402-4a5c-45cc-9b0a-50717f8a09a7.png \"Existing title\")"
        ));
        assert!(result.contains("<!-- https://github.com/user-attachments/assets/6c72b402-4a5c-45cc-9b0a-50717f8a09a7 -->"));
    }

    #[test]
    fn test_transform_markdown_image_preserves_title() {
        let text = "![ER図](https://github.com/user-attachments/assets/6c72b402-4a5c-45cc-9b0a-50717f8a09a7 \"Existing title\")";
        let mut asset_map = HashMap::new();
        asset_map.insert(
            "6c72b402-4a5c-45cc-9b0a-50717f8a09a7".to_string(),
            "1041-discussion-assets/6c72b402-4a5c-45cc-9b0a-50717f8a09a7.png".to_string(),
        );

        let result = transform_markdown_images(text, &asset_map);

        // Title should be preserved
        assert!(result.contains("\"Existing title\")"));
    }

    #[test]
    fn test_transform_markdown_image_no_github_assets() {
        let text = "![External](https://example.com/image.png)";
        let asset_map = HashMap::new();

        let result = transform_markdown_images(text, &asset_map);

        // Should remain unchanged
        assert_eq!(result, text);
    }

    #[test]
    fn test_transform_markdown_image_multiple_images() {
        let text = r#"
![Image1](https://github.com/user-attachments/assets/6c72b402-4a5c-45cc-9b0a-50717f8a09a7)
![Image2](https://github.com/user-attachments/assets/7d83c513-5b6d-46dd-a01b-61728e8b0a8b)
"#;
        let mut asset_map = HashMap::new();
        asset_map.insert(
            "6c72b402-4a5c-45cc-9b0a-50717f8a09a7".to_string(),
            "1041-discussion-assets/6c72b402-4a5c-45cc-9b0a-50717f8a09a7.png".to_string(),
        );
        asset_map.insert(
            "7d83c513-5b6d-46dd-a01b-61728e8b0a8b".to_string(),
            "1041-discussion-assets/7d83c513-5b6d-46dd-a01b-61728e8b0a8b.jpg".to_string(),
        );

        let result = transform_markdown_images(text, &asset_map);

        assert!(
            result.contains("](1041-discussion-assets/6c72b402-4a5c-45cc-9b0a-50717f8a09a7.png)")
        );
        assert!(
            result.contains("](1041-discussion-assets/7d83c513-5b6d-46dd-a01b-61728e8b0a8b.jpg)")
        );
        assert!(result.contains("<!-- https://github.com/user-attachments/assets/6c72b402-4a5c-45cc-9b0a-50717f8a09a7 -->"));
        assert!(result.contains("<!-- https://github.com/user-attachments/assets/7d83c513-5b6d-46dd-a01b-61728e8b0a8b -->"));
    }

    #[test]
    fn test_original_url_preserved_in_html_output() {
        let html = r#"<img src="https://github.com/user-attachments/assets/6c72b402-4a5c-45cc-9b0a-50717f8a09a7" alt="Diagram" />"#;
        let mut asset_map = HashMap::new();
        asset_map.insert(
            "6c72b402-4a5c-45cc-9b0a-50717f8a09a7".to_string(),
            "1041-discussion-assets/6c72b402-4a5c-45cc-9b0a-50717f8a09a7.png".to_string(),
        );

        let result = transform_html_img_tags(html, &asset_map);

        assert!(result.contains("data-original-url=\"https://github.com/user-attachments/assets/6c72b402-4a5c-45cc-9b0a-50717f8a09a7\""));
    }

    #[test]
    fn test_original_url_preserved_in_markdown_output() {
        let text = "![Diagram](https://github.com/user-attachments/assets/6c72b402-4a5c-45cc-9b0a-50717f8a09a7)";
        let mut asset_map = HashMap::new();
        asset_map.insert(
            "6c72b402-4a5c-45cc-9b0a-50717f8a09a7".to_string(),
            "1041-discussion-assets/6c72b402-4a5c-45cc-9b0a-50717f8a09a7.png".to_string(),
        );

        let result = transform_markdown_images(text, &asset_map);

        assert!(result.contains("<!-- https://github.com/user-attachments/assets/6c72b402-4a5c-45cc-9b0a-50717f8a09a7 -->"));
    }

    #[test]
    fn test_transform_discussion_body() {
        let body = r#"
<img src="https://github.com/user-attachments/assets/6c72b402-4a5c-45cc-9b0a-50717f8a09a7" alt="HTML Image" />
![Markdown Image](https://github.com/user-attachments/assets/7d83c513-5b6d-46dd-a01b-61728e8b0a8b)
"#;
        let mut asset_map = HashMap::new();
        asset_map.insert(
            "6c72b402-4a5c-45cc-9b0a-50717f8a09a7".to_string(),
            "1041-discussion-assets/6c72b402-4a5c-45cc-9b0a-50717f8a09a7.png".to_string(),
        );
        asset_map.insert(
            "7d83c513-5b6d-46dd-a01b-61728e8b0a8b".to_string(),
            "1041-discussion-assets/7d83c513-5b6d-46dd-a01b-61728e8b0a8b.jpg".to_string(),
        );

        let result = transform_discussion_body(body, &asset_map);

        // Both HTML and Markdown images should be transformed
        assert!(
            result.contains(
                "src=\"1041-discussion-assets/6c72b402-4a5c-45cc-9b0a-50717f8a09a7.png\""
            )
        );
        assert!(
            result.contains("](1041-discussion-assets/7d83c513-5b6d-46dd-a01b-61728e8b0a8b.jpg)")
        );
    }

    #[test]
    fn test_transform_comment_body() {
        let body = "![Comment Image](https://github.com/user-attachments/assets/6c72b402-4a5c-45cc-9b0a-50717f8a09a7)";
        let mut asset_map = HashMap::new();
        asset_map.insert(
            "6c72b402-4a5c-45cc-9b0a-50717f8a09a7".to_string(),
            "1041-discussion-assets/6c72b402-4a5c-45cc-9b0a-50717f8a09a7.png".to_string(),
        );

        let result = transform_comment_body(body, &asset_map);

        assert!(
            result.contains("](1041-discussion-assets/6c72b402-4a5c-45cc-9b0a-50717f8a09a7.png)")
        );
        assert!(result.contains("<!-- https://github.com/user-attachments/assets/6c72b402-4a5c-45cc-9b0a-50717f8a09a7 -->"));
    }

    #[test]
    fn test_transform_reply_body() {
        let body = "<img src=\"https://github.com/user-attachments/assets/6c72b402-4a5c-45cc-9b0a-50717f8a09a7\" alt=\"Reply\" />";
        let mut asset_map = HashMap::new();
        asset_map.insert(
            "6c72b402-4a5c-45cc-9b0a-50717f8a09a7".to_string(),
            "1041-discussion-assets/6c72b402-4a5c-45cc-9b0a-50717f8a09a7.png".to_string(),
        );

        let result = transform_reply_body(body, &asset_map);

        assert!(
            result.contains(
                "src=\"1041-discussion-assets/6c72b402-4a5c-45cc-9b0a-50717f8a09a7.png\""
            )
        );
        assert!(result.contains("data-original-url"));
    }

    #[test]
    fn test_transform_preserves_trailing_newline() {
        let text = "![Image](https://github.com/user-attachments/assets/6c72b402-4a5c-45cc-9b0a-50717f8a09a7)\n";
        let mut asset_map = HashMap::new();
        asset_map.insert(
            "6c72b402-4a5c-45cc-9b0a-50717f8a09a7".to_string(),
            "1041-discussion-assets/6c72b402-4a5c-45cc-9b0a-50717f8a09a7.png".to_string(),
        );

        let result = transform_markdown_images(text, &asset_map);

        assert!(result.ends_with('\n'));
    }
}
