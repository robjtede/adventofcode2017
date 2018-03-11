use std::io::prelude::*;
use std::io::Error;
use std::fs::File;

use super::Token;

use config::Config;

pub fn get_input(config: &Config) -> Result<String, Error> {
    let mut file: File = File::open(&config.path)?;

    let mut contents: String = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

pub fn parse_input(input: &str) -> Vec<Token> {
    let mut token_list: Vec<Token> = vec![Token::Start];
    let mut in_garbage = false;
    let mut next_cancelled = false;
    
    for x in input.trim().chars() {
        if next_cancelled {
            next_cancelled = false
        } else {
            if x == '!' {
                next_cancelled = true;
                continue
            }
            
            if in_garbage {
                let token = match x {
                    '>' => {
                        in_garbage = false;
                        Token::GarbageStart
                    },
                    _ => Token::GarbageContent,
                };
                
                token_list.push(token)
            } else {
                let token = match x {
                    '{' => Token::GroupStart,
                    '}' => Token::GroupEnd,
                    '<' => {
                        in_garbage = true;
                        Token::GarbageStart
                    },
                    _ => Token::GroupContent,
                };
                
                token_list.push(token)
            }
        }

    }
    
    token_list
}
