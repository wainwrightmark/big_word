use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read},
};

use big_word::{VECTOR_DIM, Word, WordChars, word_vectors::WordVectors};

pub fn generate_vectors() {
    let path = r#"C:\Source\ML_Models\GoogleNews-vectors-negative300.bin"#;

    //let bytes = std::fs::read(path).unwrap();
    let file = std::fs::File::open(path).unwrap();
    let mut reader = BufReader::new(file);

    reader.skip_until('\n' as u8).unwrap();

    let mut entries: HashMap<WordChars, [f32; VECTOR_DIM]> = HashMap::new();
    let mut vector_bytes = [0u8; VECTOR_DIM * 4];
    let mut ranges = Ranges::new();

    'read_data: loop {
        let mut word_bytes: Vec<u8> = Vec::new();
        match reader.read_until(' ' as u8, &mut word_bytes) {
            Ok(_) => {}
            Err(err) => {
                println!("{err}");
                break 'read_data;
            }
        }
        if word_bytes.is_empty() {
            break 'read_data;
        }
        word_bytes.remove(word_bytes.len() - 1);
        let word = String::from_utf8(word_bytes).unwrap();

        reader.read_exact(&mut vector_bytes).unwrap();

        let vector: [f32; VECTOR_DIM] = std::array::from_fn(|i| {
            let i = i * 4;
            f32::from_ne_bytes([
                vector_bytes[i],
                vector_bytes[i + 1],
                vector_bytes[i + 2],
                vector_bytes[i + 3],
            ])
        });

        ranges.include(vector);

        entries.entry(WordChars::format(&word)).or_insert(vector);
    }

    println!("{} entries", entries.len());

    // let means: [f32; 300] = sums.map(|x| (x / entries.len() as f64) as f32);

    let input_words = std::fs::File::open("words.cbor").unwrap();
    let input_words: Vec<Word> = ciborium::from_reader(input_words).unwrap();

    let mut output_words: Vec<WordVectors> = vec![];

    for word in input_words.into_iter() {
        let word = WordChars::format(&word.text);
        if let Some(vector) = entries.get(&word) {
            let bytes = ranges.convert(vector);
            // let mut data = [0u8; DIM];
            // for index in 0..DIM {
            //     let byte = index / 8;
            //     let shift = index % 8;
            //     let bit = (vector[index] >= means[index]) as u8;
            //     data[byte] |= bit << shift;
            // }
            output_words.push(WordVectors { word, bytes });
        }
    }

    let mut vectors_cbor = vec![];
    ciborium::ser::into_writer(&output_words, &mut vectors_cbor).unwrap();
    std::fs::write("vectors.cbor", vectors_cbor.as_slice()).unwrap();

    // let vectors_yaml = serde_yaml::to_string(&output_words).unwrap();
    // std::fs::write("vectors.yaml", vectors_yaml.as_str()).unwrap();
}

#[derive(Debug)]
pub struct Ranges([(f32, f32); VECTOR_DIM]);

impl Ranges {
    pub fn new() -> Self {
        Self([(f32::MAX, f32::MIN); VECTOR_DIM])
    }

    pub fn include(&mut self, data: [f32; VECTOR_DIM]) {
        for index in 0..VECTOR_DIM {
            let f = data[index];

            let (min, max) = self.0.get_mut(index).unwrap();
            if f < *min {
                *min = f
            }
            if f > *max {
                *max = f
            }
        }
    }

    pub fn convert(&self, vectors: &[f32; VECTOR_DIM]) -> [u8; VECTOR_DIM] {
        std::array::from_fn(|index| {
            let v = vectors[index];
            let (min, max) = self.0[index];

            let d = v - min;
            let range = max - min;
            //let proportion = ((d / range) * 256.0) - 128.0;
            let proportion = (d / range) * 256.0;

            proportion.round() as u8
        })
    }
}
