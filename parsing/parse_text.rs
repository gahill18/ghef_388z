use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("arianagrande.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    // print!("{}",contents);

    let artist_name = String::from("Ariana Grande");
    let category = String::from("fem_pop");

    let mut artist = Artist {
        name : String::from(&artist_name),
        songs : Vec::new(),
        category : String::from(category),
    };

    // need to make sure when text was read in that songs were separated by "<TITLE>"
    let mut split_songs = contents.split("<TITLE>");
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
        for line in &self.lines {
            print!("{}\n", line);
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
            let mut split_lines = raw_structure.split("\n");
            let lines: Vec<&str> = split_lines.collect();

            self.structures.push(Structure {
                struct_type : String::from(*lines.first().unwrap()),
                lines : lines.clone(),
            }
            )
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
