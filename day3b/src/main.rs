extern crate day3b;

use std::collections::HashMap;

use day3b::{get_pos, sum_perimeter};

fn main() {
    let target = 289326;
    // let target = 27;
    let mut spiral: HashMap<(i32, i32), i32> = HashMap::new();
    
    spiral.insert((0, 0), 1);

    let mut index = 2;
    let mut result = 0;

    while result < target {

        let (k, v) = {
            (get_pos(&index), sum_perimeter(&spiral, &index))
        };
        
        println!("{:?}: {:?}: {:?}", index, k, v);
        
        &spiral.insert(k, v);

        index += 1;
        result = v;
        
        // result += 1;
    }

    let answer = result;
    println!("Answer: {}", answer);
}
