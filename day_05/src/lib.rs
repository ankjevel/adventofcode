use std::ops::Range;

pub mod part_01;
pub mod part_02;

pub type Integer = u128;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Input {
    seeds: Vec<Integer>,
    maps: Vec<Vec<(Integer, Integer, Range<Integer>)>>,
}

impl<'a> Input {
    pub fn get_location(input: &'a Input, seed: Integer) -> Integer {
        let mut current_index = seed.to_owned();
        let mut iterator = input.maps.to_owned().into_iter();
        let mut result = 0;

        while let Some(next) = iterator.next() {
            let next_index = next.into_iter().find(
                |(_destination_range_start, _source_range_start, source_range)| {
                    source_range.contains(&current_index)
                },
            );

            current_index = match next_index {
                Some((destination_range_start, source_range_start, _source_range)) => {
                    destination_range_start + (current_index - source_range_start)
                }
                _ => current_index,
            };

            result = current_index.to_owned();
        }

        result
    }
}

pub fn parse_input(input: &str) -> Input {
    let mut last_key = "".to_owned();
    let mut current_array = vec![];
    let mut seeds = vec![];
    let mut maps = vec![];
    let map_identifier = " map:";

    let numbers = |key: &str| -> Vec<_> {
        key.split(' ')
            .filter(|string| !string.trim().is_empty())
            .map(|string| string.trim().parse::<_>().unwrap())
            .collect()
    };

    input
        .trim_start()
        .trim_end()
        .lines()
        .map(str::trim)
        .map(|string| string.to_owned())
        .for_each(|string| {
            if string.is_empty() {
                if last_key.is_empty() || current_array.is_empty() {
                    return;
                }
                maps.push(current_array.to_owned());
                current_array.clear();
                return;
            }

            let is_map_start = string.contains(map_identifier);
            let is_start = string.contains(": ") && !string.contains(map_identifier);

            if is_start {
                seeds = numbers(string.split(": ").collect::<Vec<_>>()[1]);
                return;
            }

            if is_map_start {
                current_array.clear();
                last_key = string.split(map_identifier).collect::<Vec<_>>()[0].to_owned();
                return;
            }

            let numbers = numbers(&string);
            let (destination_range_start, source_range_start, range_length) =
                (numbers[0], numbers[1], numbers[2]);
            let source_range = source_range_start..(source_range_start + range_length);
            current_array.push((destination_range_start, source_range_start, source_range));
        });

    if !current_array.is_empty() {
        maps.push(current_array.to_owned());
    }

    Input { seeds, maps }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &'static str = "
        seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48
        
        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15
    ";

    #[test]
    fn it_parses_input() {
        assert_eq!(
            parse_input(&EXAMPLE_DATA),
            Input {
                seeds: vec![79, 14, 55, 13],
                maps: vec![
                    vec![(50, 98, 98..100), (52, 50, 50..98)],
                    vec![(0, 15, 15..52), (37, 52, 52..54), (39, 0, 0..15)]
                ]
            }
        );
    }
}
