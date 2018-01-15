extern crate lazy_static;
extern crate regex;

use std::io::prelude::*;
use std::io::Error;
use std::fs::File;
use std::collections::BTreeSet;

use self::regex::Regex;

use super::Program;
use config::Config;

pub fn get_input(config: &Config) -> Result<String, Error> {
    let mut file: File = File::open(&config.path)?;

    let mut contents: String = String::new();
    file.read_to_string(&mut contents)?;
    
    Ok(contents)
}

pub fn parse_input(input: &str) -> BTreeSet<Program> {
    input
        .trim()
        .split('\n')
        .map(|x| parse_line(x))
        .collect()
}

fn parse_line(line: &str) -> Program {
    lazy_static! {
        static ref PARTS_RE: Regex = Regex::new(r"^([^ ]+)\s\((\d+)\)\s?(.*)$").unwrap();
    }
    
    let parts = PARTS_RE.captures(&line).unwrap();
    
    let name = parts.get(1).unwrap().as_str().to_string();
    let weight = parts.get(2).unwrap().as_str().parse::<usize>().unwrap();
    let children_str = parts.get(3).unwrap().as_str();
    
    let children: BTreeSet<String> = {
        if children_str != "" {
            children_str[3..]
                .split(", ")
                .map(|x| x.to_string())
                .collect()
        } else {
            BTreeSet::new()
        }
    };

    Program {
        name: name,
        weight: weight,
        children: children
    }
}
