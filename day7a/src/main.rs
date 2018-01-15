extern crate day7a as lib;

use std::env;
use std::process;

use lib::config::Config;
use lib::parser::{get_input, parse_input};
use lib::{find_root_program};

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    
    let input = get_input(&config).unwrap_or_else(|err| {
        println!("Problem getting input: {}", err);
        process::exit(1);
    });
    
    let programs = parse_input(&input);
    
    let root_program = find_root_program(&programs);
    
    let answer = &root_program.name;
    println!("Root rogram name: {}", answer);
}
