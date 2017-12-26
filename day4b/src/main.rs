extern crate day4b;

use std::env;
use std::process;
use std::collections::{BTreeMap, HashSet};
use std::iter::FromIterator;

use day4b::config::Config;
use day4b::parser::{get_input, parse_input};

type CharCount = BTreeMap<char, i32>;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let input = get_input(&config).unwrap_or_else(|err| {
        println!("Problem getting input: {}", err);
        process::exit(1);
    });

    let phrases: Vec<String> = parse_input(&input);

    let counter = phrases.iter().fold(0, |acc, phrase| {
        acc + validate_passphrase(phrase) as i32
    });

    let answer = counter;
    println!("{:?} out of {:?} passphrases are valid", answer, phrases.len());
}

fn validate_passphrase(phrase: &str) -> bool {
    let words: Vec<&str> = phrase.split(" ").collect();

    // BTreeMap instead of HashMap since BTreeMap already impls Hash
    let mut freqs: Vec<CharCount> = Vec::new();

    words.iter().for_each(|&word| {
        let mut word_freq: CharCount = BTreeMap::new();

        word.chars().for_each(|letter| {
            *word_freq.entry(letter).or_insert(0) += 1;
        });

        freqs.push(word_freq);
    });

    let set: HashSet<CharCount> = HashSet::from_iter(freqs);

    words.len() == set.len()
}
