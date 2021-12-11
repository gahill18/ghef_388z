extern crate natural;

use natural::classifier::NaiveBayesClassifier;

use std::collections::HashMap;

fn main() {
    let mut nbc = NaiveBayesClassifier::new();
    let mut dataset = HashMap::new();

    dataset.insert("test string_to_train", "test label");
    
    train_on_dataset(dataset, &mut nbc);

    guess_for("test string_to_train", &mut nbc);
    guess_for("test stringtotrain", &mut nbc);
}

fn train_on_dataset (dataset: HashMap<&str, &str>, nbc: &mut NaiveBayesClassifier) {
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

fn train_on_string (string_to_train: &str, label: &str, nbc: &mut NaiveBayesClassifier) {
    println!("string_to_train: {:?}, label: {:?}", string_to_train, label);
    nbc.train(string_to_train, label);
}

fn guess_for (string_to_guess: &str, nbc: &mut NaiveBayesClassifier) {
    let guess = nbc.guess(string_to_guess);
    println!("nbc guessed {:?} fits label {:?}", string_to_guess, guess);
}
