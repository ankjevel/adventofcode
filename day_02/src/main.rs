#![feature(label_break_value)]

use std::io::Result;

struct Program {
    input: Vec<u32>,
    pointer: usize,
}

impl Program {
    fn new(input: &Vec<u32>) -> Program {
        Program {
            input: input.clone(),
            pointer: 0,
        }
    }

    fn patch(&mut self, index: usize, value: u32) {
        self.input[index] = value;
    }

    fn run(&mut self) -> u32 {
        self.pointer = 0;

        loop {
            let opcode = self.input[self.pointer];

            if opcode == 99 {
                break self.input[0];
            }

            let noun = self.input[self.input[self.pointer + 1] as usize];
            let verb = self.input[self.input[self.pointer + 2] as usize];
            let output_position = self.input[self.pointer + 3] as usize;

            self.input[output_position] = match opcode {
                1 => noun + verb,
                2 => noun * verb,
                _ => panic!("unexpected opcode: {}", opcode),
            };

            self.pointer += 4;
        }
    }
}

fn parse_input(string: &str) -> Vec<Vec<u32>> {
    string
        .lines()
        .map(|string| string.trim())
        .filter(|string| !string.is_empty())
        .map(|string| {
            string
                .split(',')
                .map(|part| part.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect()
}

fn main() -> Result<()> {
    let input = parse_input(include_str!("../../input/day_02"));

    'part_01: {
        let mut part_01 = Program::new(&input[0]);
        part_01.patch(1, 12);
        part_01.patch(2, 2);

        println!("part_01: {}", part_01.run());
    }

    'part_02: {
        let look_for = 19690720;
        for noun in 0..=99 {
            for verb in 0..=99 {
                let mut part_02 = Program::new(&input[0]);
                part_02.patch(1, noun);
                part_02.patch(2, verb);

                if part_02.run() == look_for {
                    println!("part_02: {}", (100 * noun + verb));
                    break;
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA_01: &'static str = "
    1,0,0,0,99
    2,3,0,3,99
    2,4,4,5,99,0
    1,1,1,4,99,5,6,0,99";

    #[test]
    fn examples_for_part_1() {
        let input = parse_input(EXAMPLE_DATA_01);

        let results = input
            .iter()
            .map(|row| {
                let mut program = Program::new(&row);
                program.run();

                program.input
            })
            .collect::<Vec<Vec<u32>>>();

        assert_eq!(results[0], [2, 0, 0, 0, 99]);
        assert_eq!(results[1], [2, 3, 0, 6, 99]);
        assert_eq!(results[2], [2, 4, 4, 5, 99, 9801]);
        assert_eq!(results[3], [30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    #[test]
    fn patch_paramters() {
        let mut program = Program::new(&vec![1, 0, 0, 0, 99]);
        program.patch(1, 3);
        program.patch(2, 1);

        let output = program.run();

        assert_eq!(program.input, [3, 3, 1, 0, 99]);
        assert_eq!(output, 3);
    }
}
