extern crate day8a as lib;

use std::env;
use std::process;
use std::collections::HashMap;

use lib::config::Config;
use lib::parser::{get_input, parse_input};
use lib::{Operator, Comparator, Calculation};

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    
    let input: String = get_input(&config).unwrap_or_else(|err| {
        println!("Problem getting input: {}", err);
        process::exit(1);
    });
    
    let ops: Vec<Calculation> = parse_input(&input);
    
    let mut mem: HashMap<&str, isize> = HashMap::new();
    
    // Eg: aj dec -520 if icd < 9
    
    // iterate ops
    for calc in ops.iter() {
        // println!("{:?}", calc);
        
        // get condition register entry: icd or 0
        let mut comp = *mem.entry(calc.condition_register).or_insert(0);
        let mut skip = true;
        
        // println!("compare val: {:?}", comp);
        
        // evaluate condition: if icd < 9
        match calc.comparator {
            Comparator::Equal => {
                if comp == calc.threshold { skip = false; }
            },
            Comparator::Nequal => {
                if comp != calc.threshold { skip = false; }
            },
            Comparator::Gequal => {
                if comp >= calc.threshold { skip = false; }
            },
            Comparator::Lequal => {
                if comp <= calc.threshold { skip = false; }
            },
            Comparator::Lesser => {
                if comp < calc.threshold { skip = false; }
            },
            Comparator::Greater => {
                if comp > calc.threshold { skip = false; }
            },
        };
        
        if skip {
            // println!("skip");
            continue;
        }
        
        // get register entry: aj or 0
        let entry = mem.entry(calc.register).or_insert(0);
        
        // apply operator: dec -520
        *entry += match calc.operator {
            Operator::Inc => calc.difference,
            Operator::Dec => -calc.difference,
        };
        
        // println!("update value: {:?}", entry);
    }
    
    println!("{:?}", mem);
    
    let answer = mem.values().max();
    println!("Answer: {:?}", answer);
}
