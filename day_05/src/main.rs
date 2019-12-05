#![feature(label_break_value)]
use std::io::Result;
use std::option::Option;

struct Program {
    memory: Vec<i32>,
    pointer: usize,
    input: i32,
    output: Vec<i32>,
}

impl Program {
    fn new(memory: &Vec<i32>) -> Program {
        Program {
            memory: memory.clone(),
            pointer: 0,
            input: 0,
            output: Vec::new(),
        }
    }

    fn mode_1_2(&mut self, opcode: i32, position_mode_noun: bool, position_mode_verb: bool, position_mode_pos: bool) {
        let mut noun_index: usize = self.pointer + 1;
        let mut verb_index: usize = self.pointer + 2;
        let mut out_index: usize = self.pointer + 3;

        if position_mode_noun {
            noun_index = self.memory[noun_index] as usize;
        }
        if position_mode_verb {
            verb_index = self.memory[verb_index] as usize;
        };
        if position_mode_pos {
            out_index = self.memory[out_index] as usize;
        };

        let noun = self.memory[noun_index];
        let verb = self.memory[verb_index];

        self.memory[out_index] = if opcode == 1 {
            noun + verb
        } else {
            noun * verb
        };
    }

    fn mode_3_4(&mut self, opcode: i32, position_mode_noun: bool) {
        let index = if position_mode_noun {
            self.memory[self.pointer + 1] as usize
        } else {
            self.pointer + 1
        };

        if opcode == 3 {
            self.memory[index] = self.input.clone();
        } else {
            self.output.push(self.memory[index].clone());
        }
    }

    fn to_int(&mut self, input: &char) -> i32 {
        (&input.to_string()).parse::<i32>().unwrap()
    }

    fn position_mode(&mut self, input: &Option<char>) -> bool {
        input.unwrap_or('0') == '0'
    }

    fn run(&mut self) -> i32 {
        self.pointer = 0;

        loop {
            let opcode = self.memory[self.pointer];

            if opcode == 99 {
                break self.memory[0];
            }

            match opcode {
                1 | 2 => {
                    self.mode_1_2(opcode, true, true, true);
                    self.pointer += 4;
                }
                3 | 4 => {
                    self.mode_3_4(opcode, true);
                    self.pointer += 2;
                }
                _ => {
                    let string = opcode.to_string();
                    let mut instuction = string.chars().rev();
                    let opcode = self.to_int(&instuction.next().unwrap());
                    instuction.next();
                    match opcode {
                        1 | 2 => {
                            let mut pointer = instuction.clone();
                            let position_mode_noun = self.position_mode(&pointer.next());
                            let position_mode_verb = self.position_mode(&pointer.next());
                            let position_mode_pos = self.position_mode(&pointer.next());
                            self.mode_1_2(opcode, position_mode_noun, position_mode_verb, position_mode_pos);
                            self.pointer += 4
                        }
                        3 | 4 => {
                            let position_mode = self.position_mode(&instuction.next());
                            self.mode_3_4(opcode, position_mode);
                            self.pointer += 2
                        }
                        _ => {
                            panic!("opcode: {}", opcode)
                        }
                    }
                }
            };
        }
    }
}

fn parse_input(string: &str) -> Vec<Vec<i32>> {
    string
        .lines()
        .map(|string| string.trim())
        .filter(|string| !string.is_empty())
        .map(|string| {
            string
                .split(',')
                .map(|part| part.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect()
}

fn main() -> Result<()> {
    let input = parse_input(include_str!("../../input/day_05"));

    let mut program = Program::new(&input[0]);

    program.input = 1;
    program.run();

    println!("part_01: {}", program.run());
    println!("part_01: {:?}", program.output);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA_FROM_DAY_02: &'static str = "
        1,0,0,0,99
        2,3,0,3,99
        2,4,4,5,99,0
        1,1,1,4,99,5,6,0,99
    ";

    const EXAMPLE_DATA_01: &'static str = "
        1101,100,-1,4,0
        1002,4,3,4,33
    ";

    #[test]
    fn it_still_works_with_example_data_from_day_02() {
        println!("\n");
        let input = parse_input(EXAMPLE_DATA_FROM_DAY_02);

        let results = input
            .iter()
            .map(|row| {
                let mut program = Program::new(&row);
                program.run();

                program.memory
            })
            .collect::<Vec<Vec<i32>>>();

        assert_eq!(results[0], [2, 0, 0, 0, 99]);
        assert_eq!(results[1], [2, 3, 0, 6, 99]);
        assert_eq!(results[2], [2, 4, 4, 5, 99, 9801]);
        assert_eq!(results[3], [30, 1, 1, 4, 2, 5, 6, 0, 99]);
        println!("\n");
    }

    #[test]
    fn examples_for_part_1() {
        let input = parse_input(EXAMPLE_DATA_01);

        let results = input
            .iter()
            .map(|row| {
                let mut program = Program::new(&row);
                program.run();
                program.memory
            })
            .collect::<Vec<Vec<i32>>>();

        assert_eq!(results[0], [1101, 100, -1, 4, 99]);
        assert_eq!(results[1], [1002, 4, 3, 4, 99]);
    }
}
