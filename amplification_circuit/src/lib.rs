//! Implementation of the Amplifier Circuit described in day 7 of the 2019 [Advent of Code](adventofcode.com)

use intcode_channel_io::IntcodeThread;
use intcode_computer::{read_program_from_file, Opcode};

/// An amplifier, that is initially set to a phase mode
/// and then able to receive a signal and amplify (send) it.
/// The underlying program is run in a seperate thread and might be waiting for input,
/// which it gets through an instance of IntcodeChannelIo.
pub struct Amplifier {
    phase_setting: u8,
    thread: IntcodeThread,
}

impl Amplifier {
    pub fn new(program_file: &str, phase_setting: u8) -> Amplifier {
        let program = read_program_from_file(&program_file);
        let identifier = Some(format!("Amp {}", phase_setting));

        let thread = IntcodeThread::new(program, identifier);

        let amp = Amplifier {
            phase_setting,
            thread,
        };

        // initialize intcode program with the phase setting (further input will be the signals)
        amp.send(phase_setting.into());

        amp
    }

    /// Sends an Opcode to the underlying worker thread.
    pub fn send(&self, value: Opcode) {
        self.thread.send(value);
    }

    /// Receives an Opcode from the worker and maybe update internal exited field.
    pub fn recv(&self) -> Option<Opcode> {
        self.thread.recv()
    }

    pub fn has_exited(&self) -> bool {
        self.thread.has_exited()
    }
}
