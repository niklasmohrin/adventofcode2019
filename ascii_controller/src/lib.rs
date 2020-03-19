use intcode_channel_io::IntcodeThread;
use intcode_computer::{Opcode, ProgramMemory};
use std::collections::HashSet;
use std::convert::TryInto;

mod types;
use types::{Coordinate, Direction, Field, Move};

#[derive(Debug)]
struct AsciiMovementFunction {
    moves: Vec<Move>,
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

struct AsciiMainRoutine {
    routine: Vec<u8>,
    movement_functions: [AsciiMovementFunction; 3],
}

impl AsciiMainRoutine {
    pub fn construct_from_moves(
        moves: Vec<Move>,
        max_opcodes: usize,
    ) -> Result<Self, &'static str> {
        // Since we need to cover the first n moves, there has to be a movement function that matches these n moves
        for a_len in 0..moves.len() {
            println!("Trying a_len {}", a_len);
            let a_func = AsciiMovementFunction {
                moves: moves[..a_len].to_vec(),
            };
            if a_func.to_opcode_string().len() > max_opcodes {
                // If we did not find something up to this point, we will not find anything else,
                // since the opcode string can only get longer.
                break;
            }
            // Similarly, there has to be a movement function for the last n moves
            for b_len in 0..(moves.len() - a_len) {
                println!("\tTrying b_len {}", b_len);
                let b_func = AsciiMovementFunction {
                    moves: moves[moves.len() - b_len..].to_vec(),
                };
                if b_func.to_opcode_string().len() > max_opcodes {
                    // With this length of a, there does not seem to be a solution,
                    // that fits the requirements. The opcode string can only get longer,
                    // so it is okay to skip the other lengths.
                    break;
                }
                // Finally, we need to try to come as far as possible from the start with a and b.
                // If we cannot go any further, we know the first moves of the third movement function.
                // Similarly, we can do the same thing from the back.
                const A_INDEX: u8 = 0;
                const B_INDEX: u8 = 1;
                const C_INDEX: u8 = 2;

                let mut main: Vec<u8> = vec![A_INDEX];
                let mut covered_moves = a_len;
                let needed_moves = moves.len();
                let mut just_popped: Option<i8> = None;

                let mut c_len = 0;
                let mut c_func = AsciiMovementFunction { moves: Vec::new() };
                let mut c_funcs_on_stack = 0;

                while !main.is_empty() {
                    if b_len == 9 {
                        println!("{:?}", main);
                    }
                    if covered_moves == needed_moves && (main.len() * 2 - 1) <= max_opcodes {
                        // The number of covered moves is correct and the length of the opcode string of
                        // the main function (one character per function, comma seperated, so two chars per function minus
                        // the missing last comma) is not too long.
                        let routine = AsciiMainRoutine {
                            routine: main,
                            movement_functions: [a_func, b_func, c_func],
                        };
                        return Ok(routine);
                    } else {
                        // TODO: make this into some smart thing, so that it only compares the vectors if it really needs to know
                        let a_matches = a_len > 0
                            && a_len + covered_moves <= needed_moves
                            && moves[covered_moves..covered_moves + a_len] == a_func.moves[..];
                        let b_matches = b_len > 0
                            && b_len + covered_moves <= needed_moves
                            && moves[covered_moves..covered_moves + b_len] == b_func.moves[..];
                        let c_matches = c_len > 0
                            && c_len + covered_moves <= needed_moves
                            && moves[covered_moves..covered_moves + c_len] == c_func.moves[..];
                        if a_matches || b_matches || c_matches {
                            let popped = just_popped.take().unwrap_or(-1);
                            // implies that covered_moves < needed_moves
                            // We can still keep pushing with the known functions a and b.
                            if a_matches && popped < A_INDEX as i8 {
                                main.push(A_INDEX);
                                covered_moves += a_len;
                            } else if b_matches && popped < B_INDEX as i8 {
                                main.push(B_INDEX);
                                covered_moves += b_len;
                            } else if popped < C_INDEX as i8 {
                                // Therefore c matches.
                                main.push(C_INDEX);
                                covered_moves += c_len;
                            } else {
                                // just_popped = Some(popped);
                                // main.pop
                            }
                        } else {
                            // The last function we pushed was wrong.
                            let wrong_func = main.pop().unwrap();
                            just_popped = Some(wrong_func as i8);
                            match wrong_func {
                                A_INDEX => {
                                    covered_moves -= a_len;
                                    // Function a was wrong, try movement function b instead (if it matches).
                                    // Otherwise, do not push anything and fix the function before this one too.
                                    if b_matches {
                                        main.push(B_INDEX);
                                        covered_moves += b_len;
                                    }
                                }
                                B_INDEX => {
                                    covered_moves -= b_len;
                                    // Movement function b did not work either, so lets try to use or construct function c from here.
                                    if c_len > 0 {
                                        // We already created a function c earlier, just use it.
                                        if c_matches {
                                            main.push(C_INDEX);
                                            covered_moves += c_len;
                                            c_funcs_on_stack += 1;
                                        }
                                    } else {
                                        // We need to come up with a function c, since we cannot keep going with just a and b.
                                        for new_c_len in 1..needed_moves - covered_moves {
                                            c_func = AsciiMovementFunction {
                                                moves: moves
                                                    [covered_moves..covered_moves + new_c_len]
                                                    .to_vec(),
                                            };

                                            if c_func.to_opcode_string().len() > max_opcodes {
                                                // We cannot construct a function c.
                                                break;
                                            }

                                            let c_matches = moves
                                                [covered_moves..covered_moves + new_c_len]
                                                == c_func.moves[..];
                                            if !c_matches {
                                                // We cannot construct a function c.
                                                break;
                                            }

                                            c_len = new_c_len;
                                            c_funcs_on_stack = 1;
                                            main.push(C_INDEX);
                                            covered_moves += c_len;
                                            break;
                                        }
                                    }
                                }
                                C_INDEX => {
                                    covered_moves -= c_len;
                                    c_funcs_on_stack -= 1;

                                    if c_funcs_on_stack == 0 {
                                        // This version of a function for c did not work out, they all got popped again.
                                        // We will try another version of function c, including one more move than the last one.
                                        c_len += 1;
                                        if covered_moves + c_len > needed_moves {
                                            c_len = 0;
                                            continue;
                                        }
                                        c_func = AsciiMovementFunction {
                                            moves: moves[covered_moves..covered_moves + c_len]
                                                .to_vec(),
                                        };

                                        let c_matches = moves[covered_moves..covered_moves + c_len]
                                            == c_func.moves[..];

                                        if c_matches
                                            && c_func.to_opcode_string().len() <= max_opcodes
                                        {
                                            // We will also push it and see how it goes from here.
                                            main.push(C_INDEX);
                                            covered_moves += c_len;
                                            c_funcs_on_stack += 1;
                                        } else {
                                            // Otherwise: We cannot build a function c to fit into the other spots from here,
                                            // the other functions below wrong.
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
            }
        }

        // We tried every valid combination of functions a and b, but did not find anything.
        Err("Could not find valid distribution of moves.")
    }

    pub fn to_opcode_string(&self) -> Vec<Opcode> {
        const FUNCTION_LETTERS: [char; 3] = ['A', 'B', 'C'];
        self.routine
            .iter()
            .map(|&idx| FUNCTION_LETTERS[idx as usize].to_string())
            .collect::<Vec<String>>()
            .join(",")
            .bytes()
            .map(|c| Opcode::from(c))
            .collect()
    }
}

pub struct AsciiController {
    map: Vec<Vec<Field>>,
    thread: Option<IntcodeThread>,
    program: ProgramMemory,
}

impl AsciiController {
    pub fn new(program: ProgramMemory) -> Self {
        let map = Vec::new();

        AsciiController {
            map,
            thread: None,
            program,
        }
    }

    fn build_map(&mut self) {
        let thread = self
            .thread
            .as_ref()
            .expect("Could not build map: No intcode program thread found / self.thread == None");

        // Insert the first line manually, since the protocol does not start with a newline
        self.map.push(Vec::new());

        // Receive data from the camera until the program terminates.
        while let Some(opcode) = thread.recv() {
            if opcode == 0xa {
                // Start on a new line (0xa == 10 == '\n' == new line)
                self.map.push(Vec::new());
            } else {
                let field = opcode
                    .try_into()
                    .expect("Ascii sent some non-ascii stuff...");
                self.map.last_mut().unwrap().push(field);
            }
        }

        // Remove empty trailing lines (remove all trailing ones, although it should be one max, right?)
        while self.map.last().unwrap().len() == 0 {
            self.map.pop();
        }

        self.thread.take().unwrap().join();
    }

    fn neighbours(&self, pos: Coordinate) -> Vec<Coordinate> {
        let mut neighbours = Vec::new();

        for &dir in Direction::iter() {
            // Addition errors, if either axis would be negative, since it is an unsigned type.
            // If this happens, there surely is no neighbour there.
            let Coordinate(x, y) = match pos.addition(dir) {
                Ok(c) => c,
                Err(_) => continue,
            };

            // Furthermore, if we go out of the map on the opposite side, we can continue too.
            if y >= self.map.len() || x >= self.map[y].len() {
                continue;
            }

            // Finally, anthing but an empty space (that is Robot or Scaffolding) is considered a neighbour.
            if self.map[y][x] != Field::Space {
                neighbours.push(Coordinate(x, y));
            }
        }

        neighbours
    }

    fn neighbour_count(&self, pos: Coordinate) -> u8 {
        self.neighbours(pos)
            .len()
            .try_into()
            .expect("Somehow the position {}, {} has more than 255 neighbours ...?")
    }

    fn intersections(&self) -> Vec<Coordinate> {
        let mut intersections = Vec::new();
        for y in 0..self.map.len() {
            for x in 0..self.map[y].len() {
                if self.map[y][x] != Field::Space {
                    if self.neighbour_count(Coordinate(x, y)) > 2 {
                        intersections.push(Coordinate(x, y));
                    }
                }
            }
        }
        intersections
    }

    pub fn print_map(&self) {
        for row in self.map.iter() {
            for &field in row.iter() {
                let c: char = field.into();
                print!("{}", c);
            }
            println!();
        }
    }

    /// Efectively the solution to part one of day 17, but also needed for part two.
    pub fn run_cameras(&mut self) {
        let identifier = String::from("ASCII Camera");
        let mut thread = IntcodeThread::new(self.program.clone(), Some(identifier));
        thread.hide_debug_messages = true;

        self.thread = Some(thread);
        self.build_map();
        self.print_map();

        println!(
            "Sum of alignment parameters: {}",
            self.intersections()
                .iter()
                .map(|&Coordinate(x, y)| x * y)
                .sum::<usize>()
        );
    }

    fn find_robot(&self) -> Option<Coordinate> {
        for y in 0..self.map.len() {
            for x in 0..self.map[y].len() {
                if let Field::Robot(_) = self.map[y][x] {
                    return Some(Coordinate(x, y));
                }
            }
        }

        None
    }

    pub fn scaffolding_coords(&self) -> HashSet<Coordinate> {
        let mut s = HashSet::new();
        for y in 0..self.map.len() {
            for x in 0..self.map[y].len() {
                if self.map[y][x] != Field::Space {
                    s.insert(Coordinate(x, y));
                }
            }
        }
        s
    }

    fn pick_next(
        &self,
        cur: &Coordinate,
        facing: Direction,
        unvisited: &HashSet<Coordinate>,
    ) -> Option<(Coordinate, Direction)> {
        let mut possible = Vec::new();

        if let Ok(pos) = cur.addition(facing) {
            possible.push((pos, facing));
        }
        let dir = facing.turn(Direction::Left);
        if let Ok(pos) = cur.addition(dir) {
            possible.push((pos, dir));
        }
        let dir = facing.turn(Direction::Right);
        if let Ok(pos) = cur.addition(dir) {
            possible.push((pos, dir));
        }
        drop(dir);
        for &(pos, dir) in possible.iter() {
            if unvisited.contains(&pos) {
                return Some((pos, dir));
            }
        }
        for &(pos, dir) in possible.iter() {
            let Coordinate(x, y) = pos;
            if y < self.map.len() && x < self.map[y].len() && self.map[y][x] != Field::Space {
                return Some((pos, dir));
            }
        }

        None
    }

    pub fn moves_needed(&self) -> Vec<Move> {
        let mut cur = self.find_robot().unwrap();
        let Coordinate(x, y) = cur;

        let mut facing = match self.map[y][x] {
            Field::Robot(dir) => dir,
            _ => unreachable!(),
        };

        let mut unvisited = self.scaffolding_coords();

        let mut moves = Vec::new();
        while !unvisited.is_empty() {
            unvisited.remove(&cur);

            let (next, dir) = match self.pick_next(&cur, facing, &unvisited) {
                Some(x) => x,
                None => break,
            };

            if dir != facing {
                let turn = Move::difference(facing, dir);
                if !unvisited.contains(&next) && turn == Move::TurnAround {
                    break;
                }
                moves.push(turn);
                facing = dir;
            }

            moves.push(Move::Forward);
            cur += dir;
        }

        moves
    }

    pub fn walk_scaffolding(&mut self) -> Opcode {
        // Since self.build_map resets the self.thread field and we do want to overwrite
        // some other thread handle, we just assert.
        assert!(self.thread.is_none());

        // Spawn the intcode program thread
        // Changing address 0 from 1 to 2 starts the robot program instead of the camera program
        let mut program = self.program.clone();
        program[0] = 2;
        let identifier = String::from("ASCII Robot");
        let thread = IntcodeThread::new(program, Some(identifier));

        // Find subprograms A,B,C
        let moves = self.moves_needed();
        // let routine = self.find_walking_routine(moves);
        let routine = AsciiMainRoutine::construct_from_moves(moves, 21)
            .expect("There is no possible solution to the given scaffolding map.");

        // Submit the main movement routine
        {
            let opcodes = routine.to_opcode_string();
            // Check length at most 20 (without newline) and newline at the end
            assert!(opcodes.len() > 0);
            assert!(opcodes.len() <= 21);
            assert!(*opcodes.last().unwrap() == 0xa);
            for &opcode in opcodes.iter() {
                thread.send(opcode);
            }
        }

        // Submit the three movement functions
        for movement_function in routine.movement_functions.iter() {
            let opcodes = movement_function.to_opcode_string();
            // Check length at most 20 (without newline) and newline at the end
            assert!(opcodes.len() > 0);
            assert!(opcodes.len() <= 21);
            assert!(*opcodes.last().unwrap() == 0xa);
            for &opcode in opcodes.iter() {
                thread.send(opcode);
            }
        }

        let collected_dust = thread
            .recv()
            .expect("Ascii vacuum robot did not send the amount of collected dust.");

        collected_dust
    }
}
