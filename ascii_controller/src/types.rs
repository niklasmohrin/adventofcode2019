use core::slice::Iter;
use intcode_computer::Opcode;
use std::convert::TryInto;
use std::num::TryFromIntError;
use std::ops::{Add, AddAssign};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Coordinate(pub usize, pub usize);

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

#[derive(Clone, Copy, PartialEq)]
pub enum Field {
    Scaffold,
    Space,
    Robot(Direction),
}

impl From<Opcode> for Field {
    fn from(opcode: Opcode) -> Field {
        let byte: u8 = opcode
            .try_into()
            .expect("Invalid Opcode to Field conversion: Opcode is non-ascii.");
        let ascii: char = byte.into();
        match ascii {
            '#' => Field::Scaffold,
            '.' => Field::Space,
            '^' => Field::Robot(Direction::Up),
            'v' => Field::Robot(Direction::Down),
            '<' => Field::Robot(Direction::Left),
            '>' => Field::Robot(Direction::Right),
            _ => panic!(
                "Invalid Opcode to Field conversion: {} ({}) does not map to a Field type.",
                ascii, byte
            ),
        }
    }
}

impl From<Field> for char {
    fn from(field: Field) -> char {
        match field {
            Field::Scaffold => '#',
            Field::Space => '.',
            Field::Robot(Direction::Up) => '^',
            Field::Robot(Direction::Down) => 'v',
            Field::Robot(Direction::Left) => '<',
            Field::Robot(Direction::Right) => '>',
        }
    }
}

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
