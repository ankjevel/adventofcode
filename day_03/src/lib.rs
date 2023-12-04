use std::ops::RangeInclusive;

pub mod part_01;
pub mod part_02;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Input {
    rows: Vec<Row>,
    max_x: usize,
    max_y: usize,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Row {
    numbers: Vec<(RangeInclusive<usize>, u32)>,
    symbols: Vec<(usize, char)>,
}

fn parse(input: Vec<String>) -> Input {
    let max_x = input.first().unwrap().len();
    let max_y = input.len();

    Input {
        max_x,
        max_y,
        rows: input
            .to_owned()
            .iter()
            .map(|line| {
                let numbers: Vec<_> = line
                    .chars()
                    .into_iter()
                    .enumerate()
                    .fold(vec![], |mut indices_and_chars, (index, c)| {
                        if !c.is_digit(10) {
                            return indices_and_chars;
                        }

                        let current = (index.to_owned(), c.to_owned());
                        let last = indices_and_chars.last_mut();

                        if last.is_none() {
                            indices_and_chars.push(vec![current]);
                            return indices_and_chars;
                        }

                        let last_mut = last.unwrap();
                        if last_mut.last().unwrap().0 == index - 1 {
                            last_mut.push(current);
                        } else {
                            indices_and_chars.push(vec![current]);
                        }

                        indices_and_chars
                    })
                    .iter()
                    .map(|v| {
                        (
                            v.first().unwrap().0..=v.last().unwrap().0,
                            v.iter()
                                .fold("".to_owned(), |str, (_, c)| {
                                    String::from(str.to_owned() + &c.to_string())
                                })
                                .parse()
                                .unwrap(),
                        )
                    })
                    .collect();

                let symbols =
                    line.chars()
                        .into_iter()
                        .enumerate()
                        .fold(vec![], |mut symbols, (index, c)| {
                            if !c.is_digit(10) && c != '.' {
                                symbols.push((index, c.to_owned()))
                            }
                            symbols
                        });

                Row { symbols, numbers }
            })
            .collect(),
    }
}

pub fn parse_input(input: &str) -> Input {
    parse(
        input
            .trim_start()
            .trim_end()
            .lines()
            .map(str::trim)
            .map(|string| string.parse().unwrap())
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &'static str = "
        467..114..
        ...#.+.58.
        ...$.*....
        .664.598..
    ";

    #[test]
    fn it_parses_example() {
        assert_eq!(
            parse_input(&EXAMPLE_DATA),
            Input {
                rows: vec![
                    Row {
                        numbers: vec![(0..=2, 467), (5..=7, 114)],
                        symbols: vec![]
                    },
                    Row {
                        numbers: vec![(7..=8, 58)],
                        symbols: vec![(3, '#'), (5, '+')]
                    },
                    Row {
                        numbers: vec![],
                        symbols: vec![(3, '$'), (5, '*')]
                    },
                    Row {
                        numbers: vec![(1..=3, 664), (5..=7, 598)],
                        symbols: vec![]
                    }
                ],
                max_x: 10,
                max_y: 4,
            }
        );
    }
}
