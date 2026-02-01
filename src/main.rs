use clap::Parser;
use gh_discussion_export::cli::CliArgs;
use gh_discussion_export::client::ReqwestClient;
use gh_discussion_export::error::Result;
use gh_discussion_export::fetch::fetch_discussion;
use gh_discussion_export::output::{format_discussion, write_output};

fn main() -> Result<()> {
    // Parse command-line arguments
    let args = CliArgs::parse();

    // Extract owner, repo, number from arguments
    let owner = &args.owner;
    let repo = &args.repo;
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

    // Create GitHub client
    let http_client = Box::new(ReqwestClient::new(token)?);
    let client = gh_discussion_export::client::GitHubClient::new(http_client);

    // Fetch discussion
    let discussion = match fetch_discussion(&client, owner, repo, number) {
        Ok(discussion) => discussion,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    // Generate Markdown output
    let markdown = format_discussion(&discussion, owner, repo);

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

    Ok(())
}
