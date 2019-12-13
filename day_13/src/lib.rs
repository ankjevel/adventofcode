extern crate day_09;

use day_09::program::Program;
use std::{sync::mpsc::channel, thread::spawn};

pub fn parse_input(input: &str) -> Vec<i64> {
    input
        .lines()
        .map(|string| string.trim())
        .filter(|string| !string.is_empty())
        .next()
        .unwrap()
        .split(',')
        .map(|part| part.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
}

pub fn get_instructions(input: &Vec<i64>) -> Vec<i64> {
    let (b_sender, a_receiver) = channel();
    let (_a_sender, b_receiver) = channel();

    let instructions = spawn(move || {
        let mut output = Vec::new();
        loop {
            match a_receiver.recv() {
                Ok(instruction) => output.push(instruction),
                _ => break,
            }
        }
        output
    });

    let mut program = Program::new(input);
    spawn(move || {
        program.new_input(b_receiver);
        program.new_output(b_sender);
        program.run();
    });

    instructions.join().unwrap()
}
