pub mod config;
pub mod parser;
pub mod tests;

pub fn redistribute(banks: &Vec<i32>) -> Vec<i32> {
    let max: usize = *banks.iter().max().unwrap() as usize;
    let max_pos: usize = banks.iter().position(|&n| n as usize == max).unwrap();
    let len: usize = banks.len();
    
    let mut redist: Vec<i32> = banks.clone();
    redist[max_pos] = 0;
    
    for n in 1..max+1 {
        redist[(max_pos + n) % len] += 1;
    }
    
    redist
}
