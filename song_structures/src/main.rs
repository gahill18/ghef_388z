extern crate reqwest;
extern crate select;

use scraper::{Html, Selector};

use genius_rs::Genius;
use mini_redis::Result;

use std::collections::HashMap;
use std::env;

// For given artist, returns HashMap where song titles are mapped to their linked genius page's contents
async fn get_urls_for_artist(artist: &str) -> HashMap<String, String> {
    let token = "hfCV-emITedExDDPNttLW6pWtM2IEJYD1VbVe5E3g4x6-7kr8A2NSRZaAIqe3WUz";
    let genius = Genius::new(token.to_string());
    let mut songs: HashMap<String, String> = HashMap::new();

    // search for top 10 songs of artist
    match genius.search(artist).await {
        Ok(response) => {
            // For every response, get the page at the url provided to parse
            for n in 0..response.len() {
                let title = response[n].result.full_title.clone();
                songs.insert(title, response[n].result.url.clone());
            }
        },
        Err(e) => println!("genius song query failed, returned {:?}", e),

    }

    songs
}

async fn get_lyrics_for_url(url: &str) -> HashMap<String, Vec<String>> {
    let mut output: HashMap<String, Vec<String>> = HashMap::new();

    match reqwest::get(url).await {
        Ok(response) => {
            // Gets the HTML as a string
            match response.text().await {
                Ok(body) => {
                    let fragment = Html::parse_document(&body);
                    match Selector::parse("#lyrics-root") {
                        Ok(selector) => {
                            for lyrics in fragment.select(&selector) {
                                let save_lyrics = lyrics.text().collect::<Vec<_>>();
                                println!("Saved lyrics: {:?}", save_lyrics);
                                println!("\n\n\n\n");
                            }
                        },
                        Err(e) => println!("selector failed"),
                    }
                },
                Err(e) => println!("fragment text failed"),
            }
        },
        Err(e) => println!("url reqwest failed"),
    }

    output
}

#[tokio::main]
async fn main() -> Result <()> {
    let args: Vec<String> = env::args().collect();

    let artist = &args[1];
    let artist_urls = get_urls_for_artist(artist);

    let artist_urls = artist_urls.await;
    println!("artist_urls: {:?}\n\n\n\n", artist_urls);

    for (_title, url) in artist_urls {
        get_lyrics_for_url(&url).await;
    }

    return Ok(());
}
