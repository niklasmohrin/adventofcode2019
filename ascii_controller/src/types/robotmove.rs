use super::Direction;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Move {
    TurnLeft,
    TurnRight,
    TurnAround,
    Forward,
}

impl Move {
    pub fn difference(a: Direction, b: Direction) -> Move {
        use Direction::*;
        let a = match a {
            Up => 0,
            Right => 1,
            Down => 2,
            Left => 3,
        };
        let b = match b {
            Up => 0,
            Right => 1,
            Down => 2,
            Left => 3,
        };

        let mut difference = (a - b) % 4;
        if difference < 0 {
            difference += 4;
        }

        match difference {
            0 => Move::Forward,
            1 => Move::TurnLeft,
            2 => Move::TurnAround,
            3 => Move::TurnRight,
            _ => unreachable!(),
        }
    }
}
