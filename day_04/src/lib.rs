#[macro_use(lazy_static)]
extern crate lazy_static;
extern crate regex;

pub mod part_01;
pub mod part_02;

use regex::Regex;

lazy_static! {
    static ref ROW: Regex = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
}

pub type Input = Vec<(u32, u32, u32, u32)>;

pub fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(str::trim)
        .filter(|string| !string.is_empty())
        .map(|string| {
            let unwrapped = ROW
                .captures(&string)
                .unwrap()
                .iter()
                .map(|c| c.unwrap().to_owned().as_str().parse::<u32>().unwrap_or(0))
                .collect::<Vec<u32>>();
            (
                unwrapped[1].to_owned(),
                unwrapped[2].to_owned(),
                unwrapped[3].to_owned(),
                unwrapped[4].to_owned(),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &'static str = "
        2-4,6-8
        2-3,4-5
        2-93,2-9
        3-82,82-8
        2-91,38-91
        2-90,91-96
    ";

    #[test]
    fn it_parses_example() {
        assert_eq!(
            parse_input(&EXAMPLE_DATA),
            vec![
                (2, 4, 6, 8),
                (2, 3, 4, 5),
                (2, 93, 2, 9),
                (3, 82, 82, 8),
                (2, 91, 38, 91),
                (2, 90, 91, 96)
            ]
        );
    }
}
