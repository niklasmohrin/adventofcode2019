use intcode_computer::Opcode;

use super::Move;

mod ascii_movement_function;
pub use ascii_movement_function::AsciiMovementFunction;

#[derive(Default)]
pub struct AsciiMainRoutine {
    pub routine: Vec<u8>,
    pub movement_functions: [AsciiMovementFunction; 3],
    pub max_opcodes: usize,
    pub needed_moves: Vec<Move>,
}

impl AsciiMainRoutine {
    fn matches(&self, f: &AsciiMovementFunction, offset: usize) -> bool {
        let f_len = f.moves.len();
        let too_long =
            Self::chars_needed_for_routine_with_len(self.routine.len() + 1) > self.max_opcodes;

        !too_long
            && f_len > 0
            && offset + f_len <= self.needed_moves.len()
            && self.needed_moves[offset..offset + f_len] == f.moves[..]
    }

    fn chars_needed_for_routine_with_len(len: usize) -> usize {
        if len == 0 {
            0
        } else {
            len * 2 - 1
        }
    }

    fn routine_too_long(&self) -> bool {
        Self::chars_needed_for_routine_with_len(self.routine.len()) > self.max_opcodes
    }

    /// If possible, contructs a movement function of length c_len, that matches the next c_len moves
    /// at offset and complies with self.max_opcodes, and writes it into c_fund.
    fn next_c_func(
        &self,
        c_func: &mut AsciiMovementFunction,
        c_len: usize,
        offset: usize,
    ) -> Result<(), ()> {
        c_func.moves = self.needed_moves[offset..offset + c_len].to_vec();

        let c_matches = self.matches(&c_func, offset);
        if !c_matches || c_func.to_opcode_string().len() > self.max_opcodes {
            c_func.moves.clear();
            return Err(());
        }

        Ok(())
    }

    pub fn construct_from_moves(
        moves: Vec<Move>,
        max_opcodes: usize,
    ) -> Result<Self, &'static str> {
        let mut main = AsciiMainRoutine {
            max_opcodes,
            needed_moves: moves,
            ..Default::default()
        };

        let mut a_func: AsciiMovementFunction = Default::default();
        let mut b_func: AsciiMovementFunction = Default::default();
        let moves_needed = main.needed_moves.len();

        // Since we need to cover the first n moves, there has to be a movement function that matches these n moves
        for a_len in 1..moves_needed {
            println!("Trying a_len {}", a_len);
            a_func.moves = main.needed_moves[..a_len].to_vec();
            if a_func.to_opcode_string().len() > max_opcodes {
                // If we did not find something up to this point, we will not find anything else,
                // since the opcode string can only get longer.
                println!(
                    "Len {} is too long with {:?}",
                    a_len,
                    a_func.to_opcode_string()
                );
                break;
            }
            // Similarly, there has to be a movement function for the last n moves
            for b_len in 0..(moves_needed - a_len) {
                // println!("\tTrying b_len {}", b_len);
                b_func.moves = main.needed_moves[moves_needed - b_len..].to_vec();
                if b_func.to_opcode_string().len() > max_opcodes {
                    // With this length of a, there does not seem to be a solution,
                    // that fits the requirements. The opcode string can only get longer,
                    // so it is okay to skip the other lengths.
                    break;
                }

                if let Ok(c_func) = main.find_c_function(&a_func, &b_func) {
                    main.movement_functions = [a_func, b_func, c_func];
                    return Ok(main);
                }
            }
        }

