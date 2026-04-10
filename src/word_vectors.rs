use serde::{Deserialize, Serialize};

use crate::{VECTOR_DIM, WordChars};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WordVectors {
    pub word: WordChars,
    #[serde(with = "serde_bytes")]
    pub bytes: [u8; VECTOR_DIM],
}

impl WordVectors {
    pub fn distance_from_squared(&self, other: &Self) -> u32 {
        let mut sum = 0;
        

        for index in 0..VECTOR_DIM {
            //let a = if self.bytes[index] < 128 {0u8} else {1};
            //let b = if other.bytes[index] < 128 {0u8} else {1};

            let a = self.bytes[index];
            let b = other.bytes[index];
            let d = a.abs_diff(b);
            sum += (d * d) as u32;
        }

        sum
    }
}
