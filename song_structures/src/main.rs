extern crate regex;
extern crate reqwest;
extern crate select;

use genius_rs::Genius;
use mini_redis::Result;
use scraper::{Html, Selector};

use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

use regex::Regex;

fn parse_text_main(artist: &str, cat: &str, print: bool) -> std::io::Result<()> {
    let artist_name = String::from(artist).to_lowercase().replace(" ", "_");
    let mut artist_path = String::from("./lyrics/");
    artist_path.push_str(artist_name.as_str());
    let paths = fs::read_dir(artist_path.as_str()).unwrap();
    let category = String::from(cat).to_lowercase().replace(" ", "_");

    let mut artist = Artist {
        name: String::from(&artist_name),
        songs: Vec::new(),
        category: String::from(category),
    };

    let mut contents = String::new();

    for path in paths {
        let mut file = File::open(path.unwrap().path())?;
        contents.push_str("__TITLE__");
        file.read_to_string(&mut contents)?;
    }

    //print!("contents was:{}\n\n",contents);

    // need to make sure when text was read in that songs were separated by "__TITLE__"
    let split_songs = contents.split("__TITLE__");
    let songs: Vec<&str> = split_songs.collect();

    for text in songs {
        let split_lines = text.split("\n");
        let lines: Vec<&str> = split_lines.collect();

        let song_title = String::from(*lines.first().unwrap());

        let mut song = Song {
            artist: String::from(&artist_name),
            title: song_title,
            structures: Vec::new(),
        };

        song.parse_text(&text);
        artist.songs.push(song);
        //song.print_lyrics();
    }

    let summary = Summary::create(artist);
    if print {
        summary.print();
    }

    Ok(())
}

fn parse_text_main_compare(artist_name1: &str, artist_name2: &str) -> std::io::Result<()> {
    let artist1_name = String::from(artist_name1).to_lowercase().replace(" ", "_");
    let mut artist1_path = String::from("./lyrics/");
    artist1_path.push_str(artist1_name.as_str());
    print!("artist1_path got stored as:{}", artist1_path);
    let paths1 = fs::read_dir(artist1_path.as_str()).unwrap();

    let mut artist1 = Artist {
        name: String::from(&artist1_name),
        songs: Vec::new(),
        category: String::from("unspecified"),
    };

    let mut contents1 = String::new();

    for path in paths1 {
        let mut file = File::open(path.unwrap().path())?;
        contents1.push_str("__TITLE__");
        file.read_to_string(&mut contents1)?;
    }

    // need to make sure when text was read in that songs were separated by "__TITLE__"
    let split_songs1 = contents1.split("__TITLE__");
    let songs1: Vec<&str> = split_songs1.collect();

    for text in songs1 {
        let split_lines = text.split("\n");
        let lines: Vec<&str> = split_lines.collect();

        let song_title = String::from(*lines.first().unwrap());

        let mut song = Song {
            artist: String::from(&artist1_name),
            title: song_title,
            structures: Vec::new(),
        };

        song.parse_text(&text);
        artist1.songs.push(song);
        //song.print_lyrics();
    }

    let artist2_name = String::from(artist_name2).to_lowercase().replace(" ", "_");
    let mut artist2_path = String::from("./lyrics/");
    artist2_path.push_str(artist2_name.as_str());
    let paths2 = fs::read_dir(artist2_path.as_str()).unwrap();

    let mut artist2 = Artist {
        name: String::from(&artist2_name),
        songs: Vec::new(),
        category: String::from("unspecified"),
    };

    let mut contents2 = String::new();

    for path in paths2 {
        let mut file = File::open(path.unwrap().path())?;
        contents2.push_str("__TITLE__");
        file.read_to_string(&mut contents2)?;
    }

    // need to make sure when text was read in that songs were separated by "__TITLE__"
    let split_songs2 = contents2.split("__TITLE__");
    let songs2: Vec<&str> = split_songs2.collect();

    for text in songs2 {
        let split_lines = text.split("\n");
        let lines: Vec<&str> = split_lines.collect();

        let song_title = String::from(*lines.first().unwrap());

        let mut song = Song {
            artist: String::from(&artist2_name),
            title: song_title,
            structures: Vec::new(),
        };

        song.parse_text(&text);
        artist2.songs.push(song);
        //song.print_lyrics();
    }

    let summary1 = Summary::create(artist1);
    let summary2 = Summary::create(artist2);

    Summary::compare_print(summary1, summary2);
    Ok(())
}

