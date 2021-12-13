extern crate natural;

use natural::classifier::NaiveBayesClassifier;

use std::collections::HashMap;
//use std::error::Error;
use std::{env, fs};
use std::fs::File;
use std::io::prelude::*;


fn main() {
    let args: Vec<String> = env::args().collect();
     if args.len() != 3 {
        println!("proper usage: cargo run [mode] [path_to_dataset]");
        println!("mode can be either \"raw\" or \"prefab\"");
        return;
    }

    let mode = args.get(1).unwrap();
    println!("mode: {:?}", mode);

    let path_to_dataset = args.get(2).unwrap();

    if mode.eq("raw") {
        match lyrics_to_key_label(path_to_dataset) {
            Ok(new_path) => {
                let dataset = generate_dataset(new_path, "\n");
                let mut nbc = NaiveBayesClassifier::new();
        
                train_on_dataset(dataset, &mut nbc);
                guess_for("raw mode", &mut nbc);
            },
            Err(text) => {
                println!("{:?}", text);
            },
        }

    } 

    else if mode.eq("prefab") {
        let dataset = generate_dataset(&path_to_dataset, ",");
        let mut nbc = NaiveBayesClassifier::new();

        train_on_dataset(dataset, &mut nbc);
        guess_for("prefab mode", &mut nbc);
    } 

    else {
        println!("non-valid mode input");
        return
    }
}

// Generating Dataset Section

fn generate_dataset (path_to_dataset: &str, split_on: &str) -> HashMap<String, String> {
    let mut dataset = HashMap::new();

    match File::open(path_to_dataset) {
        Ok(mut file) => {
            // Get raw file contents
            let mut raw_file_contents = String::new();
            match file.read_to_string(&mut raw_file_contents) {
                Ok(size) => println!("size: {:?}, raw file contents: {:?}", size, raw_file_contents),
                Err(e) => println!("failed to read from file: {:?}", e),
            }

            // Get key label pairs
            let key_label_pairs =raw_file_contents.split(split_on);
            for key_label_pair in key_label_pairs {
                let mut k_l_split = key_label_pair.split("/");
                let k_to_unwrap = k_l_split.next();
                let l_to_unwrap = k_l_split.next();

                // Write key label pairs
                match (k_to_unwrap, l_to_unwrap) {
                    (Some(k), Some(l)) => {
                        dataset.insert(k.to_string(), l.to_string());
                    },
                    (_, _) => println!("failed to read key pair"),
                }
            }
        },
        Err(e) => println!("failed to open path_to_dataset: {:?}", e),
    };

    dataset
}

// Training Dataset Section

fn train_on_dataset (dataset: HashMap<String, String>, nbc: &mut NaiveBayesClassifier) {
    for key in dataset.keys() {
        match dataset.get(key) {
            Some(label) => {
                println!("key in dataset: {:?}, label: {:?}", key, label);
                nbc.train(key, label);
            },
            None => println!("no label for string {:?}", key),
        }
    }
}

// Simple wrapper function for debugging purposes
// Takes in a string for the classifier to guess the label of
fn guess_for (string_to_guess: &str, nbc: &mut NaiveBayesClassifier) {
    let guess = nbc.guess(string_to_guess);
    println!("nbc guessed {:?} fits label {:?}", string_to_guess, guess);
}

// Testing Helper Functions Section

/*
Takes in a path to the lyrics folder from song_structures output and creates a text file
that can be read by the dataset generator
*/
fn lyrics_to_key_label(path_to_lyrics_folder: &str) -> Result<&str, &str> {
    // If file to write to is successfully created
    match fs::read_dir(path_to_lyrics_folder) {
        Ok(paths) => {
            // If the path to the lyrics folder is valid, collect all of the lyrics with their genres
            let mut contents = String::new();
            for path_result in paths {
                match path_result {
                    Ok(path) => {
                        match File::open(path.path()) {

                            Ok(mut file) => {
                               let _result = file.read_to_string(&mut contents);
                               contents.push_str("\temp genre,");
                            },
                            Err(e) => println!("error reading path: {:?}", e),
                        }
                    }
                    Err(e) => println!("error reading path_result: {:?}", e),
                }
            }
                
            let path_to_new_database = "./bin/output.txt";
            match fs::write(path_to_new_database, contents) {
                Ok(_result) => {
                    println!("successfully wrote contents to file");
                    let output = path_to_new_database.clone();
                    Ok(output)
                },
                Err(_e) => core::result::Result::Err("failed to write contents to file"),
            }
        },
        Err(_e) => core::result::Result::Err("failed to write contents to file"),
    }      
    
}
