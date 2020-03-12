use intcode_computer::read_program_from_file;
use repair_robot_control::RepairRobotControl;

fn main() {
    let mut args = std::env::args();
    args.next();
    let filename = args.next().unwrap_or(String::from("input.txt"));
    let program = read_program_from_file(&filename);
    let mut robot = RepairRobotControl::new(program);
    robot.run();
}
