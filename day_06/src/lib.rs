use std::collections::VecDeque;

pub mod part_01;
pub mod part_02;

pub type Input = VecDeque<char>;

pub fn parse_input(input: &str) -> Input {
    input.trim().chars().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &'static str = "
        example
    ";

    #[test]
    fn it_parses_example() {
        assert_eq!(
            parse_input(&EXAMPLE_DATA),
            VecDeque::from(['e', 'x', 'a', 'm', 'p', 'l', 'e'])
        );
    }
}
