//! Interface for the Arcade Cabinet Game from day 13 of [Advent of Code 2019](adventofcode.com)

use intcode_computer::{run_program, InfiniteVector, IntcodeIo, Opcode, ProgramMemory};
use std::cell::RefCell;
use std::convert::TryInto;
use std::io;
use std::ops::{Index, IndexMut};

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

impl Default for Tile {
    fn default() -> Self {
        Tile::Empty
    }
}

/// Range of a single axis of the drawing board.
type Axis = u32;

/// A coordinate on the drawing board.
#[derive(PartialEq, Eq, Hash, Clone)]
struct Coordinate(Axis, Axis);

#[derive(Default)]
struct TileScreen {
    data: InfiniteVector<InfiniteVector<Tile>>,
}

impl TileScreen {
    pub fn new() -> Self {
        TileScreen {
            ..Default::default()
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn iter(&self) -> core::slice::Iter<InfiniteVector<Tile>> {
        self.data.iter()
    }
}

impl Index<Coordinate> for TileScreen {
    type Output = Tile;

    fn index(&self, index: Coordinate) -> &Self::Output {
        let Coordinate(x, y) = index;
        &self.data[y as usize][x as usize]
    }
}

impl IndexMut<Coordinate> for TileScreen {
    fn index_mut(&mut self, index: Coordinate) -> &mut Self::Output {
        let Coordinate(x, y) = index;
        &mut self.data[y as usize][x as usize]
    }
}

impl Index<usize> for TileScreen {
    type Output = InfiniteVector<Tile>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for TileScreen {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

/// Buffers three outputs from the intcode program.
struct ArcadeCabinetOutputBuffer {
    buffer: [Option<Opcode>; 3],
    buffered_opcodes: u8,
}

/// Maps from intcode computer Opcode IO to a screen state and handles stdin and stdout for the player.
struct ArcadeCabinetIo {
    pub screen: RefCell<TileScreen>,
    pub score: RefCell<Opcode>,
    nmoves: RefCell<usize>,
    stdin: io::Stdin,
    buffered_output: RefCell<ArcadeCabinetOutputBuffer>,
    automatic_mode: bool,
}

impl ArcadeCabinetIo {
    pub fn new(automatic_mode: bool) -> Self {
        let stdin = io::stdin();
        let screen = RefCell::new(TileScreen::new());
        let nmoves = RefCell::new(0);
        let score = RefCell::new(0);
        let buffered_output = RefCell::new(ArcadeCabinetOutputBuffer {
            buffer: [None, None, None],
            buffered_opcodes: 0,
        });

        Self {
            stdin,
            screen,
            nmoves,
            score,
            buffered_output,
            automatic_mode,
        }
    }

    /// Print the current screen state to stdout.
    pub fn print_screen(&self) {
        let screen = self.screen.borrow_mut();
        for y in 0..screen.len() {
            let row = &screen[y];
            for x in 0..row.len() {
                let x = x.try_into().unwrap();
                let y = y.try_into().unwrap();
                let coord = Coordinate(x, y);
                let c: char = screen[coord].into();
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
        let x = buffered_output.buffer[0].take().unwrap();
        let y = buffered_output.buffer[1].take().unwrap();

        if x == -1 && y == 0 {
            // Special output coordinates, signaling the third opcode is the current score.
            let score = buffered_output.buffer[2].take().unwrap();
            *self.score.borrow_mut() = score;
        } else {
            // Otherwise, the third opcode maps to the tile at these coordinates.
            let tile: Tile = buffered_output.buffer[2].take().unwrap().into();
            let x: Axis = x.try_into().unwrap();
            let y: Axis = y.try_into().unwrap();
            let coord = Coordinate(x, y);
            self.screen.borrow_mut()[coord] = tile;
        }
    }

    pub fn moves(&self) -> usize {
        self.nmoves.borrow().clone()
    }
}

impl IntcodeIo for ArcadeCabinetIo {
    fn read(&self) -> Opcode {
        // The intcode program wants user input, the user should now get to see the current screen.
        self.print_screen();
        self.print_score();

        *self.nmoves.borrow_mut() += 1;

        if self.automatic_mode {
            let screen = self.screen.borrow();
            let x_ball = screen
                .iter()
                .find_map(|row| row.iter().position(|&t| t == Tile::Ball))
                .unwrap();
            let x_player = screen
                .iter()
                .find_map(|row| row.iter().position(|&t| t == Tile::HorizontalPaddle))
                .unwrap();
            (x_ball as Opcode - x_player as Opcode).signum().into()
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
            .iter()
            .map(|v| v.iter().filter(|&&t| t == tile).count())
            .sum()
    }

    pub fn moves(&self) -> usize {
        self.inout.moves()
    }
}
