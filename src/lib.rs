use arrayvec::ArrayVec;
use ustr::Ustr;

use crate::character::Character;

mod character;

///Note good breakpoints are 16 and 24
pub const MAX_WORD_LENGTH: usize = 24;

#[derive(Debug, Clone,  PartialEq, Eq, PartialOrd, Ord)]
pub struct WordChars(ArrayVec<Character, MAX_WORD_LENGTH>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PartOfSpeech {
    Noun,
    Verb,
    Adjective,
    Adverb,
    Preposition,
    Conjunction,
    Interjection,
}

pub struct BasicWordList {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WordId(u32);
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SynsetIndex(u32);

#[derive(Debug)]
pub struct Word {
    pub word_index: WordId,
    /// The Word ranked by popularity.
    /// The most popular word is rank 0.
    pub popularity: u32,
    pub text: Ustr,
    pub chars: WordChars,
    /// If this word is an inflected form, the bare form of the word
    pub bare_forms: Vec<WordChars>,

    pub synsets: Vec<SynsetIndex>,
}

#[derive(Debug)]
pub struct SynSet {
    pub id: SynsetIndex,
    pub word: Vec<WordId>,
    pub definition: Ustr,
}

/* TODO

Basic Word List
Wordnet - includes part of speech
Derived words
Words By Commonness
WordVec Data
Dictionary Definitions
Pronunciation - syllables / ipa
Multi-word phrases


Program to generate all these files
Component to download all the files individually when needed

*/
