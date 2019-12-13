extern crate day_09;
extern crate termion;

pub mod game;

use day_09::program::Program;
use std::{sync::mpsc::channel, thread::spawn};

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum Tile {
    Empty,
    Wall,
    Block,
    HorizontalPaddle,
    Ball,
}

use Tile::*;

impl Tile {
    pub fn from(input: &i64) -> Tile {
        match input {
            0 => Empty,
            1 => Wall,
            2 => Block,
            3 => HorizontalPaddle,
            4 => Ball,
            _ => panic!("enum not defined {}", input),
        }
    }
}

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

pub fn into_chunks(input: &Vec<i64>) -> Vec<(usize, usize, Tile)> {
    input
        .chunks(3)
        .into_iter()
        .map(|instructions| {
            let mut iter = instructions.into_iter();
            let x = iter.next().unwrap();
            let y = iter.next().unwrap();
            let id = iter.next().unwrap();
            (
                x.to_owned() as usize,
                y.to_owned() as usize,
                Tile::from(&id),
            )
        })
        .collect::<Vec<_>>()
}
