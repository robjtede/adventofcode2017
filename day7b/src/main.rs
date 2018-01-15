extern crate day7b as lib;

use std::env;
use std::process;

use lib::config::Config;
use lib::parser::{get_input, parse_input};
use lib::{build_tree, find_root_program};

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
    
    let root = find_root_program(&programs).clone();
    let tree = build_tree(root, &programs);
    
    &tree.check_balance();
    println!("See above. Answer unbalanced program weight minus total weight different.");
}
