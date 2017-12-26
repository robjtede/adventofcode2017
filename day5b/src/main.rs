extern crate day5b;

use std::env;
use std::process;

use day5b::config::Config;
use day5b::parser::{get_input, parse_input};

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let input = get_input(&config).unwrap_or_else(|err| {
        println!("Problem getting input: {}", err);
        process::exit(1);
    });

    let mut jumps: Vec<i32> = parse_input(&input);
    let mut idx: i32 = 0;
    let mut count = 0;

    // idx as usize will wrap around if idx is negative
    while let Some(x) = jumps.iter_mut().nth(idx as usize) {
        idx += *x;

        if *x >= 3 {
            *x -= 1;
        } else {
            *x += 1;
        }

        count += 1;
    }

    let answer = count;
    println!("{:?} jumps", answer);
}
