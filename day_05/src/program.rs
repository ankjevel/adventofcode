use std::{
    iter::Iterator,
    option::Option,
    sync::mpsc::{channel, Receiver, Sender},
    thread::{spawn, JoinHandle},
};

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
    input: Receiver<i64>,
    halted: bool,
    output: Sender<i64>,
    output_value: i64,
    pub memory: Vec<i64>,
}

impl Program {
    pub fn new(memory: &Vec<i64>) -> Program {
        let (output, input) = channel();

        Program {
            original_memory: memory.clone(),
            pointer: 0,
            output_value: 0,
            relative_base: 0,
            input,
            halted: false,
            memory: memory.clone(),
            output,
        }
    }

    pub fn new_input(&mut self, receiver: Receiver<i64>) {
        self.input = receiver;
    }

    pub fn new_output(&mut self, sender: Sender<i64>) {
        self.output = sender;
    }

    pub fn is_halted(&self) -> bool {
        self.halted
    }

    fn get_index(&mut self, step: usize, mode: Mode) -> usize {
        match mode {
            Mode::Position => self.memory[self.pointer + step] as usize,
            Mode::Immediate => self.pointer + step,
            Mode::Relative => self.memory[self.pointer + step] as usize,
        }
    }

    fn ensure_memory_available(&mut self, position: &usize) {
        if *position >= self.memory.len() {
            self.memory.resize(position + 1, 0);
        }
    }

    fn get_memory<'a>(&'a mut self, step: usize, mode: Mode) -> i64 {
        let next = match mode {
            Mode::Relative => {
                let index = &(self.pointer + step);
                self.ensure_memory_available(&index);
                self.memory[*index].to_owned() + (self.relative_base as i64)
            }
            _ => {
                let index = &self.get_index(step, mode);
                self.ensure_memory_available(&index);
                self.memory[*index]
            }
        };

        next
    }

    fn get_mut_memory_ref<'a>(&'a mut self, index: &'a usize) -> &'a mut i64 {
        self.ensure_memory_available(&index);
        &mut self.memory[*index]
    }

    fn mode_1_2(&mut self, opcode: i64, mode_first: Mode, mode_second: Mode) -> i64 {
        let noun = self.get_memory(1, mode_first);
        let verb = self.get_memory(2, mode_second);
        let index = &self.get_index(3, Mode::Position);

        *self.get_mut_memory_ref(index) = if opcode == 1 {
            noun + verb
        } else {
            noun * verb
        };

        4
    }

    fn mode_3_4(&mut self, opcode: i64, mode: Mode) -> i64 {
        if opcode == 3 {
            let input = self.input.recv().unwrap_or(0);
            let index = &self.get_index(1, Mode::Position);
            *self.get_mut_memory_ref(index) = input.to_owned();
        } else {
            let output = self.get_memory(1, mode);
            self.output.send(output.to_owned()).unwrap_or(());
            self.output_value = output.to_owned();
        };

        2
    }

    fn mode_5_6(&mut self, opcode: i64, mode_first: Mode, mode_second: Mode) -> i64 {
        let param_1 = self.get_memory(1, mode_first);
        let param_2 = self.get_memory(2, mode_second);

        if (opcode == 5 && param_1 != 0) || (opcode == 6 && param_1 == 0) {
            param_2 - self.pointer as i64
        } else {
            3
        }
    }

    fn mode_7_8(&mut self, opcode: i64, mode_first: Mode, mode_second: Mode) -> i64 {
        let a = self.get_memory(1, mode_first);
        let b = self.get_memory(2, mode_second);
        let index = &self.get_index(3, Mode::Position);

        *self.get_mut_memory_ref(index) = if (opcode == 7 && a < b) || (opcode == 8 && a == b) {
            1
        } else {
            0
        };
        4
    }

    fn mode_9(&mut self, mode: Mode) -> i64 {
        let memory = self.get_memory(1, mode) as usize;
        let relative_base = self.relative_base + memory as usize;

        self.relative_base = relative_base;
        2
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

    pub fn run(&mut self) -> i64 {
        self.pointer = 0;
        self.output_value = 0;
        self.halted = false;
        self.memory = self.original_memory.clone();
        self.relative_base = 0;

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
        let mode_first = self.mode(&instuction.next());
        let mode_second = self.mode(&instuction.next());

        let next = match opcode {
            1 | 2 => self.mode_1_2(opcode, mode_first, mode_second),
            3 | 4 => self.mode_3_4(opcode, mode_first),
            5 | 6 => self.mode_5_6(opcode, mode_first, mode_second),
            7 | 8 => self.mode_7_8(opcode, mode_first, mode_second),
            9 => self.mode_9(mode_first),
            _ => panic!("[{}], opcode: {}", self.pointer, opcode),
        };

        self.pointer = ((self.pointer as i64) + next) as usize;
    }
}

pub fn exec(memory: Vec<i64>, receiver: Receiver<i64>, sender: Sender<i64>) -> JoinHandle<i64> {
    spawn(move || {
        let mut program = Program::new(&memory);
        program.new_input(receiver);
        program.new_output(sender);
        program.run()
    })
}

pub fn exec_without_channels(memory: Vec<i64>, input: Option<Vec<i64>>) -> i64 {
    let (sender, receiver) = channel();
    match input {
        Some(input) => {
            for seq in input.clone() {
                sender.send(seq).unwrap();
            }
        }
        None => {
            sender.send(0).unwrap();
        }
    };

    spawn(move || {
        let mut program = Program::new(&memory);
        program.new_input(receiver);
        program.new_output(sender);
        program.run()
    })
    .join()
    .unwrap()
}
