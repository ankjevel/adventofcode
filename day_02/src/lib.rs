pub mod part_01;
pub mod part_02;

pub type Number = i128;
pub type Input = Vec<(Number, Number)>;

pub fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(str::trim)
        .filter(|string| !string.is_empty())
        .map(str::to_owned)
        .collect::<Vec<_>>()
        // just to handle examples
        .join("")
        .split(',')
        .map(|part| {
            let splits: Vec<&str> = part.split('-').collect();
            let left: _ = splits[0].to_owned().parse().unwrap();
            let right: _ = splits[1].to_owned().parse().unwrap();
            (left, right)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = "
        11-22,33-44,55-66,
        77-88,99-1010
    ";

    #[test]
    fn it_parses_example() {
        assert_eq!(
            parse_input(EXAMPLE_DATA),
            vec![(11, 22), (33, 44), (55, 66), (77, 88), (99, 1010)]
        );
    }
}
