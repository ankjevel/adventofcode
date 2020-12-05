use std::io::Result;

use crate::{Square, Square::Tree};

pub fn main(input: &Vec<Vec<Square>>) -> Result<usize> {
    let max = input.get(0).unwrap().len();
    Ok(vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .into_iter()
        .fold(1, |count, (right, down)| {
            let mut index = right.to_owned();
            let trees = (down..input.len())
                .step_by(down)
                .map(|i| {
                    let square = input.get(i).unwrap().get(index).unwrap();
                    index = (index + right) % max;
                    square.to_owned()
                })
                .filter(|square| square == &Tree)
                .count();
            count * trees
        }))
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    use super::main;

    const EXAMPLE_DATA: &'static str = "
        ..##.......
        #...#...#..
        .#....#..#.
        ..#.#...#.#
        .#...##..#.
        ..#.##.....
        .#.#.#....#
        .#........#
        #.##...#...
        #...##....#
        .#..#...#.#
    ";

    #[test]
    fn it_gets_the_example_correct() {
        assert_eq!(main(&parse_input(EXAMPLE_DATA)).unwrap(), 336);
    }
}
