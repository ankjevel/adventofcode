use itertools::Itertools;

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

    pub fn new() -> Self {
        Self {
            ..Program::default()
        }
    }

    fn get_value(&self, value: &str) -> u64 {
        let value = value.parse::<u64>().unwrap_or(0);
        let bin_idx = format!("{:0>36b}", value)
            .chars()
            .zip(self.mask.chars())
            .map(|(b, mask)| {
                let b = if b == '0' { 0 } else { 1 };
                let n = match mask {
                    '0' => b & 0,
                    '1' => b | 1,
                    _ => b,
                };
                n.to_string()
            })
            .collect::<String>();
        u64::from_str_radix(&bin_idx, 2).unwrap()
    }

    pub fn part_01(&mut self, input: &Input) -> u64 {
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

    fn get_combinations<'a>(&self, index: &usize) -> Vec<usize> {
        format!("{:0>36b}", index)
            .chars()
            .zip(self.mask.chars())
            .map(|(b, mask)| {
                let b = if b == '0' { 0 } else { 1 };
                match mask {
                    '0' => vec![b],
                    '1' => vec![1],
                    _ => vec![0, 1],
                }
            })
            .map(IntoIterator::into_iter)
            .multi_cartesian_product()
            .map(|vec| {
                let string = vec.iter().map(|i| i.to_string()).collect::<String>();
                usize::from_str_radix(&string, 2).unwrap()
            })
            .collect::<Vec<_>>()
    }

    pub fn part_02(&mut self, input: &Input) -> u64 {
        for (key, value) in input {
            match key {
                &Mask => {
                    self.mask = value.to_string();
                }
                &Mem(index) => {
                    let value = value.parse::<u64>().unwrap_or(0);
                    for index in self.get_combinations(&index) {
                        *self.memory.entry(index).or_insert(0) = value;
                    }
                }
            }
        }
        self.sum()
    }
}
