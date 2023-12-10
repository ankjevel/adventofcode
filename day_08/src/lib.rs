use std::{cell::RefCell, collections::HashMap};

pub mod part_01;
pub mod part_02;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Direction {
    Left = 0,
    Right,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Input {
    moves: Vec<Direction>,
    nodes: RefCell<Vec<(String, (String, String))>>,
    indices: HashMap<String, usize>,
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
        nodes: RefCell::new(nodes),
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
                nodes: RefCell::new(vec![
                    ("AAA".to_owned(), ("BBB".to_owned(), "BBB".to_owned())),
                    ("BBB".to_owned(), ("AAA".to_owned(), "ZZZ".to_owned())),
                    ("ZZZ".to_owned(), ("ZZZ".to_owned(), "ZZZ".to_owned())),
                ]),
                indices: HashMap::from([
                    ("AAA".to_owned(), 0),
                    ("BBB".to_owned(), 1),
                    ("ZZZ".to_owned(), 2),
                ]),
            }
        );
    }
}
