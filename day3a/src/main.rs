fn main() {
    // let input = 16;
    let input = 289326;

    let mut n: i32 = 1;
    while n.pow(2) < input {
        n += 2;
    }

    let delta = input - (n - 2).pow(2);

    let layer = n / 2 + 1;
    let side = layer * 2 - 1;
    let len = side - 1;
    let mid = layer - 1;

    println!("input: {}", input);
    println!("layer: {}", layer);
    println!("side: {}", side);
    println!("delta: {}", delta);
    println!("len: {}", len);
    println!("mid: {}", mid);

    let xo = match delta {
        // right
        x if x <= len * 1 => mid,

        // top
        x if x <= len * 2 => (mid - (delta - len)),

        // left
        x if x <= len * 3 => -mid,

        // bottom
        x if x <= len * 4 => (delta - len * 3) - mid,

        // unknown
        _ => 0,
    };

    let yo = match delta {
        // right
        x if x <= len * 1 => mid - delta,

        // top
        x if x <= len * 2 => mid,

        // left,
        x if x <= len * 3 => (delta - len * 2) - mid,

        // bottom
        x if x <= len * 4 => -mid,

        // unknown
        _ => 0,
    };

    println!("x offset: {}", xo);
    println!("y offset: {}", yo);

    let answer = xo.abs() + yo.abs();
    println!("Answer: {}", answer);
}
