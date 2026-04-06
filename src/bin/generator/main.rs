pub mod wordnet;
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
};

use big_word::{SynSet, SynsetId, SynsetRelType, SynsetRelation, Word, WordChars, WordId};

use crate::wordnet::Relationship;

fn main() {
    let basic_words_text =
        fs::read_to_string(r"C:\Source\english_word_list\google-20000-english.txt").unwrap();
    let basic_words: BTreeSet<&str> = basic_words_text.lines().collect();

    let wn = wordnet::Database::open(&::std::path::Path::new(
        r#"C:\Source\rust\wordnet_stuff\wordnet"#,
    ))
    .unwrap();

    let mut synsets: BTreeMap<wordnet::SenseId, SynsetId> = BTreeMap::new();
    let mut words: BTreeMap<WordChars, (String, big_word::WordId)> = BTreeMap::new();
    let mut next_word_id = 0u32;

    let all_senses: Vec<(wordnet::SenseId, wordnet::Sense<'_>)> = wn.all_senses();

    for (index, (sense_id, sense)) in all_senses.iter().enumerate() {
        let synset_id = SynsetId::new(index as u32);
        synsets.insert(*sense_id, synset_id);

        for word in sense.synonyms.iter() {
            if let Ok(av) = big_word::character::normalize_characters_array(&word.word) {
                match words.entry(WordChars(av)) {
                    std::collections::btree_map::Entry::Vacant(vacant_entry) => {
                        vacant_entry.insert((word.word.clone(), WordId::new(next_word_id)));
                        next_word_id += 1;
                    }
                    std::collections::btree_map::Entry::Occupied(..) => {}
                }
            }
        }
    }

    let mut big_word_words: Vec<big_word::Word> = vec![];
    let mut big_word_synsets: Vec<big_word::SynSet> = vec![];

    'basic_words: for basic_word in basic_words.iter().take(100) {
        let Ok(av) = big_word::character::normalize_characters_array(&*basic_word.word) else {continue 'basic_words;};
        
        wn.senses(*basic_word)

    }

    for (index, sense) in all_senses.into_iter().enumerate() {

        //sense.pointers.iter().map(|x|x.)

        //sense.pointers.iter().map(|x|x.part_of_speech)

        //let words = sense.synonyms.
        // synsets.push(SynSet {
        //     id: SynsetId::new(index as u32),
        //     words: (),
        //     definition: sense.gloss,
        //     part_of_speech: convert_part_of_speech(&sense.part_of_speech),
        //     relations: todo!(),
        // });
    }

    // let mut results: Vec<Senses> = vec![];

    // for sense in all_senses {
    //     let count = sense
    //         .synonyms
    //         .iter()
    //         .filter(|x| x.word.len() > 3)
    //         .filter(|x| basic_words.contains(x.word.as_str()))
    //         .count();
    //     if count >= 6 {
    //         let s = Senses {
    //             count: count,
    //             gloss: sense.gloss,
    //             words: sense
    //                 .synonyms
    //                 .iter()
    //                 .filter(|x| x.word.len() > 3)
    //                 .map(|x| x.word.clone())
    //                 .collect(),
    //         };
    //         results.push(s);
    //     }
    // }

    // results.sort_by_key(|x| (std::cmp::Reverse(x.count), x.gloss.clone()));
    // results.dedup_by_key(|x| x.gloss.clone());

    // println!("{} results found", results.len());

    // let mut output = String::new();

    // for line in results.into_iter().take(1000) {
    //     use std::fmt::Write;
    //     writeln!(
    //         output,
    //         "{}\t{}\t{}",
    //         line.count,
    //         line.words
    //             .iter()
    //             .map(|x| x.as_str())
    //             .collect::<Vec<_>>()
    //             .join(", "),
    //         line.gloss
    //     )
    //     .unwrap();
    // }

    // std::fs::write("output.tsv", output).unwrap();
}

#[derive(Debug, Clone)]
pub struct Senses {
    pub count: usize,
    pub gloss: String,
    pub words: Vec<String>,
}

fn convert_part_of_speech(pos: &wordnet::PartOfSpeech) -> big_word::PartOfSpeech {
    match pos {
        wordnet::PartOfSpeech::Noun => big_word::PartOfSpeech::Noun,
        wordnet::PartOfSpeech::Adjective => big_word::PartOfSpeech::Adjective,
        wordnet::PartOfSpeech::AdjectiveSatellite => big_word::PartOfSpeech::Adjective,
        wordnet::PartOfSpeech::Verb => big_word::PartOfSpeech::Verb,
        wordnet::PartOfSpeech::Adverb => big_word::PartOfSpeech::Adverb,
    }
}

fn convert_relationship(rel: &Relationship) -> SynsetRelType {
    match rel {
        Relationship::Antonym => SynsetRelType::Antonym,
        Relationship::Hypernym => SynsetRelType::Hypernym,
        Relationship::InstanceHypernym => SynsetRelType::InstanceHypernym,
        Relationship::Hyponym => SynsetRelType::Hyponym,
        Relationship::MemberHolonym => SynsetRelType::MemberHolonym,
        Relationship::SubstanceHolonym => SynsetRelType::SubstanceHolonym,
        Relationship::PartHolonym => SynsetRelType::PartHolonym,
        Relationship::MemberMeronym => SynsetRelType::MemberMeronym,
        Relationship::SubstanceMeronym => SynsetRelType::SubstanceMeronym,
        Relationship::PartMeronym => SynsetRelType::PartMeronym,
        Relationship::Attribute => SynsetRelType::Attribute,
        Relationship::DerivationallyRelated => SynsetRelType::DerivationallyRelated,
        Relationship::DomainOfTopic => SynsetRelType::DomainOfTopic,
        Relationship::MemberOfTopic => SynsetRelType::MemberOfTopic,
        Relationship::DomainOfRegion => SynsetRelType::DomainOfRegion,
        Relationship::MemberOfRegion => SynsetRelType::MemberOfRegion,
        Relationship::DomainOfUsage => SynsetRelType::DomainOfUsage,
        Relationship::MemberOfUsage => SynsetRelType::MemberOfUsage,
        Relationship::Entailment => SynsetRelType::Entailment,
        Relationship::Cause => SynsetRelType::Cause,
        Relationship::AlsoSee => SynsetRelType::AlsoSee,
        Relationship::VerbGroup => SynsetRelType::VerbGroup,
        Relationship::SimilarTo => SynsetRelType::SimilarTo,
        Relationship::VerbParticiple => SynsetRelType::VerbParticiple,
        Relationship::PertainymOrDerivedFromAdjective => {
            SynsetRelType::PertainymOrDerivedFromAdjective
        }
    }
}
