//! Interface for the Arcade Cabinet Game from day 13 of [Advent of Code 2019](adventofcode.com)

use intcode_computer::{run_program, IntcodeIo, Opcode, ProgramMemory};
use std::cell::RefCell;
use std::collections::HashMap;
use std::convert::TryInto;
use std::io;

const EMPTY_TILE_CHAR: char = ' ';
const WALL_TILE_CHAR: char = '#';
const BLOCK_TILE_CHAR: char = '.';
const HORIZONTALPADDLE_TILE_CHAR: char = '-';
const BALL_TILE_CHAR: char = 'o';

/// Possible tiles on the game field.
#[derive(Clone, Copy, PartialEq)]
pub enum Tile {
    Empty,
    Wall,
    Block,
    HorizontalPaddle,
    Ball,
}

/// Mapping of program output to corresponding tile.
impl From<Opcode> for Tile {
    fn from(opcode: Opcode) -> Tile {
        use crate::Tile::*;
        match opcode {
            0 => Empty,
            1 => Wall,
            2 => Block,
            3 => HorizontalPaddle,
            4 => Ball,
            _ => panic!(
                "Invalid conversion: Opcode {} cannot be converted to Tile",
                opcode
            ),
        }
    }
}

/// Mapping of tile to printed character.
impl From<Tile> for char {
    fn from(tile: Tile) -> char {
        use crate::Tile::*;
        match tile {
            Empty => EMPTY_TILE_CHAR,
            Wall => WALL_TILE_CHAR,
            Block => BLOCK_TILE_CHAR,
            HorizontalPaddle => HORIZONTALPADDLE_TILE_CHAR,
            Ball => BALL_TILE_CHAR,
        }
    }
}

/// Range of a single axis of the drawing board.
type Axis = i32;

/// A coordinate on the drawing board.
#[derive(PartialEq, Eq, Hash, Clone)]
struct Coordinate(Axis, Axis);

/// Buffers three outputs from the intcode program.
struct ArcadeCabinetOutputBuffer {
    buffer: [Option<Opcode>; 3],
    buffered_opcodes: u8,
}

/// Maps from intcode computer Opcode IO to a screen state and handles stdin and stdout for the player.
struct ArcadeCabinetIo {
    pub screen: RefCell<HashMap<Coordinate, Tile>>, // TODO: This should be a Vec<Vec<Tile>> with background resizing
    pub score: RefCell<Opcode>,
    stdin: io::Stdin,
    buffered_output: RefCell<ArcadeCabinetOutputBuffer>,
    automatic_mode: bool,
}

impl ArcadeCabinetIo {
    pub fn new(automatic_mode: bool) -> Self {
        let stdin = io::stdin();
        let screen = RefCell::new(HashMap::new());
        let score = RefCell::new(0);
        let buffered_output = RefCell::new(ArcadeCabinetOutputBuffer {
            buffer: [None, None, None],
            buffered_opcodes: 0,
        });

        Self {
            stdin,
            screen,
            score,
            buffered_output,
            automatic_mode,
        }
    }

    /// Print the current screen state to stdout.
    pub fn print_screen(&self) {
        let screen = self.screen.borrow_mut();
        let coordinates: Vec<&Coordinate> = screen.keys().collect();
        let xs: Vec<Axis> = coordinates.iter().map(|coord| coord.0).collect();
        let ys: Vec<Axis> = coordinates.iter().map(|coord| coord.1).collect();

        let x_min = 0;
        let x_max = *xs.iter().max().unwrap();
        let y_min = 0;
        let y_max = *ys.iter().max().unwrap();

        for y in y_min..=y_max {
            for x in x_min..=x_max {
                let coord = Coordinate(x, y);
                let c = *screen.get(&coord).unwrap_or(&Tile::Empty);
                let c: char = c.into();
                print!("{}", c);
            }
            println!();
        }
    }

    /// Prints the current score.
    pub fn print_score(&self) {
        println!("Score: {}", self.score.borrow());
    }

