use big_word::{
    instant_distance::{self, Point, Search},
    word_vectors::{WordVectorData, WordVectorsF32},
};
use clap::Args;
use itertools::Itertools;
use ordered_float::OrderedFloat;

#[derive(Args, Debug)]
pub struct TestVectorDBArgs {
    pub word: String,
}

pub fn generate_vector_db() {
    let input_vectors = std::fs::File::open("vectors.cbor").unwrap();
    let input_vectors: Vec<WordVectorsF32> = ciborium::from_reader(input_vectors).unwrap();

    let (points, values) = input_vectors
        .into_iter()
        .map(|x| (WordVectorData { data: x.data }, x.word.0))
        .unzip();

    let db: instant_distance::HnswMap<WordVectorData, ustr::Ustr> =
        instant_distance::Builder::default()
            .seed(123)
            .build(points, values);

    let mut data_cbor = vec![];
    ciborium::ser::into_writer(&db, &mut data_cbor).unwrap();

    std::fs::write("vector_db.cbor", data_cbor.as_slice()).unwrap();
}

pub fn test_vector_db(args: TestVectorDBArgs) {
    println!("Test vector db");
    let db = std::fs::File::open("vector_db.cbor").unwrap();
    let db: instant_distance::HnswMap<WordVectorData, ustr::Ustr> =
        ciborium::from_reader(db).unwrap();

    println!("Deserialized db");
    let key = args.word;

    let index = db
        .values
        .iter()
        .enumerate()
        .filter(|x| x.1.eq(&key))
        .map(|x| x.0)
        .next()
        .expect("Could not find word");

    println!("Key index {index}");
    let point = db.hnsw.points[index].clone();
    let mut search = Search::default();
    let iter = db.search(&point, &mut search).take(100);

    for x in iter {
        println!("{}", x.value);
    }
}


pub fn test_closest_vectors(args: TestVectorDBArgs) {
    let input_vectors = std::fs::File::open("vectors.cbor").unwrap();
    let input_vectors: Vec<WordVectorsF32> = ciborium::from_reader(input_vectors).unwrap();

    println!("Deserialized file");
    let key = args.word;

    let point = input_vectors
        .iter()
        .enumerate()
        .filter(|x| x.1.word.0.eq(&key))
        .map(|x| x.1.data)
        .next()
        .expect("Could not find word");


    let smallest = input_vectors.iter().k_smallest_by_key(100, |x|OrderedFloat(x.data.distance(&point)));

    //let closest_points = input_vectors

    // println!("Key index {index}");
    // let point = db.hnsw.points[index].clone();
    // let mut search = Search::default();
    // let iter  = db.search(&point, &mut search).take(100);

    for x in smallest{
        let d = x.data.distance(&point);
        println!("{} ({d:.2})", x.word.0);
    }
}
