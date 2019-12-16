use ::day_15::{parse_input, repair_droid::RepairDroid};
use day_09::program::Program;
use std::{sync::mpsc::channel, thread::spawn};

fn main() {
    let input = parse_input(&include_str!("../../input/day_15"));

    println!("{:?}", part_01(&input));
}

fn part_01(input: &Vec<i64>) -> usize {
    let (b_sender, a_receiver) = channel();
    let (a_sender, b_receiver) = channel();

    let mut program = Program::new(&input);

    program.new_input(b_receiver);
    program.new_output(b_sender);

    spawn(move || program.run());
    spawn(move || RepairDroid::new(a_receiver, a_sender).run())
        .join()
        .unwrap()
}
