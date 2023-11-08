pub mod part_01;
pub mod part_02;

pub type Number = i128;
pub type Input = (Vec<Number>, Vec<Number>);

pub fn parse_input(input: &str) -> Input {
    input.trim_start().trim_end().lines().map(str::trim).fold(
        (vec![], vec![]),
        |mut values, line| {
            let distances: Vec<_> = line
                .split_whitespace()
                .map(|string| string.parse().unwrap())
                .collect();

            values.0.push(distances[0]);
            values.1.push(distances[1]);

            values
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = "
        3   4
        4   3
        2   5
        1   3
        3   9
        3   3
    ";

    #[test]
    fn it_parses_example() {
        assert_eq!(
            parse_input(EXAMPLE_DATA),
            (vec![3, 4, 2, 1, 3, 3], vec![4, 3, 5, 3, 9, 3],)
        );
    }
}
