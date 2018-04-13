extern crate day10a as lib;

use std::{env, process};
use lib::{twist, config::Config, parser::{get_input, parse_input}};

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let input: String = get_input(&config).unwrap_or_else(|err| {
        println!("Problem getting input: {}", err);
        process::exit(1);
    });

    let lengths = parse_input(&input);

    let range = 0..256;
    let size = range.end as i32;
    let mut string: Vec<i32> = range.collect();

    let mut position = 0;
    let mut skip = 0;

    for &length in lengths.iter() {
        twist(position, length, &mut string);
        position = (position + length + skip) % size;
        skip += 1;
    }

    let answer = string[0] * string[1];
    println!("Answer: {:?}", answer);
}
