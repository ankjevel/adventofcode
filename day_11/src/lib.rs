pub mod monkey;
pub mod part_01;
pub mod part_02;

use std::collections::BTreeMap;

use monkey::{Monkey, Operation::*};

pub type Input = BTreeMap<isize, Monkey>;

pub fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(str::trim)
        .filter(|string| !string.is_empty())
        .map(str::to_owned)
        .collect::<Vec<_>>()
        .chunks(6)
        .map(|rows| {
            let mut rows = rows.clone().to_owned();
            let mut rows = rows.iter_mut();

            let index = rows
                .next()
                .unwrap()
                .chars()
                .nth(7)
                .unwrap()
                .to_digit(10)
                .unwrap() as isize;

            let splits: Vec<_> = rows
                .into_iter()
                .map(|row| row.split(": ").nth(1).unwrap())
                .collect::<Vec<_>>();

            (
                index,
                Monkey::new(
                    splits[0]
                        .split(", ")
                        .map(|part| part.parse::<_>().unwrap())
                        .collect::<Vec<_>>(),
                    {
                        let (_, operation) = splits[1].split_at(10);
                        let (operation, value) = operation.split_at(2);
                        let value = if value.contains("old") {
                            core::option::Option::None
                        } else {
                            Some(value.parse::<_>().unwrap())
                        };
                        if operation.contains("*") {
                            Mul(value)
                        } else {
                            Add(value)
                        }
                    },
                    {
                        let (_, value) = splits[2].split_at(13);
                        value.parse::<_>().unwrap()
                    },
                    {
                        let (_, value) = splits[3].split_at(16);
                        value.parse().unwrap()
                    },
                    {
                        let (_, value) = splits[4].split_at(16);
                        value.parse().unwrap()
                    },
                ),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &'static str = "
        Monkey 0:
          Starting items: 79, 98
          Operation: new = old * 19
          Test: divisible by 23
            If true: throw to monkey 2
            If false: throw to monkey 3
            
        Monkey 1:
          Starting items: 54, 65, 75, 74
          Operation: new = old + 6
          Test: divisible by 19
            If true: throw to monkey 2
            If false: throw to monkey 0
    ";

    #[test]
    fn it_parses_example() {
        assert_eq!(
            parse_input(&EXAMPLE_DATA),
            BTreeMap::from_iter(vec![
                (0, Monkey::new(vec![79, 98], Mul(Some(19)), 23, 2, 3)),
                (1, Monkey::new(vec![54, 65, 75, 74], Add(Some(6)), 19, 2, 0)),
            ])
        );
    }
}
