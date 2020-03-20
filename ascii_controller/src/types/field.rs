use intcode_computer::Opcode;
use std::convert::TryInto;

use super::Direction;

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
        Field::from(ascii)
    }
}

impl From<char> for Field {
    fn from(c: char) -> Field {
        match c {
            '#' => Field::Scaffold,
            '.' => Field::Space,
            '^' => Field::Robot(Direction::Up),
            'v' => Field::Robot(Direction::Down),
            '<' => Field::Robot(Direction::Left),
            '>' => Field::Robot(Direction::Right),
            _ => panic!(
                "Invalid char to Field conversion: {} does not map to a Field type.",
                c
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
