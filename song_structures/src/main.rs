use genius_rs::Genius;
//use threadpool::ThreadPool;
use mini_redis::{client, Result};
//use std::future::Future;
//use futures::executor::LocalPool;
//use std::pin::Pin;


async fn scrape_artist(artist: &str) {
    let token = "tu4mFXq-j8GlG9mqZQlEBb0yeekm_zC5A3-mt2RxYVF3qTAPrLybFl_ykJ7Fk_E5-dyQ7bMCXHNSbaYjCOQ47g";
    let genius = Genius::new(token.to_string());

    println!("test2");

    match genius.search(artist).await {
        Ok(response) => println!("{:?}", response[0].result.full_title.clone()),
        _ => println!("genius query failed"),

    }
}

#[tokio::main]
async fn main() -> Result <()> {
    let mut client = client::connect("127.0.0.1:6379").await?;

    let artist_results = scrape_artist("test");

    artist_results.await;

    return Ok(());
}
