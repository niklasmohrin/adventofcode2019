//! Rust implementation of the Emergency Hull Painting Robot described on day 11 of [Advent of Code 2019](adventofcode.com).

use intcode_channel_io::IntcodeThread;
use intcode_computer::{Opcode, ProgramMemory};
use std::collections::HashMap;

/// Orientation of the robot.
pub enum Orientation {
    Up,
    Down,
    Left,
    Right,
}

/// Direction, the robot can turn in after a move.
pub enum TurningDirection {
    Left,
    Right,
}

impl Orientation {
    pub fn turn(&mut self, dir: &TurningDirection) {
        use crate::Orientation::*;
        *self = match dir {
            TurningDirection::Left => match self {
                Up => Left,
                Left => Down,
                Down => Right,
                Right => Up,
            },
            TurningDirection::Right => match self {
                Up => Right,
                Right => Down,
                Down => Left,
                Left => Up,
            },
        };
    }
}

/// Range of a single axis of the drawing board.
type Axis = i32;

/// A coordinate on the drawing board.
#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Coordinate(Axis, Axis);

impl Coordinate {
    pub fn walk(&mut self, orientation: &Orientation) {
        use crate::Orientation::*;
        let dx = match orientation {
            Left => -1,
            Right => 1,
            _ => 0,
        };
        let dy = match orientation {
            Up => -1,
            Down => 1,
            _ => 0,
        };
        self.0 += dx;
        self.1 += dy;
    }
}

/// Color, that the robot can paint.
#[derive(Clone, Copy)]
pub enum Color {
    Black,
    White,
}

// Conversion between Opcode and Color, both ways

impl From<Color> for Opcode {
    fn from(color: Color) -> Self {
        match color {
            Color::Black => 0,
            Color::White => 1,
        }
    }
}

impl From<Opcode> for Color {
    fn from(opcode: Opcode) -> Self {
        match opcode {
            0 => Color::Black,
            1 => Color::White,
            _ => panic!("error converting opcode {} to a color", opcode),
        }
    }
}

/// The Emergency Hull Painting Robot as described on day 11 of [Advent of Code](adventofcode.com).
/// This robot will spin up a thread for its brain, running a provided Intcode Computer program.
/// Input and output of that computer instance is handled via the interface of the IntcodeThread,
/// which uses std::mpsc channels.
/// The robot records the drawn canvas as well as the total number of moves.
/// Internally, it keeps track of its position and orientation.
pub struct EmergencyHullPaintingRobot {
    pub painted_panels: HashMap<Coordinate, Color>,
    thread: IntcodeThread,
    position: Coordinate,
    orientation: Orientation,
    pub moves: usize,
}

const WHITE_PAINTING_CHAR: char = '#';
const BLACK_PAINTING_CHAR: char = ' ';

impl EmergencyHullPaintingRobot {
    /// Creates a new EmergencyHullPaintingRobot.
    /// The instance will take ownership of the ProgramMemory,
    /// since it will be passed onto the IntcodeThread, that transfers ownership
    /// into another thread.
    pub fn new(program: ProgramMemory) -> EmergencyHullPaintingRobot {
        let painted_panels = HashMap::new();
        let identifier = Some(String::from("Robot"));
        let thread = IntcodeThread::new(program, identifier);
        let position = Coordinate(0, 0);
        let orientation = Orientation::Up;
        let moves = 0;

        EmergencyHullPaintingRobot {
            painted_panels,
            thread,
            position,
            orientation,
            moves,
        }
    }

    /// Starts communication with the underlying robot brain.
    /// The color of the starting position must be provided.
    pub fn run(&mut self, starting_panel: Color) {
        self.painted_panels
            .insert(self.position.clone(), starting_panel);

        while !self.thread.has_exited() {
            // First, send the color underneath the robot to the brain.
            let current_color = *self
                .painted_panels
                .get(&self.position)
                .unwrap_or(&Color::Black);
            self.thread.send(current_color.into());

            // Then, the brain will send back the painted color, which is saved.
            let color_painted: Color = match self.thread.recv() {
                Some(opcode) => opcode.into(),
                // The thread has been shut down.
                None => break,
            };
            self.painted_panels
                .insert(self.position.clone(), color_painted);

            // After that, the brain signals in which direction it turned,
            // it will always turn 90 degress to one side.
            let turning_direction = match self.thread.recv() {
                Some(direction) => match direction {
                    0 => TurningDirection::Left,
                    1 => TurningDirection::Right,
                    _ => panic!("[Robot]: got invalid direction from worker"),
                },
                // The thread has been shut down.
                None => break,
            };

            // Finally, update the internal state.
            self.orientation.turn(&turning_direction);
            self.position.walk(&self.orientation);

            self.moves += 1;
        }
    }

    /// Prints the painted canvas to the console.
    pub fn print_painting(&self) {
        let coordinates: Vec<&Coordinate> = self.painted_panels.keys().collect();
        let xs: Vec<Axis> = coordinates.iter().map(|coord| coord.0).collect();
        let ys: Vec<Axis> = coordinates.iter().map(|coord| coord.1).collect();

        let x_min = *xs.iter().min().unwrap();
        let x_max = *xs.iter().max().unwrap();
        let y_min = *ys.iter().min().unwrap();
        let y_max = *ys.iter().max().unwrap();

        for y in y_min..=y_max {
            for x in x_min..=x_max {
                let coord = Coordinate(x, y);
                print!(
                    "{}",
                    match self.painted_panels.get(&coord).unwrap_or(&Color::Black) {
                        Color::White => WHITE_PAINTING_CHAR,
                        Color::Black => BLACK_PAINTING_CHAR,
                    }
                );
            }
            println!();
        }
    }
}
