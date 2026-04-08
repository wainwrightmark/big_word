pub mod word_vectors;

use std::num::NonZeroU32;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use strum::{EnumCount, EnumIs, EnumIter};
use ustr::Ustr;

// ///Note good breakpoints are 16 and 24
// pub const MAX_WORD_LENGTH: usize = 24;

pub const VECTOR_DIM: usize = 300;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash)]
#[serde(transparent)]
pub struct WordChars(Ustr);

impl std::ops::Deref for WordChars {
    type Target = Ustr;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl WordChars {
    /// Format a string as a WordChars
    pub fn format(s: &str) -> Self {
        if s.chars().all(|x| x.is_ascii_lowercase()) {
            return Self(Ustr::from(s));
        } else {
            let new_s = s
                .to_ascii_lowercase()
                .replace(|x: char| !x.is_ascii_lowercase(), "");
            Self(Ustr::from(&new_s))
        }
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    EnumIs,
    EnumCount,
    EnumIter,
    Serialize_repr,
    Deserialize_repr,
)]
#[repr(u8)]

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct SynsetId(u32);

impl SynsetId {
    pub fn new(_0: u32) -> Self {
        Self(_0)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Word {
    pub text: Ustr,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    /// The Word ranked by popularity.
    /// The most popular word is rank 1.
    /// Null if popularity is unknown
    pub popularity: Option<NonZeroU32>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    /// The different meanings of the word
    pub meanings: Vec<SynsetId>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    /// Is this word only an inflected form of another word
    pub root_forms: Vec<WordChars>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SynSet {
    pub id: SynsetId,
    pub definition: String,
    pub part_of_speech: PartOfSpeech,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub words: Vec<WordChars>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub relations: Vec<SynsetRelation>,
    //todo relationships
    //pub words: Vec<>
}





#[derive(
    Debug, Clone, Copy, PartialEq, Serialize_repr, Deserialize_repr, Eq, PartialOrd, Ord, Hash,
)]
#[repr(u8)]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
