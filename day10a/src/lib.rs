pub mod config;
pub mod parser;
pub mod tests;

pub fn twist(pos: i32, len: i32, string: &mut Vec<i32>) {
    let size = string.len() as i32;
    
    for i in 0..len/2 {
        let i1 = (pos + i) % size;
        let i2 = ((pos + len) - i - 1) % size;
        string.swap(i1 as usize, i2 as usize);
    }
}
