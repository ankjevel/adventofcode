pub mod part_01;
pub mod part_02;

pub type Input = Vec<Option<u32>>;

pub fn parse_input(input: &str) -> Input {
    input
        .trim_start()
        .trim_end()
        .lines()
        .map(str::trim)
        .map(|string| {
            if string.is_empty() {
                None
            } else {
                Some(string.parse().unwrap())
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &'static str = "
        1000

        2000
        
    ";

    #[test]
    fn it_parses_example() {
        assert_eq!(
            parse_input(&EXAMPLE_DATA),
            vec![Some(1000), None, Some(2000)]
        );
    }
}
