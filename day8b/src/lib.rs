#[macro_use]
extern crate lazy_static;

pub mod config;
pub mod parser;
pub mod tests;

#[derive(Debug)]
pub enum Operator {
    Inc,
    Dec,
}

#[derive(Debug)]
pub enum Comparator {
    Equal,
    Nequal,
    Greater,
    Lesser,
    Lequal,
    Gequal,
}

#[derive(Debug)]
pub struct Calculation<'a> {
    pub register: &'a str,
    pub operator: Operator,
    pub difference: isize,
    pub condition_register: &'a str,
    pub comparator: Comparator,
    pub threshold: isize
}
