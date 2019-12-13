use ::day_13::{game::Game, get_instructions, into_chunks, parse_input, Tile::*};
use day_09::program::Program;
use std::{sync::mpsc::channel, thread::spawn};

fn part_01(input: &Vec<i64>) -> usize {
    let instructions = into_chunks(&get_instructions(input));
    instructions.iter().fold(
        0,
        |acc, (_x, _y, tile)| if tile == &Block { acc + 1 } else { acc },
    )
}

fn part_02(input: &Vec<i64>) -> u32 {
    let mut code = input.clone();
    code[0] = 2;

    let (b_sender, a_receiver) = channel();
    let (a_sender, b_receiver) = channel();

    let mut program = Program::new(&code);

    program.new_input(b_receiver);
    program.new_output(b_sender);

    spawn(move || program.run());
    spawn(move || Game::new(a_receiver, a_sender).run())
        .join()
        .unwrap()
}

fn main() {
    let input = parse_input(&include_str!("../../input/day_13"));
    let part_01_output = part_01(&input);
    let part_02_output = part_02(&input);

    println!("part_01: {}", part_01_output);
    println!("part_02: {}", part_02_output)
}
