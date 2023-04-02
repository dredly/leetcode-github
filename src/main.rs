use clap::Parser;

mod leetcode_api_client;

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
    leetcode_api_client::display_submissions().await;
}
