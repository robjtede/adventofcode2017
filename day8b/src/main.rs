extern crate day8b as lib;

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
    
    let mut max: isize = 0;
    let mut mem: HashMap<&str, isize> = HashMap::new();
    
    // Eg: aj dec -520 if icd < 9
    
    // iterate ops
    for calc in ops.iter() {
        // println!("{:?}", calc);
        let skip = {
            // get condition register entry: icd or 0
            let comp = mem.entry(calc.condition_register).or_insert(0);
            
            // println!("compare val: {:?}", *comp);
            
            // evaluate condition: if icd < 9
            let satisfied = match calc.comparator {
                Comparator::Equal =>   *comp == calc.threshold,
                Comparator::Nequal =>  *comp != calc.threshold,
                Comparator::Gequal =>  *comp >= calc.threshold,
                Comparator::Lequal =>  *comp <= calc.threshold,
                Comparator::Lesser =>  *comp <  calc.threshold,
                Comparator::Greater => *comp >  calc.threshold,
            };
            
            !satisfied
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
        
        if *entry > max {
            max = *entry;
        }
        
        // println!("update value: {:?}", entry);
    }
    
    println!("{:?}", mem);
    
    let answer = max;
    println!("Answer: {:?}", answer);
}
