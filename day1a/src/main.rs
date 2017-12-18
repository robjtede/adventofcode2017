use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents).expect(
        "something went wrong",
    );

    let nums = contents.trim();

    // accumulator
    let mut total: u32 = 0;

    // wrap around hack
    let mut last_num: u32 = nums.chars().last().unwrap_or('0').to_digit(10).unwrap();

    for (_, n) in nums.chars().enumerate() {
        let num = n.to_digit(10).unwrap_or(0);

        if num == last_num {
            total += last_num;
        }

        last_num = num
    }

    println!("{}", total);
}
