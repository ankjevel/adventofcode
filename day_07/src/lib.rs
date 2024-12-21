pub mod part_01;
pub mod part_02;

pub type Number = i64;
pub type Input = Vec<(Number, Vec<Number>)>;

pub fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(str::trim)
        .filter(|string| !string.is_empty())
        .map(str::to_owned)
        .map(|line| {
            let mut line = line.split(": ");
            (
                line.next().unwrap().parse().unwrap(),
                line.next()
                    .unwrap()
                    .split(' ')
                    .map(|part| part.parse().unwrap())
                    .collect(),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = "
        190: 10 19
        3267: 81 40 27
    ";

    #[test]
    fn it_parses_example() {
        assert_eq!(
            parse_input(EXAMPLE_DATA),
            vec![(190, vec![10, 19]), (3267, vec![81, 40, 27]),]
        );
    }
}
