//! Implementation of the Amplifier Circuit described in day 7 of the 2019 [Advent of Code](adventofcode.com)

use intcode_computer::{read_program_from_file, run_program, IntcodeIo, Opcode};
use std::cell::RefCell;
use std::sync::mpsc;
use std::thread;

/// Message type to be sent between threads.
pub enum Message {
    Data(Opcode),
    Exited,
}

/// Implementation of intcode_computer::IntcodeIo with mpsc channels.
pub struct IntcodeChannelIo {
    sender: mpsc::Sender<Message>,
    receiver: mpsc::Receiver<Message>,
}

impl IntcodeChannelIo {
    fn new(sender: mpsc::Sender<Message>, receiver: mpsc::Receiver<Message>) -> IntcodeChannelIo {
        IntcodeChannelIo { sender, receiver }
    }

    fn send_exit_signal(&self) {
        self.sender.send(Message::Exited).unwrap();
    }
}

impl IntcodeIo for IntcodeChannelIo {
    fn read(&self) -> Opcode {
        match self.receiver.recv().unwrap() {
            Message::Data(val) => val,
            _ => panic!("weird message"),
        }
    }

    fn write(&self, value: &Opcode) {
        self.sender.send(Message::Data(value.clone())).unwrap();
    }
}

/// An amplifier, that is initially set to a phase mode
/// and then able to receive a signal and amplify (send) it.
/// The underlying program is run in a seperate thread and might be waiting for input,
/// which it gets through an instance of IntcodeChannelIo.
pub struct Amplifier {
    phase_setting: u8,
    pub handle: thread::JoinHandle<()>,
    sender: mpsc::Sender<Message>,
    receiver: mpsc::Receiver<Message>,
    exited: RefCell<bool>,
}

impl Amplifier {
    pub fn new(program_file: &str, phase_setting: u8) -> Amplifier {
        let mut program = read_program_from_file(&program_file);

        // set up bidirectional channel
        let (host_sender, thread_receiver) = mpsc::channel();
        let (thread_sender, host_receiver) = mpsc::channel();
        let inout = IntcodeChannelIo::new(thread_sender, thread_receiver);

        let handle = thread::spawn(move || {
            run_program(&mut program, &inout);
            inout.send_exit_signal();
        });

        let amp = Amplifier {
            phase_setting,
            handle,
            sender: host_sender,
            receiver: host_receiver,
            exited: RefCell::new(false),
        };

        // initialize intcode program with the phase setting (further input will be the signals)
        amp.send(phase_setting.into());

        amp
    }

    /// Sends an Opcode to the underlying worker thread.
    pub fn send(&self, value: Opcode) {
        println!(
            "[Amp {}]: sending {} to worker...",
            self.phase_setting, value
        );
        self.sender
            .send(Message::Data(value))
            .unwrap_or_else(|err| {
                println!(
                    "[Amp {}]: appareanlty down; couldn't send; err: {}",
                    self.phase_setting, err
                );
            });
    }

    /// Receives an Opcode from the worker and maybe update internal exited field.
    pub fn recv(&self) -> Option<Opcode> {
        print!("[Amp {}]: ", self.phase_setting);
        match self.receiver.recv().unwrap() {
            Message::Data(val) => {
                println!("received {} from worker...", val);
                Some(val)
            }
            Message::Exited => {
                println!("exiting");
                *self.exited.borrow_mut() = true;
                None
            }
        }
    }

    /// Public getter for (internally mutable) "exited" field.
    pub fn has_exited(&self) -> bool {
        *self.exited.borrow()
    }
}
