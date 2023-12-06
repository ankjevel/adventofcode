pub mod part_01;
pub mod part_02;

pub type Integer = u128;

pub type Input = Vec<(Integer, Integer)>;

pub fn parse_input(input: &str) -> Input {
    let parse_numbers = |s: &str| -> Vec<Integer> {
        s.split(':').collect::<Vec<_>>()[1]
            .trim()
            .to_owned()
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|string| string.parse().unwrap())
            .collect()
    };

    let lines = input
        .trim_start()
        .trim_end()
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();

    let (times, distances) = (
        parse_numbers(lines.get(0).unwrap()),
        parse_numbers(lines.get(1).unwrap()),
    );

    times.into_iter().zip(distances.into_iter()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &'static str = include_str!("../../input/day_06_example");

    #[test]
    fn it_parses_input() {
        assert_eq!(
            parse_input(&EXAMPLE_DATA),
            vec![(7, 9), (15, 40), (30, 200)]
        );
    }
}
