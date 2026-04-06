use arrayvec::ArrayVec;

use crate::character::Character;

pub mod character;

///Note good breakpoints are 16 and 24
pub const MAX_WORD_LENGTH: usize = 24;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct WordChars(pub ArrayVec<Character, MAX_WORD_LENGTH>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PartOfSpeech {
    /// An substantive
    Noun,
    /// a word that describes a noun
    Adjective,
    /// a word that describes an action
    Verb,
    /// a word that describes a verb
    Adverb,
}

pub struct BasicWordList {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WordId(u32);

impl WordId {
    pub fn new(_0: u32) -> Self {
        Self(_0)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SynsetId(u32);

impl SynsetId {
    pub fn new(_0: u32) -> Self {
        Self(_0)
    }
}

#[derive(Debug)]
pub struct Word {
    pub id: WordId,
    /// The Word ranked by popularity.
    /// The most popular word is rank 0.
    pub popularity: u32,
    pub text: String,
    pub chars: WordChars,
    /// If this word is an inflected form, the bare form of the word
    pub bare_forms: Vec<WordChars>,

    pub synsets: Vec<SynsetId>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SynSet {
    pub id: SynsetId,
    pub definition: String,
    pub part_of_speech: PartOfSpeech,
    pub words: Vec<WordId>,

    pub relations: Vec<SynsetRelation>,
    //todo relationships
    //pub words: Vec<>
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SynsetRelType {
    /// an opposite word
    Antonym,
    /// broader forms of this word (a *structure* is a hypernym of a *building*)
    Hypernym,
    /// broader forms of this word of which this word is a specific instance
    /// (*The Enlightenment* is a specific instance of a *historic period*)
    InstanceHypernym,
    /// more specific versions of this word (a *courthouse* is a hyponym of a *house*)
    Hyponym,
    /// this word is a member of (the *world* is a hyponym of the *solar system*)
    MemberHolonym,
    /// this word is made with (*tin* is a substance holonym of *cassiterite*)
    SubstanceHolonym,
    /// this word is a part of (*land* is a part holonym of the *world*)
    PartHolonym,
    /// reverse of MemberHolonym (an *air bag* is a member meronym of *car*)
    MemberMeronym,
    /// reverse of SubstanceHolonym (*cassiterite* is a substance meronym of *tin*)
    SubstanceMeronym,
    /// reverse of PartHolonym (a *car* is a part holonym of an *air bag*)
    PartMeronym,
    /// *scientific* is an attribute of *scientific knowledge*
    Attribute,
    /// the word is related to (the adjective *outward* is an related to *outwardness*)
    DerivationallyRelated,
    ///
    DomainOfTopic,
    ///
    MemberOfTopic,
    ///
    DomainOfRegion,
    ///
    MemberOfRegion,
    ///
    DomainOfUsage,
    ///
    MemberOfUsage,

    /// A verb requires an action to be completed first (to *eat* requires one to *chew*)
    Entailment,
    /// A verb causes another action (to *retire* causes one to *yield*)
    Cause,
    ///
    AlsoSee,
    ///
    VerbGroup,

    ///
    SimilarTo,
    ///
    VerbParticiple,

    ///
    PertainymOrDerivedFromAdjective, // fixme
}

#[derive(Debug, Clone, PartialEq)]
pub struct SynsetRelation {
    pub to_id: SynsetId,
    pub synset_rel_type: SynsetRelType,
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
