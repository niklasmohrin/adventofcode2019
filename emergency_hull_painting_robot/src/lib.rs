use intcode_channel_io::IntcodeThread;
use intcode_computer::{Opcode, ProgramMemory};
use std::collections::HashMap;

pub enum Orientation {
    Up,
    Down,
    Left,
    Right,
}

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

type Axis = i32;

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

#[derive(Clone, Copy)]
pub enum Color {
    Black,
    White,
}

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

    pub fn run(&mut self, starting_panel: Color) {
        self.painted_panels
            .insert(self.position.clone(), starting_panel);

        while !self.thread.has_exited() {
            let current_color = *self
                .painted_panels
                .get(&self.position)
                .unwrap_or(&Color::Black);
            self.thread.send(current_color.into());

            let color_painted: Color = match self.thread.recv() {
                Some(opcode) => opcode.into(),
                None => break,
            };

            self.painted_panels
                .insert(self.position.clone(), color_painted);

            let turning_direction = match self.thread.recv() {
                Some(direction) => match direction {
                    0 => TurningDirection::Left,
                    1 => TurningDirection::Right,
                    _ => panic!("[Robot]: got invalid direction from worker"),
                },
                None => break,
            };

            self.orientation.turn(&turning_direction);
            self.position.walk(&self.orientation);

            self.moves += 1;
        }
    }

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
