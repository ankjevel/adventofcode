use std::io::Result;

use crate::{instruction::Instruction, instruction::Operation::*, Input};

#[derive(Default, Debug, Eq, PartialEq, Clone)]
struct Program {
    instructions: Vec<(bool, Instruction)>,
    original: Vec<Instruction>,
    accumulator: i64,
    index: usize,
    change: usize,
}

impl Program {
    fn new(input: &Input) -> Self {
        Self {
            original: input.to_owned(),
            ..Program::default()
        }
    }

    fn fix(&mut self) {
        let mut to_change = self.change.to_owned();
        let len = self.original.len();

        self.accumulator = 0;
        self.index = 0;

        self.instructions = self
            .original
            .iter()
            .enumerate()
            .map(|(index, instruction)| {
                if to_change != index {
                    return (false, instruction.to_owned());
                }

                let operation = instruction.operation;
                let argument = instruction.argument;

                match operation {
                    ACC => {
                        to_change += 1;
                        (false, instruction.to_owned())
                    }
                    NOP => (
                        false,
                        Instruction {
                            operation: JMP,
                            argument,
                        },
                    ),
                    JMP => (
                        false,
                        Instruction {
                            operation: NOP,
                            argument,
                        },
                    ),
                }
            })
            .collect::<Vec<_>>();

        self.change = to_change + 1 % len;
    }

    fn run(mut self) -> i64 {
        let len = self.original.len() as i64;
        let next = |curr: usize, next: &i64| -> usize {
            let next_index = curr as i64 + next;
            (if next_index > len {
                len - next_index
            } else {
                next_index
            } % len) as usize
        };

        'main: loop {
            self.fix();

            'test: loop {
                let (visited, instruction) = self.instructions.get_mut(self.index).unwrap();
                if *visited {
                    break 'test;
                }

                let argument = &instruction.argument;
                let operation = instruction.operation;
                let next = next(self.index, if operation == JMP { argument } else { &1 });

                if operation == ACC {
                    self.accumulator += argument;
                }

                if self.index != 0 && next == 0 {
                    break 'main;
                }

                *visited = true;
                self.index = next;
            }
        }

        self.accumulator
    }
}

pub fn main(input: &Input) -> Result<i64> {
    Ok(Program::new(input).run())
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    use super::*;

    const EXAMPLE_DATA: &'static str = "
        nop +0
        acc +1
        jmp +4
        acc +3
        jmp -3
        acc -99
        acc +1
        jmp -4
        acc +6
    ";

    #[test]
    fn it_gets_the_example_correct() -> Result<()> {
        assert_eq!(main(&parse_input(&EXAMPLE_DATA))?, 8);
        Ok(())
    }
}
