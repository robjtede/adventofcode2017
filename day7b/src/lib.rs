#[macro_use]
extern crate lazy_static;

use std::collections::BTreeSet;
use std::fmt::{Display, Formatter, Error};

pub mod config;
pub mod parser;
pub mod tests;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Program {
    pub name: String,
    pub weight: usize,
    children: BTreeSet<String>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProgramTree {
    pub root: Program,
    pub children: BTreeSet<ProgramTree>
}

impl ProgramTree {
    pub fn weight(&self) -> usize {
        let own_weight = self.root.weight;
        let children_weight = self.children.iter().fold(0, |acc, p| {
            acc + p.weight()
        });
        
        own_weight + children_weight
    }
    
    pub fn check_balance(&self) -> bool {
        let mut iter = self.children.iter();
        
        if self.children.iter().count() == 0 {
            true
        } else {
            let init = iter.next().unwrap().weight();

            if let Some(x) = iter.find(|x| x.weight() != init) {
                println!("{}", self);
                self.children.iter().for_each(|x| println!("-> {}", x));
                self.children.iter().next().unwrap().check_balance();
                x.check_balance();
                false
            } else {
                true
            }
        }
    }
}

impl Display for ProgramTree {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}: {} ({})", &self.root.name, &self.root.weight, &self.weight())
    }
}

pub fn build_tree(root: Program, programs: &BTreeSet<Program>) -> ProgramTree {
    let subtrees: BTreeSet<ProgramTree> = root
        .children
        .iter()
        .map(|child_name| {
            let child = get_program(child_name, &programs).unwrap().clone();
            
            build_tree(child, programs)
        })
        .collect();
    
    ProgramTree {
        root: root.clone(),
        children: subtrees,
    }
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

fn find_parent_for<'a>(program: &'a Program, programs: &'a BTreeSet<Program>) -> Option<&'a Program> {
    for parent in programs.iter() {
        if is_child(program, parent) {
            return Some(parent)
        }
    }

    None
}

fn get_program<'a>(name: &str, programs: &'a BTreeSet<Program>) -> Option<&'a Program> {
    programs.iter().find(|x| {
        x.name == name
    })
}


fn is_child(child: &Program, candidate: &Program) -> bool {
    match candidate.children.iter().find(|x| {
        **x == child.name
    }) {
        Some(_) => true,
        None => false,
    }
}
