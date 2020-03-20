use intcode_computer::Opcode;

use super::super::Move;

#[derive(Debug, Default)]
pub struct AsciiMovementFunction {
    pub moves: Vec<Move>,
}

impl AsciiMovementFunction {
    pub fn to_opcode_string(&self) -> Vec<Opcode> {
        let mut opcodes = Vec::new();
        let mut inc_last = false;
        const LEFT: Opcode = 0x4c;
        const RIGHT: Opcode = 0x52;
        const COMMA: Opcode = 0x2c;
        const ZERO: Opcode = 0x30;

        for m in self.moves.iter() {
            match &m {
                Move::TurnLeft => opcodes.push(LEFT),
                Move::TurnRight => opcodes.push(RIGHT),
                Move::TurnAround => opcodes.extend([LEFT, COMMA, LEFT].iter()),
                Move::Forward => {
                    if inc_last {
                        // index of the last real value (past the comma)
                        let idx = opcodes.len() - 2;
                        opcodes[idx] += 1;
                        inc_last = opcodes[idx] < ZERO + 9;
                    } else {
                        opcodes.push(ZERO + 1);
                        opcodes.push(COMMA);
                        inc_last = true;
                    }
                    // skip pushing the comma and resetting inc_last
                    continue;
                }
            }

            // This is only executed for turning moves
            opcodes.push(COMMA);
            inc_last = false;
        }

        // Remove that last comma
        opcodes.pop();

        opcodes
    }
}

impl From<&'static str> for AsciiMovementFunction {
    fn from(s: &'static str) -> Self {
        let mut moves = Vec::new();
        for c in s.split(",") {
            match c {
                "L" => moves.push(Move::TurnLeft),
                "R" => moves.push(Move::TurnRight),
                x => {
                    let x: usize = x.parse().unwrap();
                    for _ in 0..x {
                        moves.push(Move::Forward);
                    }
                }
            }
        }

        Self { moves }
    }
}

mod tests {
    #[test]
    fn test_opcode_str_creation() {
        use super::*;
        use crate::types::Move::*;

        let movement_function = AsciiMovementFunction {
            moves: vec![
                TurnLeft, TurnRight, TurnLeft, Forward, Forward, Forward, TurnAround, Forward,
                TurnRight, Forward,
            ],
        };

        const L: Opcode = 'L' as u8 as Opcode;
        const R: Opcode = 'R' as u8 as Opcode;
        const COMMA: Opcode = 0x2c;
        const ZERO: Opcode = 0x30;

        let opcode_str = movement_function.to_opcode_string();
        assert_eq!(
            opcode_str,
            vec![
                L,
                COMMA,
                R,
                COMMA,
                L,
                COMMA,
                ZERO + 3,
                COMMA,
                L,
                COMMA,
                L,
                COMMA,
                ZERO + 1,
                COMMA,
                R,
                COMMA,
                ZERO + 1
            ]
        )
    }
}
