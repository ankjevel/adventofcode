use std::collections::VecDeque;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Operation {
    None,
    Add(Option<i64>),
    Mul(Option<i64>),
}

use Operation::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Monkey {
    pub items: VecDeque<i64>,
    operation: Operation,
    divisible_by: i64,
    if_true: isize,
    if_false: isize,
    pub inspected: i64,
}

impl Monkey {
    pub fn new(
        items: Vec<i64>,
        operation: Operation,
        divisible_by: i64,
        if_true: isize,
        if_false: isize,
    ) -> Self {
        Monkey {
            items: VecDeque::from_iter(items.into_iter()),
            operation,
            divisible_by,
            if_true,
            if_false,
            inspected: 0,
        }
    }

    pub fn handle_item(&self, item: &i64) -> i64 {
        let item = item.to_owned();
        match self.operation {
            Add(val) => {
                if let Some(val) = val {
                    item + val
                } else {
                    item + item
                }
            }
            Mul(val) => {
                if let Some(val) = val {
                    item * val
                } else {
                    item * item
                }
            }
            None => item,
        }
    }

    pub fn throw_to(&self, worry_index: i64) -> isize {
        if worry_index % self.divisible_by == 0 {
            self.if_true
        } else {
            self.if_false
        }
    }
}
