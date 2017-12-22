pub mod position;

use std::collections::HashMap;

use position::Position;

pub fn get_pos(index: &i32) -> Position {
    let mut n: i32 = 1;
    while n.pow(2) < *index {
        n += 2;
    }

    let delta = index - (n - 2).pow(2);

    let layer = n / 2 + 1;
    let side = layer * 2 - 1;
    let len = side - 1;
    let mid = layer - 1;

    let x_offset = match delta {
        // right
        x if x <= len * 1 => mid,

        // top
        x if x <= len * 2 => mid - (delta - len * 1),

        // left
        x if x <= len * 3 => -mid,

        // bottom
        x if x <= len * 4 => (delta - len * 3) - mid,

        // unknown
        _ => 0,
    };

    let y_offset = match delta {
        // right
        x if x <= len * 1 => delta - mid,

        // top
        x if x <= len * 2 => mid,

        // left
        x if x <= len * 3 => mid - (delta - len * 2),

        // bottom
        x if x <= len * 4 => -mid,

        // unknown
        _ => 0,
    };

    return Position {
        x: x_offset,
        y: y_offset,
    };
}

pub fn sum_perimeter(spiral: &HashMap<Position, i32>, center: &Position) -> i32 {
    let dirs: Vec<Position> = vec![
        Position::new(1, 0),
        Position::new(1, 1),
        Position::new(0, 1),
        Position::new(-1, 1),
        Position::new(-1, 0),
        Position::new(-1, -1),
        Position::new(0, -1),
        Position::new(1, -1),
    ];

    let total = dirs
        .into_iter()
        .map(|offset| {
            let pos = center + offset;

            let val: &i32 = match spiral.get(&pos) {
                Some(v) => v,
                None => &0,
            };

            return val;
        })
        .sum();

    return total;
}
