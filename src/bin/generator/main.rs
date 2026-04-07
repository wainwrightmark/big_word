pub mod wordnet;
use std::{
    collections::{BTreeMap, BTreeSet, HashSet},
    fs::{self},
    num::NonZeroU32,
};

use arrayvec::ArrayVec;
use big_word::{
    SynsetId, SynsetRelType, SynsetRelation, WordChars,
    character::{Character, normalize_characters_array},
};
use strum::IntoEnumIterator;

use crate::wordnet::{
    wordnet_db::LoadMode,
    wordnet_types::{self, Pos},
};

fn main() {
    let dict_dir = ::std::path::Path::new(r#"C:\Source\english_word_list\Wordnet"#);
    let word_net = wordnet::wordnet_db::WordNet::load_with_mode(&dict_dir, LoadMode::Mmap).unwrap();

    let morphy = crate::wordnet::wordnet_morphy::Morphy::load(dict_dir).unwrap();

    let mut synsets: BTreeMap<wordnet_types::SynsetId, SynsetId> = BTreeMap::new();
    let mut used_synset_ids: BTreeSet<wordnet_types::SynsetId> = BTreeSet::default();
    //let mut words: BTreeMap<WordChars, (big_word::WordId, HashSet<String>)> = BTreeMap::new();

    let mut next_synset_id = 0u32;

    let all_senses: Vec<wordnet_types::Synset> = word_net.iter_synsets().collect();

    for sense in all_senses.iter() {
        synsets.insert(sense.id, SynsetId::new(next_synset_id));
        next_synset_id += 1;
    }

    let mut big_word_words: Vec<big_word::Word> = vec![];
    let mut big_word_synsets: Vec<big_word::SynSet> = vec![];
    let mut popularity = 1;
    let mut used_char_arrays: HashSet<ArrayVec<Character, 24>, _> = HashSet::new();

    enum WordList {
        Google20000,
        WordsAlpha,
    }

    for word_list in [WordList::Google20000, WordList::WordsAlpha] {
        let path = match word_list {
            WordList::Google20000 => r"C:\Source\english_word_list\google-20000-english.txt",
            WordList::WordsAlpha => r"C:\Source\english_word_list\words_alpha.txt",
        };

        let basic_words_text = fs::read_to_string(path).unwrap();
        'basic_words: for basic_word in basic_words_text.lines() {
            let Ok(av) = big_word::character::normalize_characters_array::<24>(&*basic_word) else {
                println!("Word '{basic_word}' has too many characters");
                continue 'basic_words;
            };

            if !used_char_arrays.insert(av) {
                continue 'basic_words;
            }

            let mut meanings = vec![];
            let mut root_forms = vec![];

            for pos in Pos::iter() {
                for lemma in morphy.lemmas_for(pos, &basic_word, |p, s| word_net.lemma_exists(p, s))
                {
                    if lemma.source.is_surface() {
                        if let Some(index_entry) = word_net.index_entry(lemma.pos, &lemma.lemma) {
                            for offset in index_entry.synset_offsets.iter().copied() {
                                let ssi = wordnet_types::SynsetId { pos, offset };
                                used_synset_ids.insert(ssi);

                                if let Some(ssi) = synsets.get(&ssi) {
                                    meanings.push(*ssi);
                                }
                            }
                        }
                    } else {
                        if let Ok(arr) = normalize_characters_array(&lemma.lemma) {
                            root_forms.push(WordChars(arr));
                        }
                    }
                }
            }
            meanings.sort();
            meanings.dedup();

            root_forms.sort();
            root_forms.dedup();

            if meanings.is_empty() && root_forms.is_empty() {
                match word_list {
                    WordList::Google20000 => {
                        println!("Word '{}' not identified", basic_word);
                    }
                    WordList::WordsAlpha => {
                        continue 'basic_words;
                    }
                }
            }

            let popularity = match word_list {
                WordList::Google20000 => {
                    let p = NonZeroU32::new(popularity);
                    popularity += 1;
                    p
                }
                WordList::WordsAlpha => None,
            };

            let word = big_word::Word {
                popularity,
                text: basic_word.to_string(),
                meanings,
                root_forms,
            };

            big_word_words.push(word);
        }
    }

    for ssi in used_synset_ids.iter().copied() {
        if let Some(synset) = word_net.get_synset(ssi) {
            let words: Vec<WordChars> = synset
                .words
                .iter()
                .flat_map(|lemma| normalize_characters_array::<24>(&lemma.text))
                .map(|x| WordChars(x))
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

    // let mut words_cbor = vec![];
    // ciborium::ser::into_writer(&big_word_words, &mut words_cbor).unwrap();
    // std::fs::write("words.cbor", words_cbor.as_slice()).unwrap();

    // let mut synsets_cbor = vec![];
    // ciborium::ser::into_writer(&big_word_synsets, &mut synsets_cbor).unwrap();
    // std::fs::write("synsets.cbor", synsets_cbor.as_slice()).unwrap();
    
    
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
