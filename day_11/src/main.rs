use ::day_11::{painting_robot::Robot, parse_input};
use day_09::program::Program;
use std::{sync::mpsc::channel, thread::spawn};

fn part_01(input: &Vec<i64>) -> usize {
    let (b_sender, a_receiver) = channel();
    let (a_sender, b_receiver) = channel();

    let mut program = Program::new(input);
    let mut robot = Robot::new(a_receiver, a_sender, false);

    spawn(move || {
        program.new_input(b_receiver);
        program.new_output(b_sender);
        program.run();
    });

    spawn(move || robot.run()).join().unwrap()
}

fn part_02(input: &Vec<i64>) {
    let (b_sender, a_receiver) = channel();
    let (a_sender, b_receiver) = channel();

    let mut program = Program::new(input);

    spawn(move || {
        program.new_input(b_receiver);
        program.new_output(b_sender);
        program.run();
    });

    spawn(move || {
        let mut robot = Robot::new(a_receiver, a_sender, true);
        robot.run();
        robot.print()
    })
    .join()
    .unwrap();
}

fn main() {
    let input = parse_input(&include_str!("../../input/day_11"));
    println!("part_01: {:?}", part_01(&input));
    println!("part_02:");
    part_02(&input);
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;

    #[test]
    fn gets_the_same_result_as_part_01() {
        let (b_sender, a_receiver) = channel();
        let (a_sender, _b_receiver) = channel();

        for input in vec![0, 0, 1, 0, 1, 0, 0, 1, 1, 0, 1, 0, -1] {
            b_sender.send(input).unwrap()
        }

        let mut robot = Robot::new(a_receiver, a_sender, false);

        let result = spawn(move || robot.run()).join().unwrap();

        assert_eq!(result, 6);
    }

    #[test]
    fn it_does_not_count_the_same_spot_twice() {
        let (b_sender, a_receiver) = channel();
        let (a_sender, _b_receiver) = channel();

        for input in vec![1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1] {
            b_sender.send(input).unwrap()
        }

        let mut robot = Robot::new(a_receiver, a_sender, false);

        let result = spawn(move || robot.run()).join().unwrap();

        assert_eq!(result, 4);
    }
}
