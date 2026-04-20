use serde::{Deserialize, Serialize};

use crate::{VECTOR_DIM, WordChars, instant_distance::Point};
use serde_big_array::BigArray;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WordVectorsF32 {
    pub word: WordChars,
    #[serde(with = "BigArray")]
    pub data: [f32; VECTOR_DIM],
}

// impl WordVectorsF32 {
//     pub fn distance_from_squared(&self, other: &Self) -> f32 {
//         let mut sum = 0f32;

//         for index in 0..VECTOR_DIM {
//             //let a = if self.bytes[index] < 128 {0u8} else {1};
//             //let b = if other.bytes[index] < 128 {0u8} else {1};

//             let a = self.data[index];
//             let b = other.data[index];
//             let d = a - b;
//             sum += d * d;
//         }

//         sum
//     }
// }

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct WordVectorData {
    #[serde(with = "BigArray")]
    pub data: [f32; VECTOR_DIM],
}

impl Point for WordVectorData {
    fn distance(&self, other: &Self) -> f32 {
        calculate_cosine_similarity(&self.data, &other.data) * -1.0
    }
}

pub fn calculate_cosine_similarity<const N: usize>(a: &[f32; N], b: &[f32; N]) -> f32 {
    let mut product: f32 = 0f32;
    let mut aa: f32 = 0f32;
    let mut bb: f32 = 0f32;

    for index in 0..N {
        let a = a[index];
        let b = b[index];

        product += a * b;
        aa += a * a;
        bb += b * b;
    }

    let result = product / (aa * bb).sqrt();

    result
}

// pub fn calculate_normalized_euclidean_distance<const N: usize>(a: &[f32; N], b: &[f32; N]) -> f32 {
//     let mut total = 0.0;
//     for i in 0..N {
//         let d = self[i] - other[i];
//         total += d * d;
//     }
//     total
// }
