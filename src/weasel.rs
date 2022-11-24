//! The `weasel` crate provides the public functions to run the Weasel genetic
//! algorithm and the private helper functions/structs used in this process
//!
//! In addition, the `tests` module is included here to test the helper methods
use rand::{
    Rng,
    seq::IteratorRandom,
};
use std::error::Error;

use crate::WeaselArgs;

/// Contains a mutation and it's `score`, or how many chars it shares with the
/// target phrase
#[derive(Clone)]
struct Mutation(String, usize);

/// Runs the Weasel genetic algorithm
///
/// # Arguments
///
/// * `args` - A WeaselArgs struct with the target phrase, approved char set,
/// and variations per generation
///
/// # Return
/// A Result with either the unit value or an std::error::Error
pub fn run_weasel(args: WeaselArgs) -> Result<(), Box<dyn Error>> {
    // Represents the best mutation from the last generation, random on gen 0
    let prev_best_str = gen_rand_string(args.phrase.len(), &args.char_set)?;
    let prev_best_score = check_equal_chars(&args.phrase, &prev_best_str);
    let mut prev_best = Mutation(prev_best_str, prev_best_score);
    let mut gen_num = 0;
    println!("Start: {}", prev_best.0);

    while prev_best.0 != args.phrase {
        let mut cur_best = prev_best.clone();

        for _ in 0..args.iterations {
            let cur_str = get_mutated_string(
                args.mutation_rate,
                &prev_best.0,
                &args.char_set
            )?;

            let cur_score = check_equal_chars(&cur_str, &args.phrase);

            // Check if this mutation is the new best for the generation
            if cur_score > cur_best.1 {
                cur_best = Mutation(cur_str, cur_score);
            }

        }

        prev_best = cur_best;
        println!("Gen {}: {}", gen_num, prev_best.0);

        gen_num += 1;
    }

    Ok(())
}

/// Generates a String of a given length with randomly chosen characters from a
/// character set
///
/// # Arguments
/// * `str_len` - A usize with the desired length of the final String
/// * `char_set` - A &ToString type containing all the chars that can be placed
/// in the final String
///
/// # Return
/// A Result with either the String or an std::error::Error
fn gen_rand_string<T: ToString>(str_len: usize, char_set: &T) 
    -> Result<String, Box<dyn Error>> {

    let mut rng = rand::thread_rng();

    let char_set = char_set.to_string();
    let mut rand_string = String::new();

    for _ in 0..str_len {
        rand_string.push(
            char_set
                .chars()
                .choose(&mut rng)
                .ok_or(r"Couldn't pick character from char set when generating random string")?
        );
    }

    Ok(rand_string)
}

/// Generate a String based on another String with a given random chance of
/// mutation on each char within the bands of a character set
///
/// # Arguments
/// * `mutation_rate`: A u8 value between [1-100] with the percentage rate
/// characters will be mutated
/// * `base_str`: A &ToString type the final string will be mutated from
/// * `char_set` - A &ToString type containing all the chars that can be used
/// for mutations
///
/// # Return
/// A Result with either the String or an std::error::Error
///
/// # Examples
/// ```
/// // The following could result in 'HELMO' or 'HDLLO' or some similar mutation
/// println!("{}", get_mutated_string(5, "HELLO", "ABCDEFGHIJKLMNOP"));
/// ```
fn get_mutated_string<T: ToString>(
        mutation_rate: u8,
        base_str: &T,
        char_set: &T
    ) -> Result<String, Box<dyn Error>> {

    let mut rng = rand::thread_rng();
    let mut mutated_str = String::new();

    // Copy the previous string to the new one char by char with a chance of
    // mutation on each char
    for c in base_str.to_string().chars() {
        let mut_roll = rng.gen_range(0..=100);

        // Check if you need to mutate
        if mut_roll <= mutation_rate {
            mutated_str.push(
                char_set
                    .to_string()
                    .chars()
                    .choose(&mut rng)
                    .ok_or("Couldn't get random character to mutate string")?
            );

            continue;
        }

        mutated_str.push(c);
    }

    Ok(mutated_str)
}

/// Compare two Strings char by char and count how many are the same
///
/// # Arguments
/// * `string_a`: A &ToString with the first value to be compared
/// * `string_b`: A &ToString with the second value to be compared
///
/// # Return
/// A usize with the number of chars shared between the two Strings
///
/// # Examples
/// ```
/// let string_a = String::from("FOO");
/// let string_b = String::from("OOF");
/// assert_eq!(check_equal_chars(&string_a, &string_b), 1);
/// ```
/// ```
/// // The lengths don't have to be equal
/// let string_a = String::from("FOOBAR");
/// let string_b = String::from("FOO");
/// assert_eq!(check_equal_chars(&string_a, &string_b), 3);
/// ```
fn check_equal_chars<T: ToString>(string_a: &T, string_b: &T) -> usize {
    let string_a = string_a.to_string();
    let string_b = string_b.to_string();

    string_a
        .chars()
        .zip(string_b.chars())
        .filter(|(a, b)| a == b)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen_rand_string() {
        let str_len = 10;
        let char_set = String::
            from("ABCDEFGHIJKLMNOPQRSTUVWXYZ abcdefghijklmnopqrstuvwxyz!?.");

        let test_str = gen_rand_string(str_len, &char_set)
            .expect("Problem generating random String!");

        for c in test_str.chars() {
            if !char_set.contains(c) {
                panic!("method uses chars outside defined char set");
            }
        }

        assert_eq!(test_str.len(), str_len);
    }

    #[test]
    fn test_get_mutated_string() {
        let test_str = String::from("HELLO");
        let char_set = String::from("ABCDEFGHIJKLMNOP");
        let mut_str = get_mutated_string(5, &test_str, &char_set)
            .expect("Problem getting mutated String!");

        assert_eq!(mut_str.len(), test_str.len());
    }

    #[test]
    fn test_same_len_string_equality() {
        let string_a = String::from("FOO");
        let string_b = String::from("OOF");

        assert_eq!(check_equal_chars(&string_a, &string_b), 1);
    }

    #[test]
    fn test_diff_len_string_equality() {
        let string_a = String::from("FOO");
        let string_b = String::from("FOOBAR");

        assert_eq!(check_equal_chars(&string_a, &string_b), 3);
    }
}
