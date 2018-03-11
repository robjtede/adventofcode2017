extern crate day9b as lib;

use std::env;
use std::process;

use lib::count_garbage;
use lib::config::Config;
use lib::parser::{get_input, parse_input};

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    
    let input: String = get_input(&config).unwrap_or_else(|err| {
        println!("Problem getting input: {}", err);
        process::exit(1);
    });
    
    let token_stream = parse_input(&input);
    let score = count_garbage(&token_stream);
    
    let answer = score;
    println!("Answer: {}", answer);
}