// stores the struct_type of the structure and the lines contained in that struct
#[derive(Clone)]
struct Structure<'a> {
    struct_type: String,
    lines: Vec<&'a str>,
}

impl<'a> Structure<'a> {
    fn new(mut raw_lines: Vec<&'a str>) -> Structure {
        let struct_types = vec![
            "Intro",
            "Verse",
            "Chorus",
            "Bridge",
            "Pre-Chorus",
            "Post-Chorus",
            "Outro",
            "Refrain",
            "Instrumental",
            "Solo",
            "Other",
        ];

        let struct_line = String::from(raw_lines.remove(0)).to_lowercase();
        let mut struct_type = String::from("Other");
        for s_type in struct_types {
            if struct_line.contains(&s_type.to_lowercase()) {
                if struct_line.contains("refrain") {
                    // they changed their terminology
                    struct_type = String::from("Chorus")
                } else {
                    struct_type = String::from(s_type);
                }
            }
        }

        Structure {
            struct_type: struct_type,
            lines: raw_lines.clone(),
        }
    }

    fn get_num_words(&self) -> u32 {
        return self.get_words().len() as u32;
    }

    fn get_words(&self) -> Vec<String> {
        let mut words = Vec::new();
        let re = Regex::new(r"([0-9a-zA-Z]+['-]*[0-9a-zA-Z]*['-]*)+").unwrap();
        for line in &self.lines {
            for cap in re.captures_iter(line) {
                words.push(String::from(&cap[0]));
            }
        }
        words
    }

    fn get_avg_word_len(&self) -> (u32, u32) {
        let mut sum_word_len = 0;
        let mut num_words = 0;
        for word in &self.get_words() {
            sum_word_len += word.chars().count();
            num_words += 1;
        }

        (sum_word_len as u32, num_words as u32)
    }
}

// stores the artist of the song, the title of the song, and the structures of the song in order
#[derive(Clone)]
struct Song<'a> {
    artist: String,
    title: String,
    structures: Vec<Structure<'a>>,
}

impl<'a> Song<'a> {
    // make sure structures is already intialized to an empty vector before calling
    fn parse_text(&mut self, text: &'a str) {
        let split_structures = text.split("[");
        let raw_structures: Vec<&str> = split_structures.collect();

        for raw_structure in raw_structures {
            let split_lines = raw_structure.split("\n");
            let raw_lines: Vec<&str> = split_lines.collect();

            let new_struct = Structure::new(raw_lines);
            if new_struct.lines.len() > 1 {
                self.structures.push(new_struct);
            }
        }
    }

    fn get_num_words(&self) -> u32 {
        let mut num_words = 0;
        for structure in &self.structures {
            num_words += structure.get_num_words();
        }
        num_words
    }

    fn get_avg_word_len(&self) -> (u32, u32) {
        let mut total_num_words = 0;
        let mut total_word_len = 0;
        for structure in &self.structures {
            let (total_struct_word_len, num_words) = structure.get_avg_word_len();
            total_word_len += total_struct_word_len;
            total_num_words += num_words;
        }

        (total_word_len, total_num_words)
    }

    fn get_total_num_lines(&self) -> u32 {
        let mut num_lines = 0;
        for structure in &self.structures {
            num_lines += structure.lines.len();
        }
        num_lines as u32
    }

    fn get_avg_lines_per_struct(self, s_type: String) -> (u32, u32) {
        let mut total_num_structs = 0;
        let mut total_num_lines = 0;

        for structure in &self.structures {
            if structure.struct_type.contains(&s_type) {
                if !(s_type.eq("Chorus")
                    && (structure.struct_type.contains("Pre")
                        || structure.struct_type.contains("Post")))
                {
                    total_num_structs += 1;
                    total_num_lines += structure.lines.len();
                }
            }
        }

        (total_num_lines as u32, total_num_structs as u32)
    }

    fn get_avg_words_per_struct(self, s_type: String) -> (u32, u32) {
        let mut total_num_structs = 0;
        let mut total_num_words = 0;

        for structure in &self.structures {
            if structure.struct_type.contains(&s_type) {
                if !(s_type.eq("Chorus")
                    && (structure.struct_type.contains("Pre")
                        || structure.struct_type.contains("Post")))
                {
                    total_num_structs += 1;
                    total_num_words += structure.get_num_words();
                }
            }
        }

        (total_num_words as u32, total_num_structs as u32)
    }
}

