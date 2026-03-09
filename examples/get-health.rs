use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    url: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    println!("{}", args.url);
}
