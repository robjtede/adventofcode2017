extern crate day6a;

use std::env;
use std::process;
use std::collections::HashSet;

use day6a::config::Config;
use day6a::parser::{get_input, parse_input};
use day6a::redistribute;

type Bank = Vec<i32>;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    
    let input = get_input(&config).unwrap_or_else(|err| {
        println!("Problem getting input: {}", err);
        process::exit(1);
    });
    
    let mut bank: Bank = parse_input(&input);

    let mut banks_vec: Vec<Bank> = Vec::new();
    let mut banks_set: HashSet<Bank> = HashSet::new();
    
    while banks_vec.len() == banks_set.len() {
         let redist: Bank = redistribute(&bank);
         
         banks_vec.push(redist.clone());
         banks_set.insert(redist.clone());
         
         bank = redist;
    }

    println!("Vec length: {:?}", banks_vec.len());
    println!("Set length: {:?}", banks_set.len());
    
    let answer = banks_vec.len();
    println!("Cycles: {:?}", answer);
}
