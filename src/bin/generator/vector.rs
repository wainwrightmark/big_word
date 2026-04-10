use std::{
    collections::{HashMap, HashSet},
    io::{BufRead, BufReader, Read},
};

use clap::Args;

use big_word::{VECTOR_DIM, Word, WordChars, word_vectors::WordVectorsF32};

// pub fn generate_vectors() {
//     let path = r#"C:\Source\ML_Models\GoogleNews-vectors-negative300.bin"#;

//     //let bytes = std::fs::read(path).unwrap();
//     let file = std::fs::File::open(path).unwrap();
//     let mut reader = BufReader::new(file);

//     reader.skip_until('\n' as u8).unwrap();

//     let mut entries: HashMap<WordChars, [f32; VECTOR_DIM]> = HashMap::new();
//     let mut vector_bytes = [0u8; VECTOR_DIM * 4];
//     let mut ranges = Ranges::new();

//     'read_data: loop {
//         let mut word_bytes: Vec<u8> = Vec::new();
//         match reader.read_until(' ' as u8, &mut word_bytes) {
//             Ok(_) => {}
//             Err(err) => {
//                 println!("{err}");
//                 break 'read_data;
//             }
//         }
//         if word_bytes.is_empty() {
//             break 'read_data;
//         }
//         word_bytes.remove(word_bytes.len() - 1);
//         let word = String::from_utf8(word_bytes).unwrap();

//         reader.read_exact(&mut vector_bytes).unwrap();

//         let vector: [f32; VECTOR_DIM] = std::array::from_fn(|i| {
//             let i = i * 4;
//             f32::from_ne_bytes([
//                 vector_bytes[i],
//                 vector_bytes[i + 1],
//                 vector_bytes[i + 2],
//                 vector_bytes[i + 3],
//             ])
//         });

//         ranges.include(vector);

//         entries.entry(WordChars::format(&word)).or_insert(vector);
//     }

//     println!("{} entries", entries.len());

//     // let means: [f32; 300] = sums.map(|x| (x / entries.len() as f64) as f32);

//     let input_words = std::fs::File::open("words.cbor").unwrap();
//     let input_words: Vec<Word> = ciborium::from_reader(input_words).unwrap();

//     let mut output_words: Vec<WordVectorsF32> = vec![];
//     let mut words_found = 0;

//     for word in input_words.into_iter() {
//         let word = WordChars::format(&word.text);
//         if let Some(vector) = entries.get(&word) {
//             let bytes = ranges.convert(vector);
//             output_words.push(WordVectorsF32 { word, bytes });
//             words_found += 1;
//         }
//     }

//     println!("{} words found", words_found);

//     output_words.sort_by_key(|x| x.word.clone());

//     let mut vectors_cbor = vec![];
//     ciborium::ser::into_writer(&output_words, &mut vectors_cbor).unwrap();
//     std::fs::write("vectors.cbor", vectors_cbor.as_slice()).unwrap();

//     // let vectors_yaml = serde_yaml::to_string(&output_words).unwrap();
//     // std::fs::write("vectors.yaml", vectors_yaml.as_str()).unwrap();
// }

// pub fn generate_vectors2() {
//     let path = r#"C:\Source\ML_Models\Glove\wiki_giga_2024_300_MFT20_vectors_seed_2024_alpha_0.75_eta_0.05_combined.txt"#;

//     //let bytes = std::fs::read(path).unwrap();
//     let file = std::fs::File::open(path).unwrap();
//     let mut reader = BufReader::new(file);

//     reader.skip_until('\n' as u8).unwrap();

//     let mut entries: HashMap<WordChars, [f32; VECTOR_DIM]> = HashMap::new();

//     let mut ranges = Ranges::new();

//     for line in reader.lines() {
//         let line = line.unwrap();
//         let mut split = line.split(" ");
//         if let Some(word) = split.next() {
//             let mut arr = [0f32; VECTOR_DIM];
//             for (index, num) in split.enumerate() {
//                 let num = num.parse::<f32>().unwrap();
//                 arr[index] = num;
//             }
//             ranges.include(arr);
//             entries.insert(WordChars::format(word), arr);

//         }
//     }

//     println!("{} entries", entries.len());

//     // let means: [f32; 300] = sums.map(|x| (x / entries.len() as f64) as f32);

//     let input_words = std::fs::File::open("words.cbor").unwrap();
//     let input_words: Vec<Word> = ciborium::from_reader(input_words).unwrap();

//     let mut output_words: Vec<WordVectorsF32> = vec![];
//     let mut words_found = 0;

//     for word in input_words.into_iter() {
//         let word = WordChars::format(&word.text);
//         if let Some(vector) = entries.get(&word) {
//             let bytes = ranges.convert(vector);
//             //println!("{}: {}", word.as_str(), bytes.map(|x|x.to_string()) .join(", "));
//             output_words.push(WordVectorsF32 { word, bytes });
//             words_found += 1;
//         }
//     }

//     println!("{} words found", words_found);

//     output_words.sort_by_key(|x| x.word.clone());

//     let mut vectors_cbor = vec![];
//     ciborium::ser::into_writer(&output_words, &mut vectors_cbor).unwrap();
//     std::fs::write("vectors.cbor", vectors_cbor.as_slice()).unwrap();

//     // let vectors_yaml = serde_yaml::to_string(&output_words).unwrap();
//     // std::fs::write("vectors.yaml", vectors_yaml.as_str()).unwrap();
// }

