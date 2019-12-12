use ::day_11::{painting_robot::Robot, parse_input};
use day_09::program::Program;
use std::{sync::mpsc::channel, thread::spawn};

fn main() {
    let (b_sender, a_receiver) = channel();
    let (a_sender, b_receiver) = channel();

    let mut program = Program::new(&parse_input(&include_str!("../../input/day_11")));
    let mut robot = Robot::new(a_receiver, a_sender);

    spawn(move || {
        program.new_input(b_receiver);
        program.new_output(b_sender);
        program.run();
    });

    let part_01_output = spawn(move || robot.run()).join().unwrap();
    println!("part_01, {}", part_01_output);
    // let mut robot = Robot::new(&instructions);
    // println!("part_01: {:?}", robot.run());
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {}
