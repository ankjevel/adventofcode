use std::ops::RangeInclusive;

pub mod part_01;
pub mod part_02;

pub type Number = usize;
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Input {
    ranges: Vec<RangeInclusive<Number>>,
    ingredients: Vec<Number>,
}

pub fn parse_input(input: &str) -> Input {
    let mut ranges = vec![];
    let mut ingredients = vec![];

    input
        .lines()
        .map(str::trim)
        .filter(|string| !string.is_empty())
        .map(str::to_owned)
        .for_each(|line| {
            if line.contains("-") {
                let arr = line.split("-").collect::<Vec<_>>();
                ranges.push(arr[0].parse().unwrap()..=arr[1].parse().unwrap());
            } else {
                ingredients.push(line.parse().unwrap());
            }
        });

    Input {
        ranges,
        ingredients,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = "
        1-2
        3-4

        5
        1
        2
    ";

    #[test]
    fn it_parses_example() {
        assert_eq!(
            parse_input(EXAMPLE_DATA),
            Input {
                ranges: vec![1..=2, 3..=4],
                ingredients: vec![5, 1, 2]
            }
        );
    }
}
