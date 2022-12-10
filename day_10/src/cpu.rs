use std::collections::HashMap;

pub struct CPU {
    cycles: usize,
    strength: i64,
    pub stack: HashMap<usize, i64>,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            cycles: 0,
            strength: 1,
            stack: HashMap::new(),
        }
    }

    pub fn store(&mut self, value: Option<i64>) {
        self.cycles += 1;
        self.stack.insert(self.cycles, self.strength);
        if let Some(unrapped) = value {
            self.strength += unrapped;
        }
    }
}