#[derive(Clone)]
struct Artist<'a> {
    name: String,
    songs: Vec<Song<'a>>,
    category: String,
}

impl<'a> Artist<'a> {
    fn get_total_num_words(self) -> u32 {
        let mut num_words = 0;
        for song in &self.songs {
            num_words += song.get_num_words();
        }
        num_words
    }

    fn get_avg_word_len(self) -> f64 {
        let mut total_num_words = 0;
        let mut total_word_len = 0;
        for song in &self.songs {
            let (total_song_word_len, num_words) = song.get_avg_word_len();
            total_word_len += total_song_word_len;
            total_num_words += num_words;
        }

        (total_word_len as f64) / (total_num_words as f64)
    }

    fn get_total_num_lines(self) -> u32 {
        let mut num_lines = 0;

        for song in &self.songs {
            num_lines += song.get_total_num_lines();
        }
        num_lines as u32
    }

    fn get_avg_lines_per_struct(self, s_type: String) -> f64 {
        let mut total_num_structs = 0;
        let mut total_num_lines = 0;

        for song in &self.songs {
            let (total_song_lines, num_structs) =
                song.clone().get_avg_lines_per_struct(s_type.clone());
            total_num_lines += total_song_lines;
            total_num_structs += num_structs;
        }

        (total_num_lines as f64) / (total_num_structs as f64)
    }

    fn get_avg_words_per_struct(self, s_type: String) -> f64 {
        let mut total_num_structs = 0;
        let mut total_num_words = 0;

        for song in &self.songs {
            let (total_song_words, num_structs) =
                song.clone().get_avg_words_per_struct(s_type.clone());

            total_num_words += total_song_words;
            total_num_structs += num_structs;
        }

        (total_num_words as f64) / (total_num_structs as f64)
    }
}

/*
 * end of parser
 */

/*
 * code for analysis
 */
struct Summary {
    artist: String,
    num_songs: u32,
    total_num_words: u32,
    avg_word_len: f64,
    total_num_lines: u32,
    avg_lines_per_intro: f64,
    avg_lines_per_pre_chorus: f64,
    avg_lines_per_chorus: f64,
    avg_lines_per_verse: f64,
    avg_words_per_intro: f64,
    avg_words_per_pre_chorus: f64,
    avg_words_per_chorus: f64,
    avg_words_per_verse: f64,
}

impl Summary {
    fn create(artist: Artist) -> Summary {
        Summary {
            artist: artist.name.clone(),
            num_songs: artist.songs.len() as u32,
            total_num_words: artist.clone().get_total_num_words(),
            avg_word_len: artist.clone().get_avg_word_len(),
            total_num_lines: artist.clone().get_total_num_lines(),
            avg_lines_per_intro: artist
                .clone()
                .get_avg_lines_per_struct(String::from("Intro")),
            avg_lines_per_pre_chorus: artist
                .clone()
                .get_avg_lines_per_struct(String::from("Pre-Chorus")),
            avg_lines_per_chorus: artist
                .clone()
                .get_avg_lines_per_struct(String::from("Chorus")),
            avg_lines_per_verse: artist
                .clone()
                .get_avg_lines_per_struct(String::from("Verse")),
            avg_words_per_intro: artist
                .clone()
                .get_avg_words_per_struct(String::from("Intro")),
            avg_words_per_pre_chorus: artist
                .clone()
                .get_avg_words_per_struct(String::from("Pre-Chorus")),
            avg_words_per_chorus: artist
                .clone()
                .get_avg_words_per_struct(String::from("Chorus")),
            avg_words_per_verse: artist
                .clone()
                .get_avg_words_per_struct(String::from("Verse")),
        }
    }

    fn print(self) {
        print!("Artist: {}\nNumber of songs analyzed: {}\nTotal number of words: {}
        \n Average number of words per song: {}\nAverage word length: {}
        \nTotal number of lines: {}\nAverage lines per song: {}
        \nAverage lines per structure: Intro = {}, Verse = {}, Pre-Chorus = {}, Chorus = {}
        \nAverage words per structure: Intro = {}, Verse = {}, Pre-Chorus = {}, Chorus = {}
        \n", self.artist, self.num_songs, self.total_num_words, 
        self.total_num_words as f64 / self.num_songs as f64, self.avg_word_len, 
        self.total_num_lines, self.total_num_lines as f64 / self.num_songs as f64, 
        self.avg_lines_per_intro, self.avg_lines_per_verse, self.avg_lines_per_pre_chorus, 
        self.avg_lines_per_chorus, self.avg_words_per_intro, self.avg_words_per_verse, 
        self.avg_words_per_pre_chorus, self.avg_words_per_chorus);
    }

