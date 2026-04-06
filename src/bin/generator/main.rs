pub mod wordnet;
use std::{
    collections::{BTreeMap, BTreeSet, HashSet},
    fs,
};

use arrayvec::ArrayVec;
use big_word::{
    SynsetId, SynsetRelType, SynsetRelation, WordId,
    character::{Character, normalize_characters_array},
};
use strum::IntoEnumIterator;

use crate::wordnet::{
    wordnet_db::LoadMode,
    wordnet_types::{self, Pos},
};

fn main() {
    let basic_words_text =
        fs::read_to_string(r"C:\Source\english_word_list\google-20000-english.txt").unwrap();
    let basic_words: Vec<&str> = basic_words_text.lines().collect();
    let dict_dir = ::std::path::Path::new(r#"C:\Source\english_word_list\Wordnet"#);
    let word_net = wordnet::wordnet_db::WordNet::load_with_mode(&dict_dir, LoadMode::Mmap).unwrap();

    let morphy = crate::wordnet::wordnet_morphy::Morphy::load(dict_dir).unwrap();

    let mut synsets: BTreeMap<wordnet_types::SynsetId, SynsetId> = BTreeMap::new();
    let mut used_synset_ids: BTreeSet<wordnet_types::SynsetId> = BTreeSet::default();
    //let mut words: BTreeMap<WordChars, (big_word::WordId, HashSet<String>)> = BTreeMap::new();
    let mut next_word_id = 0u32;
    let mut next_synset_id = 0u32;

    let all_senses: Vec<wordnet_types::Synset> = word_net.iter_synsets().collect();

    for sense in all_senses.iter() {
        synsets.insert(sense.id, SynsetId::new(next_synset_id));
        next_synset_id += 1;

        // for word in sense.words.iter() {
        //     if let Ok(av) = big_word::character::normalize_characters_array(&word.text) {
        //         match words.entry(WordChars(av)) {
        //             std::collections::btree_map::Entry::Vacant(vacant_entry) => {
        //                 vacant_entry.insert((
        //                     WordId::new(next_word_id),
        //                     HashSet::from_iter([word.text.to_string()]),
        //                 ));
        //                 next_word_id += 1;
        //             }
        //             std::collections::btree_map::Entry::Occupied(..) => {}
        //         }
        //     }
        // }
    }

    let mut big_word_words: Vec<big_word::Word> = vec![];
    let mut big_word_synsets: Vec<big_word::SynSet> = vec![];
    let mut popularity = 0;
    let mut used_char_arrays: HashSet<ArrayVec<Character, 24>, _> = HashSet::new();
    'basic_words: for basic_word in basic_words.iter().take(10000000000) {
        let Ok(av) = big_word::character::normalize_characters_array::<24>(&*basic_word) else {
            println!("Word '{basic_word}' has too many characters");
            continue 'basic_words;
        };

        if !used_char_arrays.insert(av) {
            continue 'basic_words;
        }
        let id = WordId::new(next_word_id);
        next_word_id += 1;

        // let Some((word_id, set)) = words.get(&WordChars(av.clone())) else {
        //     println!("Word '{basic_word}' is not in wordnet");
        //     continue 'basic_words;
        // };

        let mut meanings = vec![];
        let mut is_inflected = true;

        for pos in Pos::iter() {
            for lemma in morphy.lemmas_for(pos, &basic_word, |p, s| word_net.lemma_exists(p, s)) {
                if lemma.source.is_surface() {
                    is_inflected = true;
                }
                if let Some(index_entry) = word_net.index_entry(lemma.pos, &lemma.lemma) {
                    for offset in index_entry.synset_offsets.iter().copied() {
                        let ssi = wordnet_types::SynsetId { pos, offset };
                        used_synset_ids.insert(ssi);

                        if let Some(ssi) = synsets.get(&ssi) {
                            meanings.push(*ssi);
                        }
                    }
                }
            }
        }
        meanings.sort();
        meanings.dedup();

        let word = big_word::Word {
            id,
            popularity: popularity,
            text: basic_word.to_string(),
            meanings,
            is_inflected,
        };

        popularity += 1;

        big_word_words.push(word);
    }

    let big_word_map: BTreeMap<ArrayVec<Character, 24>, WordId> = big_word_words
        .iter()
        .map(|x| (normalize_characters_array::<24>(&x.text).unwrap(), x.id))
        .collect();

    for ssi in used_synset_ids.iter().copied() {
        if let Some(synset) = word_net.get_synset(ssi) {
            let words: Vec<WordId> = synset
                .words
                .iter()
                .flat_map(|lemma| normalize_characters_array::<24>(&lemma.text))
                .flat_map(|arr| big_word_map.get(&arr))
                .copied()
                .collect();

            let relations: Vec<big_word::SynsetRelation> = synset
                .pointers
                .iter()
                .filter(|x| used_synset_ids.contains(&x.target))
                .flat_map(|pointer| {
                    synsets
                        .get(&pointer.target)
                        .copied()
                        .map(|to_id| SynsetRelation {
                            to_id,
                            synset_rel_type: convert_relationship(pointer.symbol),
                        })
                })
                .collect();

            if let Some(id) = synsets.get(&ssi).copied() {
                big_word_synsets.push(big_word::SynSet {
                    id,
                    definition: synset.gloss.definition.to_string(),
                    part_of_speech: convert_synset_type(&synset.synset_type),
                    words,
                    relations,
                });
            }
        }
    }

    let words_yaml = serde_yaml::to_string(&big_word_words).unwrap();

    std::fs::write("words.yaml", words_yaml.as_str()).unwrap();

    let synsets_yaml = serde_yaml::to_string(&big_word_synsets).unwrap();

    std::fs::write("synsets.yaml", synsets_yaml.as_str()).unwrap();
}

