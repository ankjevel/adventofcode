use std::collections::HashMap;

pub mod math;
pub mod part_01;
pub mod part_02;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Direction {
    Left = 0,
    Right,
}

pub type Integer = u64;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Input {
    moves: Vec<Direction>,
    nodes: Vec<(String, (String, String))>,
    indices: HashMap<String, usize>,
}

impl Input {
    pub fn solve(input: &Input, start_index: usize) -> Integer {
        let nodes = input.nodes.to_owned();
        let mut current = nodes.get(start_index).unwrap().to_owned();

        let mut steps = 0;
        'main: loop {
            for direction in input.moves.clone() {
                steps += 1;

                let next_index = match direction {
                    Direction::Left => &*current.1 .0,
                    Direction::Right => &*current.1 .1,
                };

                let next_index = *input.indices.get(next_index).unwrap();
                let next = nodes.get(next_index).unwrap().to_owned();

                if next.0.ends_with('Z') {
                    break 'main;
                }

                current = next;
            }
        }

        steps
    }
}

pub fn parse_input(input: &str) -> Input {
    let rows = input
        .trim_start()
        .trim_end()
        .lines()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| s.to_owned())
        .collect::<Vec<_>>();

    let header = rows.get(0).unwrap();

    let moves = header
        .chars()
        .map(|c| match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("wrong direction?! {c}"),
        })
        .collect::<Vec<_>>();

    let nodes = rows
        .into_iter()
        .skip(1)
        .map(|row| {
            let split = row.split('=').collect::<Vec<_>>();
            let index = split[0].trim().to_owned();
            let nodes = split[1].trim().to_owned();
            let nodes = nodes.replace('(', "");
            let nodes = nodes.replace(')', "");

            let nodes = nodes.split(", ").collect::<Vec<_>>();
            let left = nodes[0].to_owned();
            let right = nodes[1].to_owned();
            (index, (left, right))
        })
        .collect::<Vec<_>>();

    let indices = HashMap::from_iter(
        nodes
            .clone()
            .into_iter()
            .enumerate()
            .map(|(index, n)| (n.0.to_owned(), index)),
    );

    Input {
        moves,
        nodes,
        indices,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use Direction::*;

    const EXAMPLE_DATA: &'static str = "
    LLR

    AAA = (BBB, BBB)
    BBB = (AAA, ZZZ)
    ZZZ = (ZZZ, ZZZ)
    ";

    #[test]
    fn it_parses_input() {
        assert_eq!(
            parse_input(&EXAMPLE_DATA),
            Input {
                moves: vec![Left, Left, Right],
                nodes: vec![
                    ("AAA".to_owned(), ("BBB".to_owned(), "BBB".to_owned())),
                    ("BBB".to_owned(), ("AAA".to_owned(), "ZZZ".to_owned())),
                    ("ZZZ".to_owned(), ("ZZZ".to_owned(), "ZZZ".to_owned())),
                ],
                indices: HashMap::from([
                    ("AAA".to_owned(), 0),
                    ("BBB".to_owned(), 1),
                    ("ZZZ".to_owned(), 2),
                ]),
            }
        );
    }
}
