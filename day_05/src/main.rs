use std::io::Result;
use std::option::Option;

struct Program {
    original_memory: Vec<i32>,
    memory: Vec<i32>,
    pointer: usize,
    input: i32,
    output: Vec<i32>,
}

impl Program {
    fn new(memory: &Vec<i32>) -> Program {
        Program {
            memory: memory.clone(),
            original_memory: memory.clone(),
            pointer: 0,
            input: 0,
            output: Vec::new(),
        }
    }

    fn get_index(&mut self, step: usize, positional: bool) -> usize {
        if positional {
            self.memory[self.pointer + step] as usize
        } else {
            self.pointer + step
        }
    }

    fn mode_1_2(&mut self, opcode: i32, positional_first: bool, positional_second: bool) -> usize {
        let noun_index = self.get_index(1, positional_first);
        let verb_index = self.get_index(2, positional_second);
        let out_index = self.get_index(3, true);
        let noun = self.memory[noun_index];
        let verb = self.memory[verb_index];
        self.memory[out_index] = if opcode == 1 {
            noun + verb
        } else {
            noun * verb
        };

        4
    }

    fn mode_3_4(&mut self, opcode: i32) -> usize {
        let index = self.get_index(1, true);
        if opcode == 3 {
            self.memory[index] = self.input.clone();
        } else {
            let output = self.memory[index].clone();
            self.output.push(output);
        };

        2
    }

    fn mode_5_6(
        &mut self,
        opcode: i32,
        position_mode_first: bool,
        position_mode_second: bool,
    ) -> usize {
        let first_index = self.get_index(1, position_mode_first);
        let second_index = self.get_index(2, position_mode_second);
        let param_1 = self.memory[first_index];
        let param_2 = self.memory[second_index];

        if (opcode == 5 && param_1 != 0) || (opcode == 6 && param_1 == 0) {
            param_2 as usize - self.pointer
        } else {
            3
        }
    }

    fn mode_7_8(
        &mut self,
        opcode: i32,
        position_mode_first: bool,
        position_mode_second: bool,
    ) -> usize {
        let first_index = self.get_index(1, position_mode_first);
        let second_index = self.get_index(2, position_mode_second);
        let third_index = self.get_index(3, true);

        let a = self.memory[first_index];
        let b = self.memory[second_index];

        self.memory[third_index] = if (opcode == 7 && a < b) || (opcode == 8 && a == b) {
            1
        } else {
            0
        };
        4
    }

    fn to_int(&mut self, input: &char) -> i32 {
        (&input.to_string()).parse::<i32>().unwrap()
    }

    fn position_mode(&mut self, input: &Option<char>) -> bool {
        input.unwrap_or('0') == '0'
    }

    fn run(&mut self) -> i32 {
        self.pointer = 0;
        self.output = Vec::new();
        self.memory = self.original_memory.clone();

        loop {
            let opcode = self.memory[self.pointer];

            if opcode == 99 {
                break self.memory[0];
            }

            let string = opcode.to_string();
            let mut instuction = string.chars().rev();
            let opcode = self.to_int(&instuction.next().unwrap());
            instuction.next();
            let positional_first = self.position_mode(&instuction.next());
            let positional_second = self.position_mode(&instuction.next());

            self.pointer += match opcode {
                1 | 2 => self.mode_1_2(opcode, positional_first, positional_second),
                3 | 4 => self.mode_3_4(opcode),
                5 | 6 => self.mode_5_6(opcode, positional_first, positional_second),
                7 | 8 => self.mode_7_8(opcode, positional_first, positional_second),
                _ => panic!("opcode: {}", opcode),
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
    println!("part_01: {:?}", program.output);

    program.input = 5;
    program.run();
    println!("part_02: {:?}", program.output);

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
    }

    #[test]
    fn examples_for_part_1() {
        let input = parse_input(EXAMPLE_DATA_01);

        let results = input
            .iter()
            .map(|row| {
                let mut program = Program::new(&row);
                program.run();
                program
            })
            .collect::<Vec<Program>>();

        assert_eq!(results[0].memory, [1101, 100, -1, 4, 99]);
        assert_eq!(results[1].memory, [1002, 4, 3, 4, 99]);
        assert_eq!(results[0].output, []);
        assert_eq!(results[1].output, []);
    }
}