    fn compare_print(summary1: Summary, summary2: Summary) {
        let artist1 = summary1.artist;
        let artist2 = summary2.artist;
        println!("Comparing {} and {}:", artist1, artist2);
        if (summary1.total_num_words as f64 / summary1.num_songs as f64)
            > (summary2.total_num_words as f64 / summary2.num_songs as f64)
        {
            println!(
                "Average number of words per song: {} = {} > {} = {}",
                artist1,
                (summary1.total_num_words as f64 / summary1.num_songs as f64),
                artist2,
                (summary2.total_num_words as f64 / summary2.num_songs as f64)
            );
        } else {
            println!(
                "Average number of words per song: {} = {} > {} = {}",
                artist2,
                (summary2.total_num_words as f64 / summary2.num_songs as f64),
                artist1,
                (summary1.total_num_words as f64 / summary1.num_songs as f64)
            );
        }
        if summary1.avg_word_len > summary2.avg_word_len {
            println!(
                "Average number of words per song: {} = {} > {} = {}",
                artist1, summary1.avg_word_len, artist2, summary2.avg_word_len
            );
        } else {
            println!(
                "Average number of words per song: {} = {} > {} = {}",
                artist2, summary2.avg_word_len, artist1, summary1.avg_word_len
            );
        }
        if summary1.avg_word_len > summary2.avg_word_len {
            println!(
                "Average word length: {} = {} > {} = {}",
                artist1, summary1.avg_word_len, artist2, summary2.avg_word_len
            );
        } else {
            println!(
                "Average word length: {} = {} > {} = {}",
                artist2, summary2.avg_word_len, artist1, summary1.avg_word_len
            );
        }
        if (summary1.total_num_lines as f64 / summary1.num_songs as f64)
            > (summary2.total_num_lines as f64 / summary2.num_songs as f64)
        {
            println!(
                "Average number of lines per song: {} = {} > {} = {}",
                artist1,
                (summary1.total_num_lines as f64 / summary1.num_songs as f64),
                artist2,
                (summary2.total_num_lines as f64 / summary2.num_songs as f64)
            );
        } else {
            println!(
                "Average number of lines per song: {} = {} > {} = {}",
                artist2,
                (summary2.total_num_lines as f64 / summary2.num_songs as f64),
                artist1,
                (summary1.total_num_lines as f64 / summary1.num_songs as f64)
            );
        }
        if summary1.avg_lines_per_intro > summary2.avg_lines_per_intro {
            println!(
                "Average lines per intro: {} = {} > {} = {}",
                artist1, summary1.avg_lines_per_intro, artist2, summary2.avg_lines_per_intro
            );
        } else {
            println!(
                "Average lines per intro: {} = {} > {} = {}",
                artist2, summary2.avg_lines_per_intro, artist1, summary1.avg_lines_per_intro
            );
        }
        if summary1.avg_lines_per_verse > summary2.avg_lines_per_verse {
            println!(
                "Average lines per verse: {} = {} > {} = {}",
                artist1, summary1.avg_lines_per_verse, artist2, summary2.avg_lines_per_verse
            );
        } else {
            println!(
                "Average lines per verse: {} = {} > {} = {}",
                artist2, summary2.avg_lines_per_verse, artist1, summary1.avg_lines_per_verse
            );
        }
        if summary1.avg_lines_per_pre_chorus > summary2.avg_lines_per_pre_chorus {
            println!(
                "Average lines per pre-chorus: {} = {} > {} = {}",
                artist1,
                summary1.avg_lines_per_pre_chorus,
                artist2,
                summary2.avg_lines_per_pre_chorus
            );
        } else {
            println!(
                "Average lines per pre-chorus: {} = {} > {} = {}",
                artist2,
                summary2.avg_lines_per_pre_chorus,
                artist1,
                summary1.avg_lines_per_pre_chorus
            );
        }
        if summary1.avg_lines_per_chorus > summary2.avg_lines_per_chorus {
            println!(
                "Average lines per chorus: {} = {} > {} = {}",
                artist1, summary1.avg_lines_per_chorus, artist2, summary2.avg_lines_per_chorus
            );
        } else {
            println!(
                "Average lines per chorus: {} = {} > {} = {}",
                artist2, summary2.avg_lines_per_chorus, artist1, summary1.avg_lines_per_chorus
            );
        }

        if summary1.avg_words_per_intro > summary2.avg_words_per_intro {
            println!(
                "Average words per intro: {} = {} > {} = {}",
                artist1, summary1.avg_words_per_intro, artist2, summary2.avg_words_per_intro
            );
        } else {
            println!(
                "Average words per intro: {} = {} > {} = {}",
                artist2, summary2.avg_words_per_intro, artist1, summary1.avg_words_per_intro
            );
        }
        if summary1.avg_words_per_verse > summary2.avg_words_per_verse {
            println!(
                "Average words per verse: {} = {} > {} = {}",
                artist1, summary1.avg_words_per_verse, artist2, summary2.avg_words_per_verse
            );
        } else {
            println!(
                "Average words per verse: {} = {} > {} = {}",
                artist2, summary2.avg_words_per_verse, artist1, summary1.avg_words_per_verse
            );
        }
        if summary1.avg_words_per_pre_chorus > summary2.avg_words_per_pre_chorus {
            println!(
                "Average words per pre-chorus: {} = {} > {} = {}",
                artist1,
                summary1.avg_words_per_pre_chorus,
                artist2,
                summary2.avg_words_per_pre_chorus
            );
        } else {
            println!(
                "Average words per pre-chorus: {} = {} > {} = {}",
                artist2,
                summary2.avg_words_per_pre_chorus,
                artist1,
                summary1.avg_words_per_pre_chorus
            );
        }
        if summary1.avg_words_per_chorus > summary2.avg_words_per_chorus {
            println!(
                "Average words per chorus: {} = {} > {} = {}",
                artist1, summary1.avg_words_per_chorus, artist2, summary2.avg_words_per_chorus
            );
        } else {
            println!(
                "Average words per chorus: {} = {} > {} = {}",
                artist2, summary2.avg_words_per_chorus, artist1, summary1.avg_words_per_chorus
            );
        }
    }
}

