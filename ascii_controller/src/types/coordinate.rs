use std::convert::TryInto;
use std::num::TryFromIntError;
use std::ops::{Add, AddAssign};

// use crate::types::Direction::Direction;
use super::Direction;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Coordinate(pub usize, pub usize);

impl Coordinate {
    pub fn addition(self, dir: Direction) -> Result<Coordinate, TryFromIntError> {
        use Direction::*;

        let Coordinate(x, y) = self;
        let dx: i8 = match dir {
            Left => -1,
            Right => 1,
            _ => 0,
        };
        let dy: i8 = match dir {
            Up => -1,
            Down => 1,
            _ => 0,
        };

        let x = (x as isize + dx as isize).try_into()?;
        let y = (y as isize + dy as isize).try_into()?;

        Ok(Coordinate(x, y))
    }
}

impl Add<Direction> for Coordinate {
    type Output = Coordinate;

    fn add(self, dir: Direction) -> Coordinate {
        match self.addition(dir) {
            Ok(c) => c,
            Err(e) => panic!("Invalid addition: {:?} + Direction::{:?} does not yield a valid Coordinate: {}.\nTo handle errors yourself, use Coordinate::addition instead of addition operator.", self, dir,e),
        }
    }
}

impl AddAssign<Direction> for Coordinate {
    fn add_assign(&mut self, dir: Direction) {
        *self = *self + dir;
    }
}
