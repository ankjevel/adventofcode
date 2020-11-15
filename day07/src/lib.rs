#[macro_use(lazy_static)]
extern crate lazy_static;
extern crate regex;

pub mod part_01;
pub mod part_02;

use regex::{Captures, Regex};
use std::collections::{HashMap, HashSet};

pub type Steps = HashMap<char, HashSet<char>>;

pub fn parse_input(string: &str) -> Steps {
    fn to_char(input: &Captures, index: usize) -> char {
        input.get(index).unwrap().as_str().chars().next().unwrap()
    }

    lazy_static! {
        static ref RE: Regex = Regex::new(r"Step (\w) .*? step (\w)").unwrap();
    }

    let mut steps: Steps = HashMap::new();

    string
        .lines()
        .map(|string| string.trim())
        .filter(|string| !string.is_empty())
        .for_each(|string| {
            if !RE.is_match(&string) {
                panic!("none matching for {}", string);
            }

            let cap = RE.captures(&string).unwrap();
            let required = to_char(&cap, 1);
            let step = to_char(&cap, 2);

            steps.entry(required).or_insert(HashSet::new());
            steps.entry(step).or_insert(HashSet::new());
            steps.get_mut(&step).unwrap().insert(required);
        });

    steps
}
