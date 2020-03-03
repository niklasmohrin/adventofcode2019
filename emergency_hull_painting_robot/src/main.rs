extern crate emergency_hull_painting_robot;

use emergency_hull_painting_robot::{Color, EmergencyHullPaintingRobot};
use intcode_computer::read_program_from_file;

fn main() {
    let mut args = std::env::args();
    args.next();
    let filename = args.next().unwrap_or(String::from("input.txt"));

    let program = read_program_from_file(&filename);
    let mut robot = EmergencyHullPaintingRobot::new(program);
    robot.run(Color::White);
    println!("{} moves", robot.moves);
    println!("{} field painted", robot.painted_panels.keys().len());
    robot.print_painting();
}
