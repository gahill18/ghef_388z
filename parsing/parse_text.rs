use std::fs;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let artist_name = String::from("Cardi B").to_lowercase().replace(" ", "_");
    let mut artist_path = String::from("../song_structures/lyrics/");
    artist_path.push_str(artist_name.as_str());
    let paths = fs::read_dir(artist_path.as_str()).unwrap();
    let category = String::from("fem_hip_hop");

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
