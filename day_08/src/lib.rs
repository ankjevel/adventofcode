use std::collections::BTreeMap;

pub mod part_01;
pub mod part_02;

pub type Input = BTreeMap<(usize, usize), u32>;

pub fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(str::trim)
        .filter(|string| !string.is_empty())
        .enumerate()
        .map(|(y, string)| {
            string
                .chars()
                .enumerate()
                .map(|(x, c)| ((x, y), c.to_digit(10).unwrap_or(0)))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect()
}

pub fn max(input: &Input) -> (usize, usize) {
    input.into_iter().fold((0, 0), |max, ((x, y), _)| {
        (
            if x > &max.0 { x.to_owned() } else { max.0 },
            if y > &max.1 { y.to_owned() } else { max.1 },
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &'static str = "
        12
        34
    ";

    #[test]
    fn it_parses_example() {
        assert_eq!(
            parse_input(&EXAMPLE_DATA),
            BTreeMap::from([((0, 0), 1), ((1, 0), 2), ((0, 1), 3), ((1, 1), 4)])
        );
    }
}