/*
 * end of code for analysis
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
        }
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
                                let mut save_lyrics =
                                    lyrics.text().map(String::from).collect::<Vec<_>>();

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
                                    let mut paren_found = false;
                                    let mut bracket_found = false;
                                    if save_lyrics[i].contains("(") && i + 2 < save_lyrics.len() {
                                        if (save_lyrics[i + 2].contains(")")
                                            && !save_lyrics[i + 2].contains("("))
                                            || (save_lyrics[i + 2].chars().next().unwrap() == ')')
                                        {
                                            paren_found = true;
                                            save_lyrics[i] = format!(
                                                "{}{}{}",
                                                save_lyrics[i],
                                                save_lyrics[i + 1],
                                                save_lyrics[i + 2]
                                            );
                                            save_lyrics[i + 1] =
                                                format!("{}__REMOVE_LINE__", save_lyrics[i + 1]);
                                            save_lyrics[i + 2] =
                                                format!("{}__REMOVE_LINE__", save_lyrics[i + 2]);
                                        }
                                    }
                                    if !paren_found
                                        && (save_lyrics[i].contains("(")
                                            && i + 4 < save_lyrics.len())
                                    {
                                        if (save_lyrics[i + 4].contains(")")
                                            && !save_lyrics[i + 4].contains("("))
                                            || (save_lyrics[i + 4].chars().next().unwrap() == ')')
                                        {
                                            if !(save_lyrics[i + 1].contains("__REMOVE_LINE__")
                                                || save_lyrics[i + 2].contains("__REMOVE_LINE__")
                                                || save_lyrics[i + 3].contains("__REMOVE_LINE__"))
                                            {
                                                save_lyrics[i] = format!(
                                                    "{}{}{}{}{}",
                                                    save_lyrics[i],
                                                    save_lyrics[i + 1],
                                                    save_lyrics[i + 2],
                                                    save_lyrics[i + 3],
                                                    save_lyrics[i + 4]
                                                );
                                                save_lyrics[i + 1] = format!(
                                                    "{}__REMOVE_LINE__",
                                                    save_lyrics[i + 1]
                                                );
                                                save_lyrics[i + 2] = format!(
                                                    "{}__REMOVE_LINE__",
                                                    save_lyrics[i + 2]
                                                );
                                                save_lyrics[i + 3] = format!(
                                                    "{}__REMOVE_LINE__",
                                                    save_lyrics[i + 3]
                                                );
                                                save_lyrics[i + 4] = format!(
                                                    "{}__REMOVE_LINE__",
                                                    save_lyrics[i + 4]
                                                );
                                            }
                                        }
                                    }
                                    if save_lyrics[i].contains("[")
                                        && (!save_lyrics[i].contains("]"))
                                        && i + 2 < save_lyrics.len()
                                    {
                                        if save_lyrics[i + 2].contains("]") {
                                            bracket_found = true;
                                            save_lyrics[i] = format!(
                                                "{}{}{}",
                                                save_lyrics[i],
                                                save_lyrics[i + 1],
                                                save_lyrics[i + 2]
                                            );
                                            save_lyrics[i + 1] =
                                                format!("{}__REMOVE_LINE__", save_lyrics[i + 1]);
                                            save_lyrics[i + 2] =
                                                format!("{}__REMOVE_LINE__", save_lyrics[i + 2]);
                                        }
                                    }
                                    if !bracket_found
                                        && (save_lyrics[i].contains("[")
                                            && (!save_lyrics[i].contains("]"))
                                            && i + 4 < save_lyrics.len())
                                    {
                                        if save_lyrics[i + 4].contains("]") {
                                            save_lyrics[i] = format!(
                                                "{}{}{}{}{}",
                                                save_lyrics[i],
                                                save_lyrics[i + 1],
                                                save_lyrics[i + 2],
                                                save_lyrics[i + 3],
                                                save_lyrics[i + 4]
                                            );
                                            save_lyrics[i + 1] =
                                                format!("{}__REMOVE_LINE__", save_lyrics[i + 1]);
                                            save_lyrics[i + 2] =
                                                format!("{}__REMOVE_LINE__", save_lyrics[i + 2]);
                                            save_lyrics[i + 3] =
                                                format!("{}__REMOVE_LINE__", save_lyrics[i + 3]);
                                            save_lyrics[i + 4] =
                                                format!("{}__REMOVE_LINE__", save_lyrics[i + 4]);
                                        }
                                    }

                                    i += 1;
                                }

                                for line in &save_lyrics[0..save_lyrics.len() - 6] {
                                    // - 6 is for the last 6 parts of lyrics page that aren't actually lyrics

                                    if !(*line).contains("__REMOVE_LINE__") {
                                        f.write_all(line.as_bytes()).expect("Unable to write line");
                                        f.write_all("\n".as_bytes())
                                            .expect("Unable to write new line");
                                    }
                                    /* else {
                                        println!(" FOUND MATCHES FOR WEIRD ITALICS ISSUE WITH NEWLINES ")
                                    } */
                                }
                            }
                        }
                        Err(e) => println!("selector failed: {:?}", e),
                    }
                }
                Err(e) => println!("fragment text failed: {:?}", e),
            }
        }
        Err(e) => println!("url reqwest failed: {:?}", e),
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    fs::create_dir("./lyrics").ok();
    let args: Vec<String> = env::args().collect();
    let argc = args.len();

    if argc == 4 && args[3].eq("-c") {
        let artist1 = &args[1].to_lowercase().replace(" ", "_");
        let artist2 = &args[2].to_lowercase().replace(" ", "_");

        parse_text_main_compare(artist1, artist2).ok();
    } else {
        let artist = &args[1].to_lowercase().replace(" ", "_");
        let artist_urls = get_urls_for_artist(&artist);

        let artist_urls = artist_urls.await;
        // println!("artist_urls: {:?}\n\n\n\n", artist_urls);

        // let write_path = &args[2];

        let mut print = false;
        let mut url_idx = 0;
        for (raw_title, url) in &artist_urls {
            url_idx += 1;
            let mut filename = "./lyrics/".to_owned();
            filename.push_str(&artist);
            fs::create_dir(filename).ok();
            // title will be the song title, which is ended by the word "by\u{a0}" in the url which is used for extracting it
            let title = &(raw_title[0..raw_title.find("by\u{a0}").unwrap() - 1]
                .to_lowercase()
                .replace(" ", "_")); // changes spaces in the song title to be _
                                     // print!("title was: {:?}", title);
                                     // print!("raw_title was: {:?}", raw_title);
            write_lyrics_from_urls(&url, title, artist).await;
            if url_idx == artist_urls.len() {
                print = true;
            }
            parse_text_main(artist, "test", print).ok();
        }
    }

    return Ok(());
}
