use std::io::Result;

use crate::{instruction::Instruction, instruction::Operation::*, Input};

#[derive(Default, Debug, Eq, PartialEq, Clone)]
struct Program {
    instructions: Vec<(bool, Instruction)>,
    accumulator: i64,
    index: usize,
}

impl Program {
    fn new(input: &Input) -> Self {
        Self {
            instructions: input
                .iter()
                .map(|input| (false, input.to_owned()))
                .collect(),
            ..Program::default()
        }
    }

    fn run(mut self) -> i64 {
        let len = self.instructions.len() as i64;
        let next = |curr: usize, next: &i64| -> usize {
            let next_index = curr as i64 + next;
            (if next_index > len {
                len - next_index
            } else {
                next_index
            } % len) as usize
        };

        loop {
            let (visited, instruction) = self.instructions.get_mut(self.index).unwrap();
            if *visited {
                break;
            }
            let argument = &instruction.argument;
            let next = next(
                self.index,
                if instruction.operation == JMP {
                    argument
                } else {
                    &1
                },
            );
            if instruction.operation == ACC {
                self.accumulator += argument
            }
            *visited = true;
            self.index = next;
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
        assert_eq!(main(&parse_input(&EXAMPLE_DATA))?, 5);
        Ok(())
    }
}
