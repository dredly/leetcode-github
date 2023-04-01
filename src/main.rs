use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    output: String
}

fn main() {
    let args = Args::parse();
    println!("Selected output path {}", args.output);
}
