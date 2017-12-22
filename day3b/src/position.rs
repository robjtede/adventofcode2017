use std::ops::Add;

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Position {
        return Position { x: x, y: y };
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, other: Position) -> Position {
        return Position {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl<'a, 'b> Add<&'b Position> for &'a Position {
    type Output = Position;

    fn add(self, other: &'b Position) -> Position {
        Position {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<'a> Add<&'a Position> for Position {
    type Output = Position;

    fn add(self, other: &'a Position) -> Position {
        return Position {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl<'a> Add<Position> for &'a Position {
    type Output = Position;

    fn add(self, other: Position) -> Position {
        return Position {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}