pub fn generate_vectors2() {
    let path = r#"C:\Source\ML_Models\Glove\wiki_giga_2024_300_MFT20_vectors_seed_2024_alpha_0.75_eta_0.05_combined.txt"#;

    //let bytes = std::fs::read(path).unwrap();
    let file = std::fs::File::open(path).unwrap();
    let mut reader = BufReader::new(file);

    reader.skip_until('\n' as u8).unwrap();

    let mut entries: HashMap<WordChars, [f32; VECTOR_DIM]> = HashMap::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut split = line.split(" ");
        if let Some(word) = split.next() {
            let mut arr = [0f32; VECTOR_DIM];
            for (index, num) in split.enumerate() {
                let num = num.parse::<f32>().unwrap();
                arr[index] = num;
            }
            entries.insert(WordChars::format(word), arr);
        }
    }

    println!("{} entries", entries.len());

    // let means: [f32; 300] = sums.map(|x| (x / entries.len() as f64) as f32);

    let input_words = std::fs::File::open("words.cbor").unwrap();
    let input_words: Vec<Word> = ciborium::from_reader(input_words).unwrap();

    let mut output_words: Vec<WordVectorsF32> = vec![];
    let mut words_found = 0;

    for word in input_words.into_iter() {
        let word = WordChars::format(&word.text);
        if let Some(data) = entries.remove(&word) {
            //let bytes = ranges.convert(vector);
            //println!("{}: {}", word.as_str(), bytes.map(|x|x.to_string()) .join(", "));
            output_words.push(WordVectorsF32 { word, data });
            words_found += 1;
        }
    }

    println!("{} words found", words_found);

    output_words.sort_by_key(|x| x.word.clone());

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

#[derive(Args, Debug)]
pub struct TestVectorArgs {
    pub master: String,
    pub words: Vec<String>,
}

pub fn test_vectors1(args: TestVectorArgs) {
    let path = r#"C:\Source\ML_Models\GoogleNews-vectors-negative300.bin"#;

    //let bytes = std::fs::read(path).unwrap();
    let file = std::fs::File::open(path).unwrap();
    let mut reader = BufReader::new(file);

    reader.skip_until('\n' as u8).unwrap();

    let mut entries: HashMap<WordChars, [f32; 300]> = HashMap::new();
    let mut vector_bytes = [0u8; VECTOR_DIM * 4];
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

        entries.entry(WordChars::format(&word)).or_insert(vector);
    }

    // let file = std::fs::File::open("vectors.cbor").unwrap();
    // let word_vectors: Vec<WordVectors> = ciborium::from_reader(file).unwrap();

    let master_key: WordChars = WordChars::format(&args.master);
    let word_keys: HashSet<WordChars> = args.words.iter().map(|x| WordChars::format(x)).collect();

    let master_vector = entries.get(&master_key).unwrap();

    let mut vec: Vec<(WordChars, f32)> = args
        .words
        .iter()
        .map(|x| WordChars::format(x))
        .flat_map(|x| {
            entries
                .get(&x)
                .map(|y| (x, calculate_distance(y, &master_vector)))
        })
        .collect();

    vec.sort_by(|a, b| a.1.total_cmp(&b.1));

    for (wv, distance) in vec {
        println!("{wv:16?}: {distance}")
    }
}

pub fn test_vectors2(args: TestVectorArgs) {
    let path = r#"C:\Source\ML_Models\Glove\wiki_giga_2024_300_MFT20_vectors_seed_2024_alpha_0.75_eta_0.05_combined.txt"#;

    let words_to_check: HashSet<String> = args
        .words
        .iter()
        .cloned()
        .chain([args.master.clone()])
        .map(|x| x.to_lowercase())
        .collect();

    println!("{} words to check", words_to_check.len());

    let file = std::fs::File::open(path).unwrap();

    let mut reader = BufReader::new(file);
    let mut entries: HashMap<WordChars, [f32; VECTOR_DIM]> = HashMap::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut split = line.split(" ");
        if let Some(word) = split.next() {
            if words_to_check.contains(&word.to_lowercase()) {
                //println!("Found {word}");
                let mut arr = [0f32; VECTOR_DIM];
                for (index, num) in split.enumerate() {
                    let num = num.parse::<f32>().unwrap();
                    arr[index] = num;
                }
                entries.insert(WordChars::format(word), arr);
            } else {
                //println!("Do not use '{word}'")
            }
        }
    }

    let master_key: WordChars = WordChars::format(&args.master);
    let word_keys: HashSet<WordChars> = args.words.iter().map(|x| WordChars::format(x)).collect();

    let master_vector = entries
        .get(&master_key)
        .expect("Could not get master vector");

    let mut vec: Vec<(WordChars, f32)> = args
        .words
        .iter()
        .map(|x| WordChars::format(x))
        .flat_map(|x| {
            entries
                .get(&x)
                .map(|y| (x, calculate_distance(y, &master_vector)))
        })
        .collect();

    vec.sort_by(|a, b| a.1.total_cmp(&b.1));

    for (wv, distance) in vec {
        println!("{:16}: {distance}", wv.to_string())
    }
}

fn calculate_distance(a: &[f32; VECTOR_DIM], b: &[f32; VECTOR_DIM]) -> f32 {
    let mut sum = 0f32;

    for index in 0..VECTOR_DIM {
        let a = a[index];
        let b = b[index];
        let d = (a - b).abs();
        sum += (d * d) as f32;
    }

    sum
}
