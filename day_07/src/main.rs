extern crate day_05;
extern crate permute;

use day_05::{
    parse_input,
    program::{exec, exec_without_channels},
};
use permute::permute;
use std::{iter::Iterator, sync::mpsc::channel};

fn part_01(sequence: &Vec<i64>, memory: &Vec<i64>) -> i64 {
    let mut input = vec![0];
    for signal in sequence {
        input.insert(0, signal.to_owned());
        input.insert(
            0,
            exec_without_channels(memory.clone(), Some(input.clone())),
        )
    }
    input[0]
}

fn part_02(sequence: &Vec<i64>, memory: &Vec<i64>) -> i64 {
    let mut seq = sequence.to_owned();
    let mut iter = seq.iter_mut();

    let (e_sender, a_receiver) = channel();
    let (a_sender, b_receiver) = channel();
    let (b_sender, c_receiver) = channel();
    let (c_sender, d_receiver) = channel();
    let (d_sender, e_receiver) = channel();

    e_sender.send(*iter.next().unwrap()).unwrap();
    a_sender.send(*iter.next().unwrap()).unwrap();
    b_sender.send(*iter.next().unwrap()).unwrap();
    c_sender.send(*iter.next().unwrap()).unwrap();
    d_sender.send(*iter.next().unwrap()).unwrap();
    e_sender.send(0).unwrap();

    exec(memory.clone(), a_receiver, a_sender);
    exec(memory.clone(), b_receiver, b_sender);
    exec(memory.clone(), c_receiver, c_sender);
    exec(memory.clone(), d_receiver, d_sender);
    exec(memory.clone(), e_receiver, e_sender).join().unwrap()
}

fn main() {
    let input = parse_input(include_str!("../../input/day_07"));
    let answer_part_01 = permute((0..=4).collect())
        .into_iter()
        .map(|sequence| part_01(&sequence, &input[0]))
        .max()
        .unwrap_or(0);

    let answer_part_02 = permute((5..=9).collect())
        .into_iter()
        .map(|sequence| part_02(&sequence, &input[0]))
        .max()
        .unwrap_or(0);

    println!("part_01: {:?}", answer_part_01);
    println!("part_02: {:?}", answer_part_02);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLES_FOR_PART_01: &'static str = "
        3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0
        3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0
        3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0
    ";

    #[test]
    fn it_gets_the_examples_right_on_part_01() {
        let input = parse_input(&EXAMPLES_FOR_PART_01);

        assert_eq!(part_01(&vec![4, 3, 2, 1, 0], &input[0]), 43210);
        assert_eq!(part_01(&vec![0, 1, 2, 3, 4], &input[1]), 54321);
        assert_eq!(part_01(&vec![1, 0, 4, 3, 2], &input[2]), 65210);
    }
}
