use std::mem;
pub mod part_01;
pub mod part_02;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Input {
    pub numbers: Vec<u32>,
    pub boards: Vec<Vec<Vec<u32>>>,
}

pub fn parse_input(input: &str) -> Input {
    let mut lines = input.lines().map(str::trim).map(str::to_owned).into_iter();
    let mut numbers: Vec<_> = vec![];

    loop {
        let curr = lines.next();
        let unwrap = curr.expect("no value exists");

        if unwrap.is_empty() {
            continue;
        }

        let _old = mem::replace(
            &mut numbers,
            unwrap
                .split(&",")
                .map(|c| u32::from_str_radix(c, 10).unwrap())
                .collect::<Vec<_>>(),
        );

        break;
    }

    let mut boards = vec![];
    let mut board = vec![];
    lines.for_each(|v| {
        if v.trim().is_empty() {
            if !board.is_empty() {
                boards.push(board.to_owned());
                board.clear();
            }
            return;
        }

        board.push(
            v.split_whitespace()
                .map(|c| u32::from_str_radix(c, 10).unwrap())
                .collect::<Vec<_>>(),
        );
    });

    Input { numbers, boards }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &'static str = "
        7,4,9,5

        22 13 17 11  0
        8  2 23  4 24
    ";

    #[test]
    fn it_parses_example() {
        assert_eq!(
            parse_input(&EXAMPLE_DATA),
            Input {
                numbers: vec![7, 4, 9, 5],
                boards: vec![vec![vec![22, 13, 17, 11, 0], vec![8, 2, 23, 4, 24]]]
            }
        );
    }
}
