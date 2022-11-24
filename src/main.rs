//! This program takes an input phrase, and runs the Weasel genetic algorithm to
//! reach that target phrase, outputting the best candidate in each generation
//!
//! # Examples
//! ```
//! // Here a mutation rate is provided of 10% for each candidate
//! $ ./weasel -p "Hello!" -m 10
//! Start: pqeIJu
//! Gen 0: pqeIJu
//! Gen 1: peeIJu
//! -- snip --
//! Gen 10: Helle!
//! Gen 11: Hello!
//! ```
use clap::Parser;

pub mod weasel;

/// Run the weasel genetic algorithm on a given phrase and approved character
/// set
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct WeaselArgs {
    /// The phrase to run the algorithm on
    #[arg(short, long)]
    phrase: String,

    /// The approved character set
    #[arg(short, long, default_value_t = 
        String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ abcdefghijklmnopqrstuvwxyz!?."
    ))]
    char_set: String,

    /// The number of variations to produce per generation, >= 1
    #[arg(short, long, default_value_t = 100)]
    iterations: u32,

    /// The mutation rate for each string, from 1-100
    #[arg(short, long, default_value_t = 5)]
    mutation_rate: u8,
}

fn main() {
    let args = WeaselArgs::parse();
    let mut exit_code = 0;

    // Check argument validity
    if args.mutation_rate > 100 || args.mutation_rate < 1 {
        eprintln!("Mutation value should be within [1-100], not {}",
            args.mutation_rate);
        exit_code = 1;
    } else if args.iterations < 1 {
        eprintln!("Iterations value should be >= 1, not {}", args.iterations);
        exit_code = 1;
    } else if let Err(e) = weasel::run_weasel(args) {
        eprintln!("{}", e);
        exit_code = 1;
    }

    std::process::exit(exit_code);
}
