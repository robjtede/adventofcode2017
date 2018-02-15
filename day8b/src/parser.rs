extern crate lazy_static;
extern crate regex;

use std::io::prelude::*;
use std::io::Error;
use std::fs::File;

use self::regex::Regex;

use super::Calculation;
use super::Operator;
use super::Comparator;
    
use config::Config;

pub fn get_input(config: &Config) -> Result<String, Error> {
    let mut file: File = File::open(&config.path)?;

    let mut contents: String = String::new();
    file.read_to_string(&mut contents)?;
    
    Ok(contents)
}

// aj dec -520 if icd < 9

pub fn parse_input(input: &str) -> Vec<Calculation> {
    lazy_static! {
        static ref PARTS_RE: Regex = Regex::new(r"^(\w+) (\w+) (-?\d+) if (\w+) ([!=<>]{1,2}) (-?\d+)$").unwrap();
    }
    
    input
        .trim()
        .split('\n')
        .map(|x| {
            let parts = PARTS_RE.captures(x).expect("regex not matched");
            
            let operator = match parts.get(2).unwrap().as_str() {
                "inc" => Operator::Inc,
                "dec" => Operator::Dec,
                _ => panic!("invalid operator in '{:?}'", x),
            };
            
            let comparator = match parts.get(5).unwrap().as_str() {
                "==" => Comparator::Equal,
                "!=" => Comparator::Nequal,
                "<=" => Comparator::Lequal,
                ">=" => Comparator::Gequal,
                "<" => Comparator::Lesser,
                ">" => Comparator::Greater,
                _ => panic!("invalid comparator in '{:?}'", x),
            };
            
            Calculation {
                register: parts.get(1).unwrap().as_str(),
                operator: operator,
                difference: parts.get(3).unwrap().as_str().parse::<isize>().unwrap(),
                condition_register: parts.get(4).unwrap().as_str(),
                comparator: comparator,
                threshold: parts.get(6).unwrap().as_str().parse::<isize>().unwrap(),
            }
        })
        .collect()
}
