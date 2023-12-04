pub mod part_01;
pub mod part_02;

pub type Input = Vec<String>;

pub fn parse_input(input: &str) -> Input {
    input
        .trim_start()
        .trim_end()
        .lines()
        .map(str::trim)
        .map(|string| string.parse().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &'static str = "
    1abc2
    pqr3stu8vwx
    ";

    #[test]
    fn it_parses_example() {
        assert_eq!(parse_input(&EXAMPLE_DATA), vec!["1abc2", "pqr3stu8vwx"]);
    }
}
