use std::fs;
use clap::Args;

#[derive(Args, Debug)]
pub struct CheckWordArgs {
    pub word: String,
    
}

pub fn check_words(args: CheckWordArgs) {
    let words_data = fs::read("words.cbor").unwrap();
    let words: Vec<big_word::Word> = ciborium::from_reader(words_data.as_slice()).unwrap();    

    for word in words.into_iter(){
        if word.text.eq_ignore_ascii_case(&args.word){
            println!("Found word {}", word.text);

            for synset_id in word.meanings.iter()  {
                println!("{}", synset_id.0)
            }
        }
    }

    let synsets_data = fs::read("synsets.cbor").unwrap();
    let synsets: Vec<big_word::SynSet> = ciborium::from_reader(synsets_data.as_slice()).unwrap();

    for synset in synsets.into_iter().take(5){
        println!("Synset {}: {}", synset.id.0, synset.definition);
    }
    
}