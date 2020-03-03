//! This is the actual solution to the task.

extern crate itertools;

use amplification_circuit::Amplifier;
use itertools::Itertools;

// https://docs.rs/itertools/0.8.2/itertools/trait.Itertools.html#method.permutations

#[allow(dead_code)]
fn first_part(filename: &str) {
    let mut max_signal = 0;
    for setting in (0..5).permutations(5) {
        let mut signal = 0;
        for phase in setting {
            let amp = Amplifier::new(&filename, phase);
            amp.send(signal.clone());
            signal = amp.recv().unwrap();
            // amp.handle.join().unwrap();
        }
        if signal > max_signal {
            max_signal = signal;
        }
    }
    println!("{}", max_signal);
}

#[allow(dead_code)]
fn second_part(filename: &str) {
    let mut max_signal = 0;
    for setting in (5..10).permutations(5) {
        let amplifiers: Vec<Amplifier> = setting
            .iter()
            .map(|i| Amplifier::new(&filename, *i))
            .collect();
        let mut signal = 0;
        while !amplifiers.last().unwrap().has_exited() {
            for i in 0..5 {
                amplifiers[i].send(signal.clone());
                // this will return Option::None if the thread has halted
                // in this case, we will go this loop one final time and the outer loop condition will fail afterwards
                signal = amplifiers[i].recv().unwrap_or(signal);
            }
        }

        println!("signal = {}", signal);

        if signal > max_signal {
            max_signal = signal;
        }
    }
    println!("max_signal = {}", max_signal);
}

fn main() {
    let mut args = std::env::args();
    args.next();
    let filename = args.next().unwrap_or("input.txt".to_string());
    first_part(&filename);
    second_part(&filename);
}
