use intcode_computer::Opcode;
use std::ops::Add;

pub type Axis = i32;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Coordinate(pub Axis, pub Axis);

#[derive(Clone, Copy)]
pub enum Direction {
    North,
    West,
    South,
    East,
}

impl Direction {
    pub fn inverse(&self) -> Self {
        use Direction::*;
        match self {
            North => South,
            South => North,
            West => East,
            East => West,
        }
    }
}

impl Add<Direction> for Coordinate {
    type Output = Self;

    fn add(self, dir: Direction) -> Self {
        let Coordinate(x, y) = self;
        use Direction::*;
        let dx = match dir {
            East => -1,
            West => 1,
            _ => 0,
        };
        let dy = match dir {
            North => -1,
            South => 1,
            _ => 0,
        };
        Coordinate(x + dx, y + dy)
    }
}

impl From<Direction> for Opcode {
    fn from(dir: Direction) -> Opcode {
        use Direction::*;
        match dir {
            North => 1,
            South => 2,
            West => 3,
            East => 4,
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum BlockType {
    Unknown,
    Walkable,
    OxygenSystem,
    Wall,
}

impl From<Opcode> for BlockType {
    fn from(opcode: Opcode) -> BlockType {
        use BlockType::*;
        match opcode {
            0 => Wall,
            1 => Walkable,
            2 => OxygenSystem,
            x => panic!("Invalid conversion from Opcode to Blocktype: {}", x),
        }
    }
}

impl From<BlockType> for char {
    fn from(block: BlockType) -> char {
        use BlockType::*;
        match block {
            Unknown => ' ',
            Walkable => '.',
            OxygenSystem => 'X',
            Wall => '#',
        }
    }
}
