//! IO over mpsc channels for the [Advent of Code](adventofcode.com) Intcode Computer.

use intcode_computer::{IntcodeIo, Opcode};
use std::sync::mpsc;

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
