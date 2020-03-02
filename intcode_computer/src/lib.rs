//! Implementation of an Intcode Computer as described in the 2019 [Advent of Code](adventofcode.com)

use std::collections::HashMap;
use std::convert::TryInto;
use std::fs;
use std::io;

pub type Opcode = i32;
pub type ProgramMemory = Vec<Opcode>;

/// IO of the Computer can be done through any struct that implements this.
pub trait IntcodeIo {
    fn read(&self) -> Opcode;
    fn write(&self, value: &Opcode);
}

/// Implementation of IntcodeIo using stdin and stdout (println! macro).
struct IntcodeStdIo {
    stdin: io::Stdin,
}

impl IntcodeStdIo {
    fn new(stdin: io::Stdin) -> IntcodeStdIo {
        IntcodeStdIo { stdin }
    }
}

impl IntcodeIo for IntcodeStdIo {
    fn read(&self) -> Opcode {
        let mut val = String::new();
        self.stdin.read_line(&mut val).unwrap();
        val.trim().parse().unwrap()
    }

    fn write(&self, value: &Opcode) {
        println!("{}", value);
    }
}

/// Pairs of instruction part of an opcode and the correspondig length (including parameters).
/// This array can (and will) be used to initialize a HashMap<u8, u8> for faster lookup.
const OPCODE_LENGHTS_ARR: [(u8, u8); 9] = [
    (99, 1),
    (1, 4),
    (2, 4),
    (3, 2),
    (4, 2),
    (5, 3),
    (6, 3),
    (7, 4),
    (8, 4),
];

/// Run a program as described in the challenges of [Advent of Code](adventofcode.com).
pub fn run_program<T: IntcodeIo>(program: &mut ProgramMemory, inout: &T) {
    let opcode_lenghts: HashMap<u8, u8> = OPCODE_LENGHTS_ARR.iter().cloned().collect();

    // program counter, starting at index 0
    let mut pc = 0usize;

    loop {
        // fetch the opcode and split it into instruction and modes
        let opcode = program[pc];
        let instruction: u8 = (opcode % 100).try_into().unwrap();
        let modes = [
            (opcode / 100) % 10,
            (opcode / 1000) % 10,
            (opcode / 10000) % 10,
        ];

        let op_len = opcode_lenghts[&instruction] as usize;

        // determine parameter addresses according to parameter modes
        // the needed values are right after the opcode
        let parameter_adrs: Vec<usize> = (0usize..(op_len - 1))
            .map(|i| match modes[i] {
                0 => program[pc + 1usize + i as usize] as usize,
                1 => pc + 1usize + i as usize,
                _ => panic!("unsupported operand mode!"),
            })
            .collect();

        // set if a jump is performed; if not the pc will have to be incremented according to the opcode length and parameter count
        let mut pc_jumped = false;

        match instruction {
            // halt the program
            99 => break,
            // addition
            1 => {
                let p1 = program[parameter_adrs[0]];
                let p2 = program[parameter_adrs[1]];
                program[parameter_adrs[2]] = p1 + p2;
            }
            // multiplication
            2 => {
                let p1 = program[parameter_adrs[0]];
                let p2 = program[parameter_adrs[1]];
                program[parameter_adrs[2]] = p1 * p2;
            }
            // input
            3 => {
                let val = inout.read();
                program[parameter_adrs[0]] = val;
            }
            // output
            4 => {
                let val = program[parameter_adrs[0]];
                inout.write(&val);
            }
            // jump not equal
            5 => {
                let p1 = program[parameter_adrs[0]];
                let p2 = program[parameter_adrs[1]];
                if p1 != 0 {
                    pc_jumped = true;
                    pc = p2.try_into().unwrap();
                }
            }
            // jump equal
            6 => {
                let p1 = program[parameter_adrs[0]];
                let p2 = program[parameter_adrs[1]];
                if p1 == 0 {
                    pc_jumped = true;
                    pc = p2.try_into().unwrap();
                }
            }
            // less than
            7 => {
                let p1 = program[parameter_adrs[0]];
                let p2 = program[parameter_adrs[1]];
                program[parameter_adrs[2]] = (p1 < p2).into();
            }
            // equality
            8 => {
                let p1 = program[parameter_adrs[0]];
                let p2 = program[parameter_adrs[1]];
                program[parameter_adrs[2]] = (p1 == p2).into();
            }
            _ => panic!("unsupported instruction!"),
        };

        // pc should not be incremented if the current instruction triggered a jump
        if !pc_jumped {
            pc += op_len;
        }
    }
}

/// read and parse an intcode program file
pub fn read_program_from_file(filename: &str) -> ProgramMemory {
    fs::read_to_string(filename)
        .unwrap()
        .split(",")
        .map(|s| s.trim().parse::<Opcode>().unwrap())
        .collect()
}

/// read, parse and execute an intcode program file
pub fn run_program_from_file(filename: &str) {
    let mut program = read_program_from_file(filename);
    let stdin = io::stdin();
    let inout = IntcodeStdIo::new(stdin);
    run_program(&mut program, &inout);
}
