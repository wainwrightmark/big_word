use clap::{Parser, Subcommand};

use crate::{check_words::CheckWordArgs, vector::TestVectorArgs, vector_db::TestVectorDBArgs};

pub mod check_words;
pub mod generate_words;
pub mod vector;
pub mod vector_db;
pub mod wordnet;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::GenerateWords => crate::generate_words::generate_words_and_synsets(),
        Commands::GenerateVectors => crate::vector::generate_vectors2(),
        Commands::CheckWords(args) => crate::check_words::check_words(args),
        Commands::TestVectors(test_vector_args) => {
            crate::vector::test_vectors2(test_vector_args);
        }
        Commands::GenerateVectorDB => crate::vector_db::generate_vector_db(),
        Commands::TestVectorDB(args) => {
            crate::vector_db::test_vector_db(args)
        }
        Commands::TestClosestVectors(args) => {
            crate::vector_db::test_closest_vectors(args)
        }
    }
}

#[derive(Parser, Debug, Default)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Default, Subcommand, Debug)]
pub enum Commands {
    #[default]
    GenerateWords,
    GenerateVectors,
    CheckWords(CheckWordArgs),
    TestVectors(TestVectorArgs),

    GenerateVectorDB,

    TestVectorDB(TestVectorDBArgs),
    TestClosestVectors(TestVectorDBArgs),
}
