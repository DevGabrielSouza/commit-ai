mod cli;
mod git;
mod openai;
mod commit;

use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let api_key = env::var("OPENAI_API_KEY").expect("API key not found in environment");

    let args = cli::parse_args();

    match git::get_git_changes() {
        Ok(diff) => {
            match openai::generate_commit_message(&diff, &api_key).await {
                Ok(commit_message) => {
                    println!("Generated Commit Message:\n{}", commit_message);
                    if args.auto_commit {
                        commit::create_commit(&commit_message).unwrap_or_else(|e| {
                            eprintln!("Failed to create commit: {}", e);
                        });
                    }
                }
                Err(e) => eprintln!("Failed to generate commit message: {}", e),
            }
        }
        Err(e) => eprintln!("Failed to get git diff: {}", e),
    }
}
