extern crate day4a;

use std::env;
use std::process;
use std::collections::HashSet;

use day4a::config::Config;
use day4a::parser::{get_input, parse_input};

fn main() {
    let config: Config = Config::new(env::args()).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let input = get_input(&config).unwrap_or_else(|err| {
        println!("Problem getting input: {}", err);
        process::exit(1);
    });

    let phrases: Vec<String> = parse_input(&input);

    let counter = phrases.iter().fold(0, |acc, phrase| {
        return acc + match validate_passphrase(phrase) {
            true => 1,
            false => 0
        }
    });

    let answer = counter;
    println!("{:?} out of {:?} passphrases are valid", answer, phrases.len());
}

fn validate_passphrase(phrase: &str) -> bool {
    let words: Vec<&str> = phrase.split(" ").collect();
    let mut set: HashSet<&str> = HashSet::new();

    words.iter().for_each(|&w| { set.insert(w); });

    return words.len() == set.len();
}
