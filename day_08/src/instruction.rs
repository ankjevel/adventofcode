#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Operation {
    NOP,
    JMP,
    ACC,
}

use Operation::*;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Instruction {
    pub operation: Operation,
    pub argument: i64,
}

impl Instruction {
    pub fn new(operation: &str, argument: i64) -> Self {
        Self {
            operation: match operation {
                "jmp" => JMP,
                "acc" => ACC,
                _ => NOP,
            },
            argument,
        }
    }
}
