extern crate day3b;

use std::collections::HashMap;

use day3b::position::Position;
use day3b::{get_pos, sum_perimeter};

fn main() {
    let target = 289326;

    let mut spiral: HashMap<Position, i32> = HashMap::new();

    spiral.insert(Position::new(0, 0), 1);

    let mut index = 2;
    let mut result = 0;

    while result < target {
        let center = get_pos(&index);
        let sum = sum_perimeter(&spiral, &center);

        println!("{}, {:?}: {}", index, center, sum);

        &spiral.insert(center, sum);

        index += 1;
        result = sum;
    }

    let answer = result;
    println!("Answer: {}", answer);
}
