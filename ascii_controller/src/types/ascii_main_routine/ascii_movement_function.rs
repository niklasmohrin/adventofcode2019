use intcode_computer::Opcode;

use super::super::Move;

#[derive(Debug, Default)]
pub struct AsciiMovementFunction {
    pub moves: Vec<Move>,
}

impl AsciiMovementFunction {
    pub fn to_opcode_string(&self) -> Vec<Opcode> {
        let mut opcodes = Vec::new();
        for m in self.moves.iter() {
            match &m {
                Move::TurnLeft => opcodes.push(76),                  // 'L'
                Move::TurnRight => opcodes.push(82),                 // 'R'
                Move::TurnAround => opcodes.extend([76, 76].iter()), // 'L', 'L'
                Move::Forward => {
                    match opcodes.last_mut() {
                        Some(v) if *v >= 0x30 && *v < 0x39 => *v += 1,
                        _ => opcodes.push(0x31),
                    };
                }
            }
        }

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
