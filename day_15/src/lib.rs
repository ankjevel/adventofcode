pub mod memory_game;
pub mod part_01;
pub mod part_02;

pub type Input = Vec<u64>;

pub fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(str::trim)
        .filter(|string| !string.is_empty())
        .map(|string| {
            string
                .split(",")
                .map(|n| n.parse::<_>().unwrap_or(0))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &'static str = "
        0,3,6
    ";

    #[test]
    fn it_parses_example() {
        assert_eq!(parse_input(&EXAMPLE_DATA), vec![0, 3, 6]);
    }
}
