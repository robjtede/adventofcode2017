use std::collections::HashMap;

pub fn get_pos(index: &i32) -> (i32, i32) {
    let mut n: i32 = 1;
    while n.pow(2) < *index {
        n += 2;
    }

    let delta = index - (n - 2).pow(2);

    let layer = n / 2 + 1;
    let side = layer * 2 - 1;
    let len = side - 1;
    let mid = layer - 1;

    let xo = match delta {
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

    let yo = match delta {
        // right
        x if x <= len * 1 => delta - mid,

        // top
        x if x <= len * 2 => mid,

        // left,
        x if x <= len * 3 => mid - (delta - len * 2),

        // bottom
        x if x <= len * 4 => -mid,

        // unknown
        _ => 0,
    };

    return (xo, yo);
}

pub fn spiral_cell(spiral: &HashMap<(i32, i32), i32>, pos: &(i32, i32)) -> i32 {
    let v = match spiral.get(&pos) {
        Some(v) => *v,
        None => 0,
    };
    
    return v;
}

pub fn sum_perimeter(spiral: &HashMap<(i32, i32), i32>, index: &i32) -> i32 {
    let mut tot = 0;
    let c = get_pos(&index);
    
    let dirs = [
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1)
    ];
    
    for dir in dirs.iter() {
        let pos = (c.0 + dir.0, c.1 + dir.1);
        let sum = spiral_cell(&spiral, &pos);
        
        tot += sum;
    }
    
    return tot;
}
