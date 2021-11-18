use genius_rs::Genius;
use mini_redis::{client, Result};
use std::env;


async fn scrape_artist(artist: &str) {
    let token = "iD4KBNaCN84dVIvRTsT7aUZS2ZvJbXEqI0CfARtb96RfsEAbliPg0ZdW1ObFCLb7";
    let genius = Genius::new(token.to_string());

    match genius.search(artist).await {
        Ok(response) => {

            println!("{:?}", response[0].result.full_title.clone());
        },
        e => println!("genius query failed, returned {:?}", e),

    }
}

#[tokio::main]
async fn main() -> Result <()> {
    let args: Vec<String> = env::args().collect();
    println!("args: {:?}", args);

    let query = &args[1];

    let artist_results = scrape_artist(query);

    artist_results.await;

    return Ok(());
}
