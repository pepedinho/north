use std::io;

use clap::Parser;
use tokio;

use crate::{display::display::display_repos, requests::requests::GitHubClient};

mod display;
mod parsing;
mod requests;

#[derive(Parser)]
struct Cli {
    repo_name: String,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let client = GitHubClient::new();
    let args = Cli::parse();
    let repos = match client.search_repos(&args.repo_name).await {
        Ok(repos) => repos,
        Err(e) => {
            eprintln!("Failed to search repositories: {}", e);
            return Ok(());
        }
    };

    let _ = display_repos(repos, client).await;
    Ok(())
}
