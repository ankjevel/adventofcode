use std::{collections::HashMap, io::Result};

use crate::Input;

type Bags = HashMap<String, Vec<String>>;

fn look_inside(name: &str, bags: &Bags) -> bool {
    if name == "shiny gold" {
        return true;
    }

    match &bags.get(name) {
        Some(inside) => {
            if inside.contains(&"shiny gold".to_string()) {
                return true;
            }
            inside.iter().filter(|name| look_inside(name, bags)).count() > 0
        }
        _ => false,
    }
}

pub fn main(input: &Input) -> Result<usize> {
    let bags: Bags = input
        .iter()
        .filter(|(_, inside)| !inside.is_empty())
        .map(|(name, inside)| {
            (
                name.to_string(),
                inside.iter().map(|(_, name)| name.to_string()).collect(),
            )
        })
        .collect();

    Ok(bags
        .clone()
        .into_iter()
        .filter(|(name, _)| name != &"shiny gold")
        .filter(|(_, inside)| {
            inside
                .iter()
                .filter(|name| look_inside(&name, &bags))
                .count()
                > 0
        })
        .count())
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    use super::*;

    const EXAMPLE_DATA: &'static str = "
        light red bags contain 1 bright white bag, 2 muted yellow bags.
        dark orange bags contain 3 bright white bags, 4 muted yellow bags.
        bright white bags contain 1 shiny gold bag.
        muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
        shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
        dark olive bags contain 3 faded blue bags, 4 dotted black bags.
        vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
        faded blue bags contain no other bags.
        dotted black bags contain no other bags.
    ";

    #[test]
    fn it_gets_the_example_correct() -> Result<()> {
        assert_eq!(main(&parse_input(&EXAMPLE_DATA))?, 4);
        Ok(())
    }
}