        // We tried every valid combination of functions a and b, but did not find anything.
        Err("Could not find valid distribution of moves.")
    }

    fn find_c_function(
        &mut self,
        a_func: &AsciiMovementFunction,
        b_func: &AsciiMovementFunction,
    ) -> Result<AsciiMovementFunction, ()> {
        // Finally, we need to try to come as far as possible from the start with a and b.
        // If we cannot go any further, we know the first moves of the third movement function.
        // Similarly, we can do the same thing from the back.
        let moves_needed = self.needed_moves.len();
        let a_len = a_func.moves.len();
        let b_len = b_func.moves.len();

        let mut c_func = Default::default();
        let mut c_len = 0;
        let mut c_funcs_on_stack = 0;

        const A_INDEX: u8 = 0;
        const B_INDEX: u8 = 1;
        const C_INDEX: u8 = 2;

        self.routine = vec![A_INDEX];
        let mut covered_moves = a_len;
        let mut just_popped: Option<i8> = None;

        while !self.routine.is_empty() {
            // if b_len == 9 {
            // println!("{:?}", self.routine);
            // }
            if covered_moves == moves_needed && !self.routine_too_long() {
                // The number of covered moves is correct and the length of the opcode string of
                // the main function (one character per function, comma seperated, so two chars per function minus
                // the missing last comma) is not too long.
                return Ok(c_func);
            } else {
                // TODO: make this into some smart thing, so that it only compares the vectors if it really needs to know
                let a_matches = self.matches(&a_func, covered_moves);
                let b_matches = self.matches(&b_func, covered_moves);
                let c_matches = self.matches(&c_func, covered_moves);
                let popped = just_popped.take().unwrap_or(-1);
                if a_matches && popped < A_INDEX as i8 {
                    self.routine.push(A_INDEX);
                    covered_moves += a_len;
                } else if b_matches && popped < B_INDEX as i8 {
                    self.routine.push(B_INDEX);
                    covered_moves += b_len;
                } else if c_matches && popped < C_INDEX as i8 {
                    self.routine.push(C_INDEX);
                    covered_moves += c_len;
                    c_funcs_on_stack += 1;
                } else {
                    // The last function we pushed was wrong.
                    let wrong_func = self.routine.pop().unwrap();
                    just_popped = Some(wrong_func as i8);

                    match wrong_func {
                        A_INDEX => {
                            covered_moves -= a_len;
                        }
                        B_INDEX => {
                            covered_moves -= b_len;
                        }
                        C_INDEX => {
                            covered_moves -= c_len;
                            c_funcs_on_stack -= 1;

                            if c_funcs_on_stack == 0 {
                                // This version of a function for c did not work out, they all got popped again.
                                // We will try another version of function c, including one more move than the last one.
                                c_len += 1;
                                if covered_moves + c_len > moves_needed {
                                    c_len = 0;
                                    c_func.moves.clear();
                                    continue;
                                }

                                if let Ok(()) = self.next_c_func(&mut c_func, c_len, covered_moves)
                                {
                                    self.routine.push(C_INDEX);
                                    covered_moves += c_len;
                                    c_funcs_on_stack += 1;
                                } else {
                                    // Otherwise: We cannot build a function c to fit into the other spots from here,
                                    // the other functions below wrong.
                                    c_func.moves.clear();
                                    c_len = 0;
                                }
                            }

                            // Otherwise, this can still work with our current version of function c,
                            // just keep trying to swap the other functions.
                        }
                        x => panic!("Somehow there is this other thing here: {}", x),
                    }
                }
            }
        }

        // Main stack is empty, so we did not find anything.
        // Continue with next possible function b.
        self.routine.clear();
        Err(())
    }

    pub fn to_opcode_string(&self) -> Vec<Opcode> {
        const FUNCTION_LETTERS: [&'static str; 3] = ["A", "B", "C"];
        self.routine
            .iter()
            .map(|&idx| FUNCTION_LETTERS[idx as usize])
            .collect::<Vec<&'static str>>()
            .join(",")
            .bytes()
            .map(|c| Opcode::from(c))
            .collect()
    }
}

mod tests {
    #[test]
    fn test_find_c() {
        use super::*;
        use crate::types::Field;
        use crate::AsciiController;

        let map = "#######...#####
#.....#...#...#
#.....#...#...#
......#...#...#
......#...###.#
......#.....#.#
^########...#.#
......#.#...#.#
......#########
........#...#..
....#########..
....#...#......
....#...#......
....#...#......
....#####......";
        let map = map
            .lines()
            .map(|line| line.chars().map(|c| Field::from(c)).collect())
            .collect();
        let controller = AsciiController {
            map,
            ..Default::default()
        };
        let moves = controller.moves_needed();
        let a_func = AsciiMovementFunction::from("R,8,R,8");
        let b_func = AsciiMovementFunction::from("R,4,R,4,R,8");

        let mut main = AsciiMainRoutine {
            max_opcodes: 20,
            needed_moves: moves,
            ..Default::default()
        };

        main.find_c_function(&a_func, &b_func)
            .expect("No function c found, although it exists");
    }

    #[test]
    fn test_opcode_string_creation() {
        use super::*;
        let main = AsciiMainRoutine {
            routine: vec![0, 1, 2, 1, 2, 0],
            ..Default::default()
        };

        const A: Opcode = 0x41;
        const B: Opcode = 0x42;
        const C: Opcode = 0x43;
        const COMMA: Opcode = 0x2c;

        let opcode_str = main.to_opcode_string();
        assert_eq!(
            opcode_str,
            vec![A, COMMA, B, COMMA, C, COMMA, B, COMMA, C, COMMA, A]
        );
    }
}
