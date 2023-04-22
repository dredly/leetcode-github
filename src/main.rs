#[macro_use]
extern crate lazy_static;

use clap::Parser;
use std::collections::HashMap;

mod graphql_queries;
mod leetcode_api_client;
mod models;
mod repo_builder;
mod utils;

lazy_static! {
    static ref LANGUAGE_NAMES_TO_EXTENSIONS: HashMap<&'static str, &'static str> = HashMap::from([
        ("rust", "rs"),
        ("python", "py"),
        ("python3", "py"),
        ("elixir", "ex")
    ]);
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    output: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    println!("Selected output path {}", args.output);
    let graphql_client = leetcode_api_client::get_graphql_client().await;
    let submission_details = leetcode_api_client::get_all_submission_details(&graphql_client).await;
    repo_builder::initialise_repo(&args.output).expect("Big time fail");

    for submission_detail in &submission_details {
        repo_builder::add_submission_to_repo(&args.output, submission_detail)
            .expect("BIG TIME FAIL");
    }

    println!(
        "Found details for {} accepted solutions",
        submission_details.len()
    );
}
