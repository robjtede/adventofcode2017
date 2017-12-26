use std::io::prelude::*;
use std::io::Error;
use std::fs::File;

use config::Config;

pub fn get_input(config: &Config) -> Result<String, Error> {
    let mut file: File = File::open(&config.path)?;

    let mut contents: String = String::new();
    file.read_to_string(&mut contents)?;
    
    Ok(contents)
}

pub fn parse_input(input: &str) -> Vec<String> {
    let lines: Vec<String> = input
        .trim()
        .split('\n')
        .map(|x| x.to_string())
        .collect();

    lines
}
