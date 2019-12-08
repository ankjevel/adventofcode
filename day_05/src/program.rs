use std::{
    iter::Iterator,
    option::Option,
    sync::mpsc::{channel, Receiver, Sender},
    thread::{spawn, JoinHandle},
};

pub struct Program {
    original_memory: Vec<i32>,
    pointer: usize,
    input: Receiver<i32>,
    halted: bool,
    output: Sender<i32>,
    output_value: i32,
    pub memory: Vec<i32>,
}

impl Program {
    pub fn new(memory: &Vec<i32>) -> Program {
        let (output, input) = channel();

        Program {
            original_memory: memory.clone(),
            pointer: 0,
            output_value: 0,
            input,
            halted: false,
            memory: memory.clone(),
            output,
        }
    }

    pub fn new_input(&mut self, input: Receiver<i32>) {
        self.input = input;
    }

    pub fn new_output(&mut self, output: Sender<i32>) {
        self.output = output;
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
            let input = self.input.recv().unwrap_or(0);
            self.memory[index] = input.to_owned();
        } else {
            let output = self.memory[index].clone();
            self.output.send(output.to_owned()).unwrap_or(());
            self.output_value = output.to_owned();
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
        self.output_value = 0;
        self.halted = false;
        self.memory = self.original_memory.clone();

        while !self.halted {
            self.eval()
        }

        self.output_value.to_owned()
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
            9 => {
                self.halted = true;
                0
            }
            _ => panic!("[{}], opcode: {}", self.pointer, opcode),
        };
    }
}

pub fn exec(memory: Vec<i32>, input: Receiver<i32>, output: Sender<i32>) -> JoinHandle<i32> {
    spawn(move || {
        let mut program = Program::new(&memory);
        program.new_input(input);
        program.new_output(output);
        program.run()
    })
}

pub fn exec_without_channels(memory: Vec<i32>, input: Option<Vec<i32>>) -> i32 {
    let (c_out, c_in) = channel();
    match input {
        Some(input) => {
            for seq in input.clone() {
                c_out.send(seq).unwrap();
            }
        }
        None => {
            c_out.send(0).unwrap();
        }
    };
    spawn(move || {
        let mut program = Program::new(&memory);
        program.new_input(c_in);
        program.new_output(c_out);
        program.run()
    })
    .join()
    .unwrap()
}
