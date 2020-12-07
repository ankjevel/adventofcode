#[macro_use(lazy_static)]
extern crate lazy_static;
extern crate regex;

pub mod part_01;
pub mod part_02;

use std::collections::HashMap;

use regex::Regex;

lazy_static! {
    static ref RE: Regex =
        Regex::new(r"(\w+ \w+) bags contain(?: (\d) (\w+ \w+) bags?[\.,]){0,}").unwrap();
    static ref ITER: Regex = Regex::new(r"(?: (\d) (\w+ \w+) bags?[\.,])").unwrap();
}

pub type Input = HashMap<String, Vec<(u8, String)>>;

pub fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(str::trim)
        .filter(|string| !string.is_empty())
        .filter(|string| RE.is_match(&string))
        .map(|string| {
            let caps = RE.captures(&string).unwrap();
            let name = &caps.get(1).unwrap().as_str();
            (
                name.to_string(),
                ITER.captures_iter(string)
                    .filter(|group| group.get(1).is_some())
                    .map(|group| {
                        (
                            group.get(1).unwrap().as_str().parse::<u8>().unwrap_or(0),
                            group.get(2).unwrap().as_str().to_owned(),
                        )
                    })
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Input>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &'static str = "
        bright white bags contain 1 shiny gold bag.
        dim fuchsia bags contain 5 dark red bags, 1 muted green bag, 4 clear indigo bags.
        dotted black bags contain no other bags.
    ";

    #[test]
    fn it_parses_example() {
        let mut expected = HashMap::new();

        expected.insert(
            "bright white".to_string(),
            vec![(1, "shiny gold".to_string())],
        );

        expected.insert(
            "dim fuchsia".to_string(),
            vec![
                (5, "dark red".to_string()),
                (1, "muted green".to_string()),
                (4, "clear indigo".to_string()),
            ],
        );

        expected.insert("dotted black".to_string(), vec![]);

        assert_eq!(parse_input(&EXAMPLE_DATA), expected);
    }
}
