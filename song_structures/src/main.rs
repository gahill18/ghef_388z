use genius_rs::Genius;
use mini_redis::Result;
use std::collections::HashMap;
use std::env;

// For given artist, returns HashMap where song titles are mapped to their genius url's
async fn scrape_song_urls_for_artist(artist: &str) -> HashMap<String, String> {
    let token = "iD4KBNaCN84dVIvRTsT7aUZS2ZvJbXEqI0CfARtb96RfsEAbliPg0ZdW1ObFCLb7";
    let genius = Genius::new(token.to_string());
    let mut songs: HashMap<String, String> = HashMap::new();

    match genius.search(artist).await {
        Ok(response) => {
            for n in 0..response.len() {
                songs.insert(response[n].result.full_title.clone(), response[n].result.url.clone());
            }
        },
        e => println!("genius song urls query failed, returned {:?}", e),

    }

    songs
}

async fn get_lyrics_for_songs(songs: HashMap<String, String>) -> HashMap<String, Vec<String>> {
    let token = "iD4KBNaCN84dVIvRTsT7aUZS2ZvJbXEqI0CfARtb96RfsEAbliPg0ZdW1ObFCLb7";
    let genius = Genius::new(token.to_string());
    let mut song_lyrics: HashMap<String, Vec<String>> = HashMap::new();

    for (title, url) in songs.iter() {
        match genius.get_lyrics(url).await {
            Ok(response) => {
                song_lyrics.insert(title.to_string(), response.clone());
            },
            e => println!("genius song lyrics query failed, returned {:?}", e),
        }
    }

    song_lyrics
}

#[tokio::main]
async fn main() -> Result <()> {
    let args: Vec<String> = env::args().collect();
    println!("args: {:?}", args);

    let artist = &args[1];

    let songs = scrape_song_urls_for_artist(artist);
    let songs = songs.await;

    println!("For artist {:?}, found songs {:?}", artist, songs);

    let lyrics = get_lyrics_for_songs(songs);
    let lyrics = lyrics.await;

    println!("Lyrics: {:?}", lyrics);

    return Ok(());
}
