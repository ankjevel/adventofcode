use std::collections::HashMap;

use crate::Input;

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub enum Key {
    Mask,
    Mem(usize),
}

use Key::*;

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct Program {
    memory: HashMap<usize, u64>,
    mask: String,
}

impl Program {
    fn sum(&self) -> u64 {
        self.memory.values().sum()
    }

    fn get_value(&self, value: &str) -> u64 {
        let value = value.parse::<u64>().unwrap_or(0);
        let binary_ish = format!("{:0>36b}", value);
        let mask = self.mask.to_owned();
        let bin_idx = binary_ish
            .chars()
            .enumerate()
            .map(|(index, b)| {
                let b = if b == '0' { 0 } else { 1 };
                let n = match &mask.chars().nth(index).unwrap() {
                    '0' => b & 0,
                    '1' => b | 1,
                    _ => b,
                };
                n.to_string()
            })
            .collect::<String>();
        u64::from_str_radix(&bin_idx, 2).unwrap()
    }

    pub fn new() -> Self {
        Self {
            ..Program::default()
        }
    }

    pub fn run(&mut self, input: &Input) -> u64 {
        for (key, value) in input {
            match key {
                &Mask => {
                    self.mask = value.to_string();
                }
                &Mem(index) => {
                    *self.memory.entry(index).or_insert(0) = self.get_value(value);
                }
            }
        }

        self.sum()
    }
}
