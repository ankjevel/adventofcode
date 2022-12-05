#[macro_use(lazy_static)]
extern crate lazy_static;
extern crate regex;

pub mod part_01;
pub mod part_02;

use std::collections::{BTreeMap, VecDeque};

use regex::Regex;

lazy_static! {
    static ref ROW: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    static ref INDICES: Regex = Regex::new(r"(\d+)").unwrap();
    static ref STACK: Regex = Regex::new(r"[(\w)]").unwrap();
}

pub type InputStack = BTreeMap<usize, VecDeque<char>>;
pub type Input = (InputStack, Vec<(u32, usize, usize)>);

pub fn parse_input(input: &str) -> Input {
    let mut stacks: InputStack = BTreeMap::new();

    let mut stack_lines = input
        .lines()
        .filter(|string| !string.contains("move") && !string.trim().is_empty())
        .map(str::to_owned)
        .collect::<Vec<String>>();

    let indices = stack_lines.pop().unwrap();
    let trim_to = indices.find(char::is_alphanumeric).unwrap() - 1;

    let lines = stack_lines
        .to_owned()
        .into_iter()
        .map(|line| {
            let (_, line) = line.trim_end().split_at(trim_to);
            line.chars()
                .collect::<Vec<char>>()
                .chunks(4)
                .map(|c| c.iter().collect::<String>())
                .map(|string| string.chars().nth(1).unwrap())
                .collect()
        })
        .collect::<Vec<Vec<char>>>();
    INDICES
        .find_iter(&indices)
        .map(|c| c.to_owned().as_str().parse::<usize>().unwrap_or(0))
        .into_iter()
        .enumerate()
        .for_each(|(i, n)| {
            stacks.insert(
                n,
                lines
                    .to_owned()
                    .iter()
                    .filter_map(|line| {
                        if let Some(row) = line.get(i) {
                            if row.is_whitespace() {
                                None
                            } else {
                                Some(row.to_owned())
                            }
                        } else {
                            None
                        }
                    })
                    .collect(),
            );
        });

    (
        stacks,
        input
            .lines()
            .map(str::trim)
            .filter(|string| !string.is_empty() && string.starts_with('m'))
            .map(str::to_owned)
            .map(|string| {
                let unwrapped = ROW
                    .captures(&string)
                    .unwrap()
                    .iter()
                    .map(|c| c.unwrap().to_owned().as_str())
                    .collect::<Vec<&str>>();

                (
                    unwrapped[1].parse::<u32>().unwrap_or(0).to_owned(),
                    unwrapped[2].parse::<usize>().unwrap_or(0).to_owned(),
                    unwrapped[3].parse::<usize>().unwrap_or(0).to_owned(),
                )
            })
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &'static str = "
                [F]
        [A]     [E]
        [B] [C] [D]   
         1   2   3
        
        move 1 from 2 to 1
        move 3 from 1 to 3
    ";

    #[test]
    fn it_parses_example() {
        assert_eq!(
            parse_input(&EXAMPLE_DATA),
            (
                BTreeMap::from([
                    (1, VecDeque::from(['A', 'B'])),
                    (2, VecDeque::from(['C'])),
                    (3, VecDeque::from(['F', 'E', 'D']))
                ]),
                vec![(1, 2, 1), (3, 1, 3)]
            )
        );
    }
}
