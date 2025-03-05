mod scraper;
mod utils;

use clap::Parser;
use std::error::Error;

#[derive(Parser)]
#[command(version, about = "A simple Rust web scraper")]
struct Args {
    #[arg(short, long)]
    url: String,

    #[arg(short, long, default_value = "results.md")]
    output: String,

    #[arg(short, long, default_value_t = 1)]
    depth: usize,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    println!("Scraping: {}", args.url);
    println!("Output file: {}", args.output);
    println!("Depth: {}", args.depth);

    let scraper = scraper::Scraper::new();
    let scraped_data = scraper.scrape(&args.url, args.depth).await?;

    utils::save_to_file(&args.output, &scraped_data)?;

    println!("Scraping complete! Results saved to {}", args.output);

    Ok(())
}
