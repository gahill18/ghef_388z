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
    println!("path_to_dataset: {:?}", path_to_dataset);
    
    let dataset = generate_dataset(path_to_dataset);
    let mut nbc = NaiveBayesClassifier::new();
    
    train_on_dataset(dataset, &mut nbc);

    guess_for("strand_kiss", &mut nbc);
    guess_for("handsome valid", &mut nbc);
}

// Generating Dataset Section

fn generate_dataset (path_to_dataset: &str) -> HashMap<String, String> {
    let mut dataset = HashMap::new();

    match File::open(path_to_dataset) {
        Ok(mut file) => {
            // Get raw file contents
            let mut raw_file_contents = String::new();
            match file.read_to_string(&mut raw_file_contents) {
                Ok(size) => println!("raw file contents: {:?}", raw_file_contents),
                Err(e) => println!("failed to read from file: {:?}", e),
            }

            // Get key label pairs
            let key_label_pairs =raw_file_contents.split("\n");
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

fn guess_for (string_to_guess: &str, nbc: &mut NaiveBayesClassifier) {
    let guess = nbc.guess(string_to_guess);
    println!("nbc guessed {:?} fits label {:?}", string_to_guess, guess);
}
