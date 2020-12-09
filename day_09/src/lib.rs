pub mod part_01;
pub mod part_02;

pub type Input = Vec<u64>;

pub fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(str::trim)
        .filter(|string| !string.is_empty())
        .map(|string| string.parse::<_>().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &'static str = "
        35
        20
        15
    ";

    #[test]
    fn it_parses_example() {
        assert_eq!(parse_input(&EXAMPLE_DATA), vec![35, 20, 15]);
    }
}
