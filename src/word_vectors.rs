use serde::{Deserialize, Serialize};

use crate::{VECTOR_DIM, WordChars, instant_distance::Point};
use serde_big_array::BigArray;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WordVectorsF32 {
    pub word: WordChars,
    #[serde(with = "BigArray")]
    pub data: [f32; VECTOR_DIM],
}

impl WordVectorsF32 {
    pub fn distance_from_squared(&self, other: &Self) -> f32 {
        let mut sum = 0f32;

        for index in 0..VECTOR_DIM {
            //let a = if self.bytes[index] < 128 {0u8} else {1};
            //let b = if other.bytes[index] < 128 {0u8} else {1};

            let a = self.data[index];
            let b = other.data[index];
            let d = a - b;
            sum += d * d;
        }

        sum
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct WordVectorData {
    #[serde(with = "BigArray")]
    pub data: [f32; VECTOR_DIM],
}

impl Point for WordVectorData {
    fn distance(&self, other: &Self) -> f32 {
        let mut sum = 0f32;

        for index in 0..VECTOR_DIM {

            let a = self.data[index];
            let b = other.data[index];
            let d = a - b;
            sum += d * d;
        }

        sum
    }
}
