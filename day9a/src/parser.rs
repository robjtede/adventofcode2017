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
    let mut last_token: Token = Token::Start;
    let mut next_cancelled = false;
    
    for x in input.trim().chars() {
        if next_cancelled {
            next_cancelled = false
        } else {
            if x == '!' {
                next_cancelled = true;
                continue
            }
            
            if last_token != Token::GarbageStart {
                let token = match x {
                    '{' => Token::GroupStart,
                    '}' => Token::GroupEnd,
                    '<' => Token::GarbageStart,
                    '>' => Token::GarbageEnd,
                    _ => Token::GroupContent,
                };
                
                last_token = token;
                token_list.push(token)
            } else {
                if x == '>' {
                    last_token = Token::GarbageEnd;
                    token_list.push(Token::GarbageEnd)
                }
            }
        }

    }
    
    token_list
}
