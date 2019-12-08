use std::iter::Iterator;
use std::option::Option;
// use std::collections::VecDeque;

pub struct Program {
    original_memory: Vec<i32>,
    pointer: usize,
    input_pointer: usize,
    input: Vec<i32>,
    halted: bool,
    pub memory: Vec<i32>,
    pub output: Vec<i32>,
}

impl Program {
    pub fn new(memory: &Vec<i32>) -> Program {
        Program {
            original_memory: memory.clone(),
            pointer: 0,
            input: vec![0],
            input_pointer: 0,
            halted: false,
            memory: memory.clone(),
            output: Vec::new(),
        }
    }

    pub fn new_input(&mut self, input: &Vec<i32>) {
        self.input_pointer = 0;
        self.input = input.clone();
    }

    pub fn is_halted(&self) -> bool {
        self.halted
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
            let input = self.input.get(self.input_pointer).unwrap_or(&0);
            self.input_pointer += 1;
            self.memory[index] = *input;
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

    pub fn run(&mut self) -> i32 {
        self.pointer = 0;
        self.output = Vec::new();
        self.memory = self.original_memory.clone();
        self.halted = false;

        while !self.halted {
            self.eval()
        }

        *self.output.last().unwrap_or(&0i32)
    }

    fn eval(&mut self) {
        let opcode = self.memory[self.pointer];

        if opcode == 99 {
            self.halted = true;
            return;
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
