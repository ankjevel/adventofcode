pub mod part_01;
pub mod part_02;

type Number = u64;
pub type Input = Vec<Vec<Number>>;

pub fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(str::trim)
        .filter(|string| !string.is_empty())
        .map(|s| {
            s.chars()
                .into_iter()
                .map(|c| c.to_string().parse().unwrap())
                .collect::<Vec<_>>()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = "
        123
        456
    ";

    #[test]
    fn it_parses_example() {
        assert_eq!(
            parse_input(EXAMPLE_DATA),
            vec![vec![1, 2, 3], vec![4, 5, 6]]
        );
    }
}
