use std::{
    collections::{BTreeMap, BTreeSet, HashSet},
    fs::{self},
    num::NonZeroU32,
    sync::Arc,
};

use big_word::{SynsetId, SynsetRelType, SynsetRelation, WordChars};
use strum::IntoEnumIterator;
use ustr::Ustr;

use crate::wordnet::{
    self,
    wordnet_db::LoadMode,
    wordnet_types::{self, Pos},
};

pub fn generate_words_and_synsets() {
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
    let mut used_char_arrays: HashSet<WordChars, _> = HashSet::new();

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
            let av = WordChars::format(basic_word);

            if !used_char_arrays.insert(av) {
                continue 'basic_words;
            }

            let mut meanings = vec![];
            let mut root_forms = vec![];

            let mut text = Ustr::from(basic_word);

            

            for pos in Pos::iter() {
                for lemma in morphy.lemmas_for(pos, &basic_word, |p, s| word_net.lemma_exists(p, s))
                {
                    if lemma.source.is_surface() {
                        if let Some(index_entry) = word_net.index_entry(lemma.pos, &lemma.lemma) {
                            if let Some(spelling) = word_net.spellings.get(&(pos, lemma.lemma.to_string())){
                                text = Ustr::from(spelling);
                            }
                            
                            for offset in index_entry.synset_offsets.iter().copied() {
                                let ssi = wordnet_types::SynsetId { pos, offset };
                                used_synset_ids.insert(ssi);

                                if let Some(ssi) = synsets.get(&ssi) {
                                    meanings.push(*ssi);
                                }
                            }
                        }
                    } else {
                        root_forms.push(WordChars::format(&lemma.lemma));
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
                        //println!("Word '{}' not identified", basic_word);
                    }
                    WordList::WordsAlpha => {
                        //Do not include these words
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
                text,
                meanings: Arc::new(meanings),
                root_forms: Arc::new(root_forms),
            };

            big_word_words.push(word);
        }
    }

    for ssi in used_synset_ids.iter().copied() {
        if let Some(synset) = word_net.get_synset(ssi) {
            let words: Vec<WordChars> = synset
                .words
                .iter()
                .map(|x| WordChars::format(x.text))
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
                    words: Arc::new(words),
                    relations: Arc::new(relations),
                });
            }
        }
    }

    big_word_words.sort_by_cached_key(|x| WordChars::format(&x.text));
    big_word_synsets.sort_by_key(|x| x.id);

    // let max_meanings = big_word_words.iter().map(|x|x.meanings.len()).max().unwrap_or_default();
    // let max_root_forms = big_word_words.iter().map(|x|x.root_forms.len()).max().unwrap_or_default();

    // println!("Max meanings: {max_meanings}. Max root forms: {max_root_forms}");

    // for w in big_word_words.iter().filter(|x|x.meanings.len() >= 20){
    //     println!("Word has {} meanings: {}", w.meanings.len(), w.text);
    // }

    // let max_synset_words = big_word_synsets.iter().map(|x|x.words.len()).max().unwrap_or_default();
    // let max_synset_relations = big_word_synsets.iter().map(|x|x.relations.len()).max().unwrap_or_default();

    // for s in big_word_synsets.iter().filter(|x|x.words.len() >= 20){
    //     println!("Synset has {} words: {}", s.words.len(), s.definition);
    // }

    // for s in big_word_synsets.iter().filter(|x|x.relations.len() >= 100){
    //     println!("Synset has {} relations: {}", s.relations.len(), s.definition);
    // }

    // println!("Max synset words: {max_synset_words}. Max synset relations: {max_synset_relations}");

    let mut words_cbor = vec![];
    ciborium::ser::into_writer(&big_word_words, &mut words_cbor).unwrap();
    std::fs::write("words.cbor", words_cbor.as_slice()).unwrap();

    let mut synsets_cbor = vec![];
    ciborium::ser::into_writer(&big_word_synsets, &mut synsets_cbor).unwrap();
    std::fs::write("synsets.cbor", synsets_cbor.as_slice()).unwrap();

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
