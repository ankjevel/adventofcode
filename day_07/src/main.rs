extern crate day_05;
extern crate permute;

use day_05::{parse_input, program::Program};
use permute::permute;

fn part_01(sequence: &Vec<i32>, memory: &Vec<i32>) -> i32 {
    let mut program = Program::new(&memory);
    let mut input = vec![0];
    for signal in sequence {
        input.insert(0, signal.to_owned());
        program.new_input(&input);
        input.insert(0, program.run());
    }
    input[0]
}

fn main() {
    let input = parse_input(include_str!("../../input/day_07"));
    let answer_part_01 = permute((0..=4).collect())
        .into_iter()
        .map(|sequence| part_01(&sequence, &input[0]))
        .max()
        .unwrap();

    permute((5..=9).collect())
        .into_iter()
        .for_each(|_sequence| {});

    // println!("part_02: {:?}", answer_part_02);
    println!("part_01: {:?}", answer_part_01);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLES_FOR_PART_01: &'static str = "
        3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0
        3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0
        3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0
    ";

    const EXAMPLES_FOR_PART_02: &'static str = "
        3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5
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
