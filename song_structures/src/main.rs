use genius_rs::Genius;
extern crate reqwest; 
use mini_redis::Result;
use std::collections::HashMap;
use std::env;

// For given artist, returns HashMap where song titles are mapped to their linked genius page's contents
async fn scrape_pages_for_artist(artist: &str) -> HashMap<String, String> {
    let token = "hfCV-emITedExDDPNttLW6pWtM2IEJYD1VbVe5E3g4x6-7kr8A2NSRZaAIqe3WUz";
    let genius = Genius::new(token.to_string());
    let mut songs: HashMap<String, String> = HashMap::new();

    // search for top 10 songs of artist
    match genius.search(artist).await {
        Ok(response) => {
            // For every response, get the page at the url provided to parse
            for n in 0..response.len() {
                match reqwest::get(&response[n].result.url).await {
                    Ok(page) => {
                        match page.text().await {
                            Ok(lyrics) => {
                                let title = response[n].result.full_title.clone();
                                songs.insert(title, lyrics);
                            },
                            e => println!("genius page text query failed, returned {:?}", e),
                        };
                    },
                    e => println!("genius song url query failed, returned {:?}", e),
                }
            }
        },
        e => println!("genius song query failed, returned {:?}", e),

    }

    println!("method run");

    songs
}

fn get_lyrics_from_page(page_contents: String) -> Vec<String> {
    let mut output = Vec::new();
    let mut processed_page = page_contents;
    output.insert(0, processed_page);

    output
}

fn get_lyrics_for_pages (map: &mut HashMap<String, String>) -> HashMap<String, Vec<String>>{
    let mut output: HashMap<String, Vec<String>> = HashMap::new();
    for (song_title, page_contents) in &*map {
        output.insert(song_title.to_string(), get_lyrics_from_page(page_contents.to_string()));
    }

    output
}

#[tokio::main]
async fn main() -> Result <()> {
    let args: Vec<String> = env::args().collect();
    println!("args: {:?}", args);

    let artist = &args[1];
    let pages = scrape_pages_for_artist(artist);

    let mut pages = pages.await;
    println!("pages: {:?}", pages);

    let lyrics = get_lyrics_for_pages(&mut pages);
    println!("lyrics: {:?}", lyrics);

    return Ok(());
}
