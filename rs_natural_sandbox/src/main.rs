extern crate natural;

use natural::classifier::NaiveBayesClassifier;

use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;


fn main() {
    let args: Vec<String> = env::args().collect();
     if args.len() != 2 {
        println!("proper usage: cargo run [path_to_dataset]");
        return;
    }

    
    

    let path_to_dataset = args.get(1).unwrap();
    
    let dataset = generate_dataset(path_to_dataset);
    let mut nbc = NaiveBayesClassifier::new();
    
    train_on_dataset(dataset, &mut nbc);

    guess_for("test string_to_guess", &mut nbc);
    guess_for("test string_to_train", &mut nbc);
}

// Generating Dataset Section

fn generate_dataset (path_to_dataset: &str) -> HashMap<String, String> {
    let mut dataset = HashMap::new();

    match File::open(path_to_dataset) {
        Ok(mut file) => {
            let mut contents = String::new();
            match file.read_to_string(&mut contents) {
                Ok(size) => println!("read in {:?} bytes", size),
                Err(e) => println!("failed to read from file: {:?}", e),
            }
            dataset.insert(contents, "temp".to_string());
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

fn guess_for (string_to_guess: &str, nbc: &mut NaiveBayesClassifier) {
    let guess = nbc.guess(string_to_guess);
    println!("nbc guessed {:?} fits label {:?}", string_to_guess, guess);
}
