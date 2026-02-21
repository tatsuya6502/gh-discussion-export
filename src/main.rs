use std::collections::HashMap;
use std::path::Path;

use clap::Parser;
use gh_discussion_export::assets::{
    dedupe_asset_urls, detect_asset_urls, detect_markdown_assets, download_assets_parallel,
};
use gh_discussion_export::cli::CliArgs;
use gh_discussion_export::client::ReqwestClient;
use gh_discussion_export::fetch::fetch_discussion;
use gh_discussion_export::output::{format_discussion, write_output};

fn main() {
    // Parse command-line arguments
    let args = CliArgs::parse();

    // Extract owner, repo, number from arguments
    let (owner, repo) = match args.repo_components() {
        Ok(components) => components,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };
    let number = args.number;

    // Determine output path (use arg value or default to `<number>-discussion.md`)
    let output_path = args.output_path();

    // Get GitHub token
    let token = match gh_discussion_export::auth::get_github_token() {
        Ok(token) => token,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    // Create GitHub client (keep ReqwestClient for asset downloads)
    let reqwest_client = match ReqwestClient::new(token.clone()) {
        Ok(client) => client,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };
    let client = gh_discussion_export::client::GitHubClient::new(Box::new(reqwest_client.clone()));

    // Fetch discussion
    let discussion = match fetch_discussion(&client, &owner, &repo, number) {
        Ok(discussion) => discussion,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    // Build asset_map if downloading assets
    let asset_map: Option<HashMap<String, String>> = if args.should_download_assets() {
        // Collect all asset URLs from discussion body, comments, and replies
        let mut all_urls = Vec::new();

        // Detect assets in discussion body
        all_urls.extend(detect_asset_urls(&discussion.body));
        all_urls.extend(detect_markdown_assets(&discussion.body));

        // Detect assets in comments and replies
        if let Some(ref comments) = discussion.comments.nodes {
            for comment in comments.iter().flatten() {
                all_urls.extend(detect_asset_urls(&comment.body));
                all_urls.extend(detect_markdown_assets(&comment.body));

                if let Some(ref replies) = comment.replies.nodes {
                    for reply in replies.iter().flatten() {
                        all_urls.extend(detect_asset_urls(&reply.body));
                        all_urls.extend(detect_markdown_assets(&reply.body));
                    }
                }
            }
        }

        // Deduplicate URLs
        let unique_urls = dedupe_asset_urls(all_urls);

        if unique_urls.is_empty() {
            // No assets detected, skip directory creation
            None
        } else {
            // Create asset directory in the same directory as the output file
            let asset_dir_name = args.asset_dir_name();
            let output_parent = Path::new(&output_path).parent().unwrap_or(Path::new("."));
            let asset_dir = output_parent.join(&asset_dir_name);

            if let Err(e) = std::fs::create_dir_all(&asset_dir) {
                eprintln!(
                    "Error: Failed to create asset directory '{}': {}",
                    asset_dir_name, e
                );
                std::process::exit(1);
            }

            // Download assets
            let download_results = download_assets_parallel(
                reqwest_client.client(),
                &token,
                unique_urls.clone(),
                &asset_dir,
                args.parallel,
            );

            // Count successes and failures
            let success_count = download_results.iter().filter(|r| r.result.is_ok()).count();
            let failure_count = download_results
                .iter()
                .filter(|r| r.result.is_err())
                .count();

            // Print warnings for failed downloads
            for result in &download_results {
                if let Err(e) = &result.result {
                    // Task 11.6: Provide clear message for 401 errors
                    if matches!(e, gh_discussion_export::error::Error::Authentication) {
                        eprintln!(
                            "Error: Authentication failed for asset '{}'. Please run `gh auth login` to authenticate.",
                            result.url
                        );
                    } else {
                        eprintln!("Warning: Failed to download asset '{}': {}", result.url, e);
                    }
                }
            }

            // Build asset_map from UUID to local path (only successful downloads)
            let mut map = HashMap::new();
            for result in &download_results {
                if result.result.is_ok() {
                    let local_path =
                        format!("{}/{}{}", asset_dir_name, result.uuid, result.extension);
                    map.insert(result.uuid.clone(), local_path);
                }
            }

            // Print summary
            println!(
                "Downloaded {} asset(s) to: {}",
                success_count, asset_dir_name
            );
            if failure_count > 0 {
                println!("Warning: {} asset(s) failed to download", failure_count);
            }

            Some(map)
        }
    } else {
        None
    };

    // Generate Markdown output (with asset transformation if asset_map is provided)
    let markdown = format_discussion(&discussion, &owner, &repo, asset_map.as_ref());

    // Write output file
    match write_output(&markdown, &output_path) {
        Ok(()) => {}
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    // Print success message
    println!("Discussion exported to: {}", output_path);
}
