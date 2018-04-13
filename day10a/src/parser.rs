use std::{fs::File, io::{prelude::*, Error}};

use config::Config;

pub fn get_input(config: &Config) -> Result<String, Error> {
    let mut file: File = File::open(&config.path)?;

    let mut contents: String = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

pub fn parse_input(input: &str) -> Vec<i32> {
    let lengths = input
        .trim()
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    lengths
}
