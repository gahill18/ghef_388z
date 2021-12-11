extern crate reqwest;
extern crate select;

use scraper::{Html, Selector};
use genius_rs::Genius;
use mini_redis::Result;

use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

fn parse_text_main(artist: &str, cat: &str) -> std::io::Result<()> {
    let artist_name = String::from(artist).to_lowercase().replace(" ", "_");
    let mut artist_path = String::from("./lyrics/");
    artist_path.push_str(artist_name.as_str());
    let paths = fs::read_dir(artist_path.as_str()).unwrap();
    let category = String::from(cat).to_lowercase().replace(" ", "_");

    let mut artist = Artist {
        name : String::from(&artist_name),
        songs : Vec::new(),
        category : String::from(category),
    };

    let mut contents = String::new();

    for path in paths {
        let mut file = File::open(path.unwrap().path())?;
        contents.push_str("__TITLE__");
        file.read_to_string(&mut contents)?;
    }

    //print!("contents was:{}\n\n",contents);

    // need to make sure when text was read in that songs were separated by "__TITLE__"
    let mut split_songs = contents.split("__TITLE__");
    let songs: Vec<&str> = split_songs.collect();

    for text in songs {

        let mut split_lines = text.split("\n");
        let lines: Vec<&str> = split_lines.collect();

        let song_title = String::from(*lines.first().unwrap());

        let mut song = Song {
            artist : String::from(&artist_name),
            title : song_title, 
            structures : Vec::new(), 
        };

        song.parse_text(&text);

        song.print_lyrics();
    }
    Ok(())
}


// stores the struct_type of the structure and the lines contained in that struct
struct Structure<'a> {
    struct_type: String,
    lines: Vec<&'a str>,
}

impl<'a> Structure<'a> {
    fn print_lines(&self) {
        print!("Structure type: {}\n",self.struct_type);
        for line in &self.lines {
            print!("{}\n", line);
        }
    }

    fn new(mut raw_lines : Vec<&'a str>) -> Structure {
        let struct_types = vec!["Intro","Verse","Chorus","Bridge","Pre-Chorus","Post-Chorus","Outro","Refrain","Instrumental","Solo","Other"];

        let struct_line = String::from(raw_lines.remove(0)).to_lowercase();
        let mut struct_type = String::from("Other");
        for s_type in struct_types {
            if struct_line.contains(&s_type.to_lowercase()) {
                struct_type = String::from(s_type);
            }
        }

        Structure {
            struct_type : struct_type,
            lines : raw_lines.clone(),
        }
    }
}

// stores the artist of the song, the title of the song, and the structures of the song in order
struct Song<'a> {
    artist: String,
    title: String,
    structures : Vec<Structure<'a>>,
}

impl<'a> Song<'a> {

    // make sure structures is already intialized to an empty vector before calling
    fn parse_text(&mut self, text: &'a str) {
        
        let mut split_structures = text.split("[");
        let mut raw_structures: Vec<&str> = split_structures.collect();

        for raw_structure in raw_structures {
            let split_lines = raw_structure.split("\n");
            let raw_lines: Vec<&str> = split_lines.collect();

            self.structures.push(Structure::new(raw_lines))
        }
    }

    fn print_lyrics(&self) {
        for structure in &self.structures {
            structure.print_lines();
        }
        print!("\n\n")
    }
}

struct Artist<'a> {
    name: String,
    songs: Vec<Song<'a>>,
    category: String,
}


/*
 * end of parser
 */












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

// Takes in a genius url and a write path, gets the lyrics from the url,
// then writes to the file at write_location
async fn write_lyrics_from_urls(url: &str, song_title: &str, artist: &str) {
    match reqwest::get(url).await {
        Ok(response) => {
            // Gets the HTML as a string
            match response.text().await {
                Ok(page) => {
                    let fragment = Html::parse_document(&page);
                    match Selector::parse("#lyrics-root") {
                        Ok(selector) => {
                            for lyrics in fragment.select(&selector) {
                                let mut save_lyrics = lyrics.text().map(String::from).collect::<Vec<_>>();
                                
                                let mut filename = "./lyrics/".to_owned();
                                filename.push_str(artist);
                                filename.push_str("/");
                                filename.push_str(song_title);
                                filename.push_str(".txt");
                                
                                //fs::write(filename, "hello world").expect("Unable to write file");
                                //println!("lyrics before writing: {:?}\n\n", save_lyrics); // modify this line for file I/O
                                let mut f = File::create(filename).expect("Unable to create file");

                                let mut i = 0;
                                while i < save_lyrics.len() {
                                    if save_lyrics[i].contains("(") && i + 2 < save_lyrics.len() {
                                        if save_lyrics[i+2].contains(")") && !save_lyrics[i+2].contains("(") {
                                            save_lyrics[i] = format!("{}{}{}", save_lyrics[i], save_lyrics[i+1], save_lyrics[i+2]);
                                            save_lyrics[i+1] = format!("{}__REMOVE_LINE__",save_lyrics[i+1]);
                                            save_lyrics[i+2] = format!("{}__REMOVE_LINE__",save_lyrics[i+2]);
                                        }
                                    }
                                    if save_lyrics[i].contains("[") && (! save_lyrics[i].contains("]")) && i + 2 < save_lyrics.len() {
                                        if save_lyrics[i+2].contains("]") {
                                            save_lyrics[i] = format!("{}{}{}", save_lyrics[i], save_lyrics[i+1], save_lyrics[i+2]);
                                            save_lyrics[i+1] = format!("{}__REMOVE_LINE__",save_lyrics[i+1]);
                                            save_lyrics[i+2] = format!("{}__REMOVE_LINE__",save_lyrics[i+2]);
                                        }
                                    }
                                    i += 1;
                                }


                                for line in &save_lyrics[0..save_lyrics.len() - 6] { // - 6 is for the last 6 parts of lyrics page that aren't actually lyrics

                                    if ! (*line).contains("__REMOVE_LINE__")  {  
                                        f.write_all(line.as_bytes()).expect("Unable to write line");
                                        f.write_all("\n".as_bytes()).expect("Unable to write new line");
                                    }
                                    else {
                                        println!(" FOUND MATCHES FOR WEIRD ITALICS ISSUE WITH NEWLINES ")
                                    } 
                                }
                            }
                            
                        },
                        Err(e) => println!("selector failed: {:?}", e),
                    }
                },
                Err(e) => println!("fragment text failed: {:?}", e),
            }
        },
        Err(e) => println!("url reqwest failed: {:?}", e),
    }
}

#[tokio::main]
async fn main() -> Result <()> {
    fs::create_dir("./lyrics");
    let args: Vec<String> = env::args().collect();

    let artist = &args[1].to_lowercase().replace(" ", "_");
    let artist_urls = get_urls_for_artist(&artist);

    let artist_urls = artist_urls.await;
    println!("artist_urls: {:?}\n\n\n\n", artist_urls);

    // let write_path = &args[2];

    for (raw_title, url) in artist_urls {
        let mut filename = "./lyrics/".to_owned();
        filename.push_str(&artist);
        fs::create_dir(filename);
        // title will be the song title, which is ended by the word "by\u{a0}" in the url which is used for extracting it
        let title = &(raw_title[0..raw_title.find("by\u{a0}").unwrap() - 1].to_lowercase().replace(" ", "_")); // changes spaces in the song title to be _
        // print!("title was: {:?}", title);
        // print!("raw_title was: {:?}", raw_title);
        write_lyrics_from_urls(&url, title, artist).await;
        parse_text_main(artist, "test");
    }

    return Ok(());
}