#[derive(Debug, Clone)]
pub struct Senses {
    pub count: usize,
    pub gloss: String,
    pub words: Vec<String>,
}

fn convert_synset_type(pos: &wordnet::wordnet_types::SynsetType) -> big_word::PartOfSpeech {
    match pos {
        wordnet::wordnet_types::SynsetType::Noun => big_word::PartOfSpeech::Noun,
        wordnet::wordnet_types::SynsetType::Adj => big_word::PartOfSpeech::Adjective,
        wordnet_types::SynsetType::AdjSatellite => big_word::PartOfSpeech::Adjective,
        wordnet::wordnet_types::SynsetType::Verb => big_word::PartOfSpeech::Verb,
        wordnet::wordnet_types::SynsetType::Adv => big_word::PartOfSpeech::Adverb,
    }
}

fn convert_part_of_speech(pos: &wordnet::wordnet_types::Pos) -> big_word::PartOfSpeech {
    match pos {
        wordnet::wordnet_types::Pos::Noun => big_word::PartOfSpeech::Noun,
        wordnet::wordnet_types::Pos::Adj => big_word::PartOfSpeech::Adjective,

        wordnet::wordnet_types::Pos::Verb => big_word::PartOfSpeech::Verb,
        wordnet::wordnet_types::Pos::Adv => big_word::PartOfSpeech::Adverb,
    }
}

fn convert_relationship(pointer_symbol: &str) -> SynsetRelType {
    match pointer_symbol {
        "!" => SynsetRelType::Antonym,
        "@" => SynsetRelType::Hypernym,
        "@i" => SynsetRelType::InstanceHypernym,
        "~" => SynsetRelType::Hyponym,
        "~i" => SynsetRelType::InstanceHypernym,
        "#m" => SynsetRelType::MemberHolonym,
        "#s" => SynsetRelType::SubstanceHolonym,
        "#p" => SynsetRelType::PartHolonym,
        "%m" => SynsetRelType::MemberMeronym,
        "%s" => SynsetRelType::SubstanceMeronym,
        "%p" => SynsetRelType::PartMeronym,
        "=" => SynsetRelType::Attribute,
        "+" => SynsetRelType::DerivationallyRelated,
        ";c" => SynsetRelType::DomainOfTopic,
        "-c" => SynsetRelType::MemberOfTopic,
        ";r" => SynsetRelType::DomainOfRegion,
        "-r" => SynsetRelType::MemberOfRegion,
        ";u" => SynsetRelType::DomainOfUsage,
        "-u" => SynsetRelType::MemberOfUsage,
        "*" => SynsetRelType::Entailment,
        ">" => SynsetRelType::Cause,
        "^" => SynsetRelType::AlsoSee,
        "$" => SynsetRelType::VerbGroup,
        "&" => SynsetRelType::SimilarTo,
        "<" => SynsetRelType::VerbParticiple,
        "\\" => SynsetRelType::PertainymOrDerivedFromAdjective,
        x => panic!("illegal relationship code '{x}'"),
    }
}
