use std::{
    iter::Iterator,
    option::Option,
    sync::mpsc::{channel, Receiver, Sender},
    thread::spawn,
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
    halted: bool,
    input: Receiver<i64>,
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
            relative_base: 0,
            input,
            halted: false,
            memory: memory.clone(),
            output,
            output_value: 0,
        }
    }

    pub fn new_input(&mut self, receiver: Receiver<i64>) {
        self.input = receiver;
    }

    pub fn new_output(&mut self, sender: Sender<i64>) {
        self.output = sender;
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
            let input = self.input.recv().unwrap_or(0);
            *self.get_mut_memory_ref(1, mode) = input;
        } else {
            let output = self.get_memory(1, mode);
            self.output.send(output.to_owned()).unwrap();
            self.output_value = output;
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

    pub fn run(&mut self) -> i64 {
        self.pointer = 0;
        self.output_value = 0;
        self.halted = false;
        self.memory = self.original_memory.clone();
        self.relative_base = 0;

        while !self.halted {
            self.eval();
        }

        self.output_value.to_owned()
    }
}

pub fn exec(memory: Vec<i64>, output: Option<Vec<i64>>) -> i64 {
    let (program_sender, _exec_reciever) = channel();
    let (exec_sender, program_receiver) = channel();

    if let Some(vec) = output {
        vec.into_iter()
            .for_each(|val| exec_sender.send(val).unwrap())
    };

    let handle = spawn(move || {
        let mut program = Program::new(&memory);
        program.new_input(program_receiver);
        program.new_output(program_sender);
        program.run()
    });

    handle.join().unwrap()
}

pub fn run<F: Send + Sync + 'static, T: Send + Sync + 'static>(input: &Vec<i64>, fun: F) -> T
where
    F: Fn(Receiver<i64>, Sender<i64>) -> T,
{
    let (b_sender, a_receiver) = channel();
    let (a_sender, b_receiver) = channel();

    let mut program = Program::new(input);

    program.new_input(b_receiver);
    program.new_output(b_sender);

    spawn(move || program.run());
    spawn(move || fun(a_receiver, a_sender)).join().unwrap()
}
