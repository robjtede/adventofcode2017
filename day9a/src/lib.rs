pub mod config;
pub mod parser;
pub mod tests;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Token {
    Start,
    GroupStart,
    GroupContent,
    GroupEnd,
    GarbageStart,
    GarbageEnd,
}

pub fn simplify_stream(stream: &Vec<Token>) -> String {
    let mut simple_stream: String = String::from("");
    
    for x in stream {
        let token: &str = match x {
            &Token::GroupStart => "{",
            &Token::GroupEnd => "}",
            // &Token::GroupContent => "x",
            // &Token::GarbageStart => "<",
            // &Token::GarbageEnd => ">",
            _ => "",
        };
        
        simple_stream = [simple_stream, token.to_owned()].concat()
    }
    
    simple_stream
}

pub fn count_groups(stream: &Vec<Token>) -> usize {
    let mut open_count = 0;
    let mut closed_count = 0;
    
    for x in stream {
        if x == &Token::GroupStart {
            open_count += 1
        } else if x == &Token::GroupEnd {
            open_count -= 1;
            closed_count += 1
        }
        
        if open_count < 0 {
            panic!("groups are not balanced")
        }
    }
    
    if open_count > 0 {
        panic!("groups are not balanced")
    }
    
    closed_count
}

pub fn count_score(stream: &Vec<Token>) -> usize {
    let mut open_count = 0;
    let mut score = 0;
    
    for x in stream {
        if x == &Token::GroupStart {
            open_count += 1;
        } else if x == &Token::GroupEnd {
            score += open_count;
            open_count -= 1
        }
        
        if open_count < 0 {
            panic!("groups are not balanced")
        }
    }
    
    if open_count > 0 {
        panic!("groups are not balanced")
    }
    
    score
}
