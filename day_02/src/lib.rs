pub mod hand;
pub mod part_01;
pub mod part_02;

use hand::*;

pub type Input = Vec<(Hand, Hand)>;

pub fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(str::trim)
        .filter(|string| !string.is_empty())
        .map(|row| {
            (
                Hand::new(&row.chars().nth(0).unwrap()),
                Hand::new(&row.chars().nth(2).unwrap()),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    use Hand::*;

    const EXAMPLE_DATA: &'static str = "
        A X
        B Y
        C Z
    ";

    #[test]
    fn it_parses_example() {
        assert_eq!(
            parse_input(&EXAMPLE_DATA),
            vec![(Rock, Rock), (Paper, Paper), (Scissors, Scissors)]
        );
    }
}
