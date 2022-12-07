pub mod part_01;
pub mod part_02;

pub type Input = Vec<String>;

pub fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(str::trim)
        .filter(|string| !string.is_empty())
        .map(str::to_owned)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &'static str = "
        example
    ";

    #[test]
    fn it_parses_example() {
        assert_eq!(parse_input(&EXAMPLE_DATA), vec!["example"]);
    }
}
