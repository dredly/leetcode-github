use clap::Parser;

mod graphql_queries;
mod leetcode_api_client;
mod models;
mod repo_builder;

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
    leetcode_api_client::display_first_submission_details(&graphql_client).await;
}
