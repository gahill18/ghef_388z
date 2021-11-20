use genius_rs::Genius;
use mini_redis::Result;
use std::collections::HashMap;
use std::env;

// For given artist, returns HashMap where song titles are mapped to their genius page's lyrics
async fn scrape_song_lyrics_for_artist(artist: &str) -> HashMap<String, Vec<String>> {
    let token = "hfCV-emITedExDDPNttLW6pWtM2IEJYD1VbVe5E3g4x6-7kr8A2NSRZaAIqe3WUz";
    let genius = Genius::new(token.to_string());
    let mut songs: HashMap<String, Vec<String>> = HashMap::new();

    // search for top 10 songs of artist
    match genius.search(artist).await {
        Ok(response) => {
            // For every response, add the lyrics found to the HashMap
            for n in 0..response.len() {
                match genius.get_lyrics(&response[n].result.url).await {
                    Ok(lyrics) => {
                        let title = response[n].result.full_title.clone();
                        songs.insert(title, lyrics);
                    },
                    e => println!("genius song lyrics query failed, returned {:?}", e),
                }
            }
        },
        e => println!("genius song urls query failed, returned {:?}", e),

    }

    println!("method run");

    songs
}

#[tokio::main]
async fn main() -> Result <()> {
    let args: Vec<String> = env::args().collect();
    println!("args: {:?}", args);

    let artist = &args[1];
    let lyrics = scrape_song_lyrics_for_artist(artist);

    let lyrics = lyrics.await;

    println!("lyrics: {:?}", lyrics);

    return Ok(());
}
