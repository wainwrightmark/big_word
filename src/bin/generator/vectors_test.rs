use std::collections::HashSet;

use big_word::{WordChars, word_vectors::WordVectors};
use clap::Args;

#[derive(Args, Debug)]
pub struct TestVectorArgs {
    pub master: String,
    pub words: Vec<String>,
}

pub fn test_vectors(args: TestVectorArgs) {
    let file = std::fs::File::open("vectors.cbor").unwrap();
    let word_vectors: Vec<WordVectors> = ciborium::from_reader(file).unwrap();

    let master_key = WordChars::format(&args.master);
    let word_keys: HashSet<WordChars> = args.words.iter().map(|x| WordChars::format(x)).collect();

    let mut master_word: Option<WordVectors> = None;
    let mut found_words: Vec<WordVectors> = vec![];

    for wv in word_vectors.into_iter() {
        if wv.word == master_key {
            master_word = Some(wv);
        } else if word_keys.contains(&wv.word) {
            found_words.push(wv);
        }
    }

    let master_word = master_word.expect("Could not find master word");

    let mut distances: Vec<_> = found_words
        .into_iter()
        .map(|wv| (wv.distance_from_squared(&master_word), wv ))
        .collect();

    distances.sort_by_key(|x|x.0);

    println!("Master: '{}'", master_word.word.as_str());

    for (distance, wv) in distances.into_iter(){
        println!("{:16}: {distance}", wv.word.as_str())
    }


}
