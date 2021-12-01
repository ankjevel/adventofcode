pub mod part_01;
pub mod part_02;

pub type Input = Vec<u32>;

pub fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(str::trim)
        .filter(|string| !string.is_empty())
        .map(|src| u32::from_str_radix(src, 10).unwrap_or(0))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &'static str = "
        199
        200
        208
    ";

    #[test]
    fn it_parses_example() {
        assert_eq!(parse_input(&EXAMPLE_DATA), vec![199, 200, 208]);
    }
}
