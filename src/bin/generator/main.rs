use clap::{Parser, Subcommand};

use crate::vector::TestVectorArgs;

pub mod generate_words;
pub mod vector;
pub mod wordnet;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::GenerateWords => crate::generate_words::generate_words_and_synsets(),
        Commands::GenerateVectors => crate::vector::generate_vectors2(),
        Commands::TestVectors(test_vector_args) => {
            crate::vector::test_vectors2(test_vector_args);
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
    TestVectors(TestVectorArgs),
}
