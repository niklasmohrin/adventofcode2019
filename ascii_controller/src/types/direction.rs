use core::slice::Iter;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn turn(&self, dir: Direction) -> Direction {
        use Direction::*;
        match dir {
            Left => match self {
                Up => Left,
                Left => Down,
                Down => Right,
                Right => Up,
            },
            Right => match self {
                Up => Right,
                Right => Down,
                Down => Left,
                Left => Up,
            },
            _ => panic!("Invalid turn of direction"),
        }
    }

    pub fn iter() -> Iter<'static, Direction> {
        use Direction::*;
        static DIRECTIONS: [Direction; 4] = [Up, Right, Down, Left];
        DIRECTIONS.iter()
    }
}
