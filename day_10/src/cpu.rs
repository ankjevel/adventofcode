use std::collections::BTreeMap;

use crate::{instruction::Instruction::*, Input};

pub struct CPU {
    cycles: usize,
    strength: i64,
    pub stack: BTreeMap<usize, i64>,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            cycles: 0,
            strength: 1,
            stack: BTreeMap::new(),
        }
    }

    pub fn store(&mut self, value: Option<i64>) {
        self.cycles += 1;
        self.stack.insert(self.cycles, self.strength);
        if let Some(unrapped) = value {
            self.strength += unrapped;
        }
    }

    pub fn parse(&mut self, instructions: &Input) {
        for instruction in instructions.to_owned() {
            self.store(None);
            if let AddX(val) = instruction {
                self.store(Some(val));
            }
        }
    }
}
