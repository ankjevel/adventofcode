use std::{iter::Iterator, option::Option};

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
enum Mode {
    Position,
    Immediate,
    Relative,
}

pub struct Program {
    original_memory: Vec<i64>,
    pointer: usize,
    relative_base: usize,
    halted: bool,
    pub input: Vec<i64>,
    pub output: Vec<i64>,
    pub memory: Vec<i64>,
}

impl Program {
    pub fn new(memory: &Vec<i64>) -> Program {
        Program {
            original_memory: memory.clone(),
            pointer: 0,
            relative_base: 0,
            input: vec![0],
            halted: false,
            memory: memory.clone(),
            output: vec![],
        }
    }

    fn get_next_input(&mut self) -> Option<i64> {
        match self.input.iter_mut().next() {
            Some(input) => Some(*input),
            _ => None,
        }
    }

    fn get_index(&mut self, step: usize, mode: Mode) -> usize {
        let parameter = self.pointer + step;

        match mode {
            Mode::Immediate => parameter,
            _ => self.memory[parameter] as usize,
        }
    }

    fn get_memory<'a>(&'a mut self, step: usize, mode: Mode) -> i64 {
        let mut index = self.get_index(step, mode);

        if mode == Mode::Relative {
            index = ((index as i64) + (self.relative_base as i64)) as usize;
        }

        if let Some(&value) = self.memory.get(index.to_owned() as usize) {
            value
        } else {
            0
        }
    }

    fn ensure_memory_available(&mut self, position: &usize) {
        if *position >= self.memory.len() {
            self.memory.resize(position + 1, 0);
        }
    }

    fn get_mut_memory_ref<'a>(&'a mut self, step: usize, mode: Mode) -> &'a mut i64 {
        let mut index = self.get_index(step, mode) as i64;

        if mode == Mode::Relative {
            index += self.relative_base as i64;
        }

        let usize_index = index as usize;

        self.ensure_memory_available(&usize_index);
        &mut self.memory[usize_index]
    }

    fn to_int(&mut self, input: &char) -> i64 {
        (&input.to_string()).parse::<i64>().unwrap()
    }

    fn mode(&mut self, input: &Option<char>) -> Mode {
        match input.unwrap_or('0') {
            '0' => Mode::Position,
            '1' => Mode::Immediate,
            '2' => Mode::Relative,
            _ => Mode::Position,
        }
    }

    fn opcode_1_2(
        &mut self,
        opcode: i64,
        mode_first: Mode,
        mode_second: Mode,
        mode_third: Mode,
    ) -> i64 {
        let a = self.get_memory(1, mode_first);
        let b = self.get_memory(2, mode_second);

        *self.get_mut_memory_ref(3, mode_third) = if opcode == 1 { a + b } else { a * b };
        4
    }

    fn opcode_3_4(&mut self, opcode: i64, mode: Mode) -> i64 {
        if opcode == 3 {
            match self.get_next_input() {
                Some(input) => {
                    *self.get_mut_memory_ref(1, mode) = input;
                }
                _ => {}
            }
        } else {
            let output = self.get_memory(1, mode);
            self.output.push(output);
        };
        2
    }

    fn opcode_5_6(&mut self, opcode: i64, mode_first: Mode, mode_second: Mode) -> i64 {
        let a = self.get_memory(1, mode_first);
        let b = self.get_memory(2, mode_second);

        if (opcode == 5 && a != 0) || (opcode == 6 && a == 0) {
            b - self.pointer as i64
        } else {
            3
        }
    }

    fn opcode_7_8(
        &mut self,
        opcode: i64,
        mode_first: Mode,
        mode_second: Mode,
        mode_third: Mode,
    ) -> i64 {
        let a = self.get_memory(1, mode_first);
        let b = self.get_memory(2, mode_second);

        *self.get_mut_memory_ref(3, mode_third) =
            if (opcode == 7 && a < b) || (opcode == 8 && a == b) {
                1
            } else {
                0
            };
        4
    }

    fn opcode_9(&mut self, mode: Mode) -> i64 {
        let memory = self.get_memory(1, mode);
        let relative_base = (self.relative_base as i64) + memory;
        self.relative_base = relative_base as usize;
        2
    }

    pub fn run(&mut self) -> i64 {
        self.pointer = 0;
        self.halted = false;
        self.memory = self.original_memory.clone();
        self.relative_base = 0;
        self.output.clear();

        while !self.halted {
            self.eval()
        }

        *self.output.clone().iter().last().unwrap_or(&0i64)
    }

    fn eval(&mut self) {
        let string = self.memory[self.pointer].to_string();
        let mut instuction = string.chars().rev();

        let opcode = self.to_int(&instuction.next().unwrap());
        let opcode_padding = self.to_int(&instuction.next().unwrap_or('0'));

        let mode_first = self.mode(&instuction.next());
        let mode_second = self.mode(&instuction.next());
        let mode_third = self.mode(&instuction.next());


        if opcode == 9 && opcode_padding == 9 {
            self.halted = true;
            return;
        }

        let next = match opcode {
            1 | 2 => self.opcode_1_2(opcode, mode_first, mode_second, mode_third),
            3 | 4 => self.opcode_3_4(opcode, mode_first),
            5 | 6 => self.opcode_5_6(opcode, mode_first, mode_second),
            7 | 8 => self.opcode_7_8(opcode, mode_first, mode_second, mode_third),
            9 => self.opcode_9(mode_first),
            _ => panic!("[{}], opcode: {}", self.pointer, opcode),
        };

        self.pointer = ((self.pointer as i64) + next) as usize;
    }
}

pub fn exec(memory: &Vec<i64>, input: Option<Vec<i64>>) -> i64 {
    let input_unwrapped = match input {
        Some(input) => input.to_owned(),
        None => vec![0],
    };

    let mut program = Program::new(memory);
    program.input = input_unwrapped;
    program.run()
}
