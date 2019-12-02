#[macro_use]
extern crate lazy_static;
extern crate regex;

mod part_01;
mod part_02;

use regex::{Captures, Regex};
use std::collections::{HashMap, HashSet};
use std::io::Result;

use part_01::main as part_01;
use part_02::main as part_02;

type Steps = HashMap<char, HashSet<char>>;

fn main() -> Result<()> {
    let input = parse_input(include_str!("../../input/day_07"));

    println!("part_01: {}", part_01(&input).unwrap());
    println!("part_02: {:?}", part_02(&input).unwrap());

    Ok(())
}

fn parse_input(string: &str) -> Steps {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn insert_into(steps: &mut Steps, step: char, required: Vec<char>) {
        steps.insert(step, required.into_iter().collect::<HashSet<char>>());
    }

    const EXAMPLE_DATA: &'static str = "
        Step C must be finished before step A can begin.
        Step C must be finished before step F can begin.
        Step A must be finished before step B can begin.
        Step A must be finished before step D can begin.
        Step B must be finished before step E can begin.
        Step D must be finished before step E can begin.
        Step F must be finished before step E can begin.
    ";

    #[test]
    fn it_should_parse_input() {
        let mut expected: Steps = HashMap::new();

        insert_into(&mut expected, 'C', vec![]);
        insert_into(&mut expected, 'A', vec!['C']);
        insert_into(&mut expected, 'B', vec!['A']);
        insert_into(&mut expected, 'D', vec!['A']);
        insert_into(&mut expected, 'F', vec!['C']);
        insert_into(&mut expected, 'E', vec!['B', 'D', 'F']);

        assert_eq!(parse_input(&EXAMPLE_DATA), expected)
    }

    #[test]
    fn it_should_get_result_as_example_01() {
        assert_eq!(part_01(&parse_input(&EXAMPLE_DATA)).unwrap(), "CABDFE")
    }
}
