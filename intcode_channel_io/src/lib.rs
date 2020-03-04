//! IO over mpsc channels for the [Advent of Code](adventofcode.com) Intcode Computer.

use intcode_computer::{run_program, IntcodeIo, Opcode, ProgramMemory};
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
    pub fn new(
        sender: mpsc::Sender<Message>,
        receiver: mpsc::Receiver<Message>,
    ) -> IntcodeChannelIo {
        IntcodeChannelIo { sender, receiver }
    }

    pub fn send_exit_signal(&self) {
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

/// The interface for the worker thread
pub struct IntcodeThread {
    pub handle: thread::JoinHandle<ProgramMemory>,
    sender: mpsc::Sender<Message>,
    receiver: mpsc::Receiver<Message>,
    exited: RefCell<bool>,
    pub identifier: String,
}

impl IntcodeThread {
    pub fn new(mut program: ProgramMemory, identifier: Option<String>) -> IntcodeThread {
        // set up bidirectional channel
        let (host_sender, thread_receiver) = mpsc::channel();
        let (thread_sender, host_receiver) = mpsc::channel();
        let inout = IntcodeChannelIo::new(thread_sender, thread_receiver);

        let handle = thread::spawn(move || {
            run_program(&mut program, &inout);
            inout.send_exit_signal();
            program
        });

        let identifier = identifier.unwrap_or(String::from("Thread ?"));

        IntcodeThread {
            handle,
            sender: host_sender,
            receiver: host_receiver,
            exited: RefCell::new(false),
            identifier,
        }
    }

    /// Sends an Opcode to the underlying worker thread.
    pub fn send(&self, value: Opcode) {
        println!("[{}]: sending <{}> to worker", self.identifier, value);
        self.sender
            .send(Message::Data(value))
            .unwrap_or_else(|err| {
                println!(
                    "{}: worker appareanlty down; couldn't send; err: '{}'",
                    self.identifier, err
                );
            });
    }

    /// Receives an Opcode from the worker and maybe update internal exited field.
    pub fn recv(&self) -> Option<Opcode> {
        match self.receiver.recv().unwrap() {
            Message::Data(val) => {
                println!("[{}]: received <{}> from worker...", self.identifier, val);
                Some(val)
            }
            Message::Exited => {
                println!("[{}]: worker has exited", self.identifier);
                *self.exited.borrow_mut() = true;
                None
            }
        }
    }

    /// Public getter for (internally mutable) "exited" field.
    pub fn has_exited(&self) -> bool {
        *self.exited.borrow()
    }

    pub fn clone_sender(&self) -> mpsc::Sender<Message> {
        self.sender.clone()
    }
}
