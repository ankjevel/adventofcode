pub mod grid;
pub mod part_01;

pub fn parse_input(string: &str) -> u16 {
    string
        .lines()
        .map(|string| string.trim())
        .filter(|string| !string.is_empty())
        .next()
        .unwrap()
        .parse::<_>()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    const PUZZLE_INPUT: &'static str = include_str!("../../input/day_11");

    #[test]
    fn it_parses_the_puzzle_input() {
        assert_eq!(parse_input(&PUZZLE_INPUT), 1309)
    }
}