    /// Consume all three buffered Opcodes and update state accordingly.
    /// This will either update the screen or the score.
    fn handle_buffer(&self) {
        let mut buffered_output = self.buffered_output.borrow_mut();
        assert_eq!(buffered_output.buffered_opcodes, 3);
        buffered_output.buffered_opcodes = 0;

        // The first and second opcodes are the x and y position on the screen.
        let x: Axis = buffered_output.buffer[0]
            .take()
            .unwrap()
            .try_into()
            .unwrap();
        let y: Axis = buffered_output.buffer[1]
            .take()
            .unwrap()
            .try_into()
            .unwrap();

        if x == -1 && y == 0 {
            // Special output coordinates, signaling the third opcode is the current score.
            let score = buffered_output.buffer[2].take().unwrap();
            *self.score.borrow_mut() = score;
        } else {
            // Otherwise, the third opcode maps to the tile at these coordinates.
            let tile: Tile = buffered_output.buffer[2].take().unwrap().into();
            let coord = Coordinate(x, y);
            self.screen.borrow_mut().insert(coord, tile);
        }
    }
}

impl IntcodeIo for ArcadeCabinetIo {
    fn read(&self) -> Opcode {
        // Read internal index, increase by one and keep working with current one
        *self.total_inputs_read.borrow_mut() += 1;

        // The intcode program wants user input, the user should now get to see the current screen.
        self.print_screen();
        self.print_score();

        if self.automatic_mode {
            let screen = self.screen.borrow();
            let x_ball = (screen
                .iter()
                .find(|&(_coord, tile)| *tile == Tile::Ball)
                .unwrap()
                .0) // Coordinate
                .0; // x value
            let x_player = (screen
                .iter()
                .find(|&(_coord, tile)| *tile == Tile::HorizontalPaddle)
                .unwrap()
                .0) // Coordinate
                .0; // x value
            (x_ball - x_player).signum().into()
        } else {
            let mut val = String::new();
            self.stdin.read_line(&mut val).unwrap();

            // Instead of typing -1, 0 or 1 manually, the keys a, s and d can be used instead.
            match val.chars().next().unwrap() {
                'a' => -1,
                's' => 0,
                'd' => 1,
                // Matches everything else
                unknown_char => panic!("Invalid input: {}", unknown_char),
            }
        }
    }

    /// The intcode program outputs something.
    /// Since the Arcade Cabinet Games always output three values that belong together,
    /// we will buffer these and only do something if we collected three opcodes.
    fn write(&self, value: &Opcode) {
        {
            // This extra scope is needed, so that the mutable reference to the buffer
            // is out of scope again when handling the buffer.
            // If we wouldn't do so, both this method and self.handle_buffer would own a
            // mutable reference to the buffer, which is not allowed.
            let mut buffered_output = self.buffered_output.borrow_mut();
            let index: usize = buffered_output.buffered_opcodes.into();
            buffered_output.buffer[index] = Some(*value);
            buffered_output.buffered_opcodes += 1;
            if buffered_output.buffered_opcodes < 3 {
                // If we do not have three values yet, we should not call self.handle_buffer, or it will panic.
                return;
            }
        }
        self.handle_buffer();
    }
}

/// Wrapper around (/ sole user of) ArcadeCabinetIo and public interface
/// to run an Arcade Cabinet Game as provided on day 13 of [Advent of Code 2019](adventofcode.com).
pub struct ArcadeCabinet {
    inout: ArcadeCabinetIo,
}

impl ArcadeCabinet {
    pub fn new(automatic_mode: bool) -> Self {
        let inout = ArcadeCabinetIo::new(automatic_mode);
        Self { inout }
    }

    /// Run a game with <quarters> many quarters inserted into the machine.
    /// If quarters is 0, the game will not start.
    pub fn run(&self, mut program: ProgramMemory, quarters: Opcode) {
        program[0] = quarters;
        run_program(&mut program, &self.inout);
        self.inout.print_screen();
        self.inout.print_score();
    }

    /// Count the occurences of a specific type of tile on the current screen.
    pub fn count_tile(&self, tile: Tile) -> usize {
        self.inout
            .screen
            .borrow()
            .values()
            .filter(|&&t| t == tile)
            .count()
    }
}
