use intcode_channel_io::IntcodeThread;
use intcode_computer::{Opcode, ProgramMemory};
use std::collections::HashSet;
use std::convert::TryInto;

mod types;
use types::{AsciiMainRoutine, Coordinate, Direction, Field, Move};

#[derive(Default)]
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
        let mut cur = self.find_robot().expect("Cannot determine needed moves, because the robot (the starting position and orientation) could not be found. Is the map already revealed?");
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
        let routine = AsciiMainRoutine::construct_from_moves(moves, 20)
            .expect("There is no possible solution to the given scaffolding map.");

        const NEWLINE: Opcode = 0x0a;
        // Submit the main movement routine
        {
            let opcodes = routine.to_opcode_string();
            // Check length at most 20 (without newline) and newline at the end
            assert!(opcodes.len() > 0);
            assert!(opcodes.len() <= 21);
            assert!(*opcodes.last().unwrap() == NEWLINE);
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
            assert!(*opcodes.last().unwrap() == NEWLINE);
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

#[cfg(test)]
pub mod tests {
    use super::*;

    pub const MAP: &'static str = "#######...#####
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

    pub fn get_map() -> Vec<Vec<Field>> {
        MAP.lines()
            .map(|line| line.chars().map(|c| Field::from(c)).collect())
            .collect()
    }

    #[test]
    fn test_get_moves() {
        use crate::types::Move::*;
        let map = get_map();
        let instance = AsciiController {
            map,
            ..Default::default()
        };
        let moves = instance.moves_needed();
        let expected_first_moves = vec![
            TurnRight, Forward, Forward, Forward, Forward, Forward, Forward, Forward, Forward,
            TurnRight, Forward, Forward, Forward, Forward, Forward, Forward, Forward, Forward,
            TurnRight, Forward, Forward, Forward, Forward, TurnRight, Forward, Forward, Forward,
            Forward, TurnRight, Forward, Forward, Forward, Forward, Forward, Forward, Forward,
            Forward, TurnLeft,
        ];

        let len = expected_first_moves.len();
        assert_eq!(moves[..len], expected_first_moves[..]);
    }

    #[test]
    fn test_find_robot() {
        let map = get_map();
        let instance = AsciiController {
            map,
            ..Default::default()
        };
        let robot = instance
            .find_robot()
            .expect("Robot was not found, although it is on the map.");
        assert_eq!(robot, Coordinate(0, 6));
    }
}
