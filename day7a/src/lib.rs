#[macro_use]
extern crate lazy_static;

use std::collections::BTreeSet;

pub mod config;
pub mod parser;
pub mod tests;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Program {
    pub name: String,
    pub weight: usize,
    children: BTreeSet<String>,
}

/**
 * find_root_program
 *
 * A program is either the root program or is the child of another program.
 * So, take the first program and then find which program is its parent. Then
 * find that one's parent. Recurse until no parent can be found.
 *
 */
pub fn find_root_program(programs: &BTreeSet<Program>) -> &Program {
    let mut candidate: &Program = programs.iter().nth(0).unwrap();

    while let Some(prog) = find_parent_for(candidate, programs) {
        find_parent_for(prog, programs);
        candidate = prog;
    }

    candidate
}

fn find_parent_for<'a>(program: &Program, programs: &'a BTreeSet<Program>) -> Option<&'a Program> {
    for parent in programs.iter() {
        if is_child(program, parent) {
            return Some(parent)
        }
    }

    None
}

fn is_child(child: &Program, candidate: &Program) -> bool {
    match candidate.children.iter().find(|x| {
        if **x == child.name {
            true
        } else {
            false
        }
    }) {
        Some(_) => true,
        None => false,
    }
}
