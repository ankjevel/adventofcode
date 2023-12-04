use std::{collections::BTreeMap, io::Result};

use crate::Input;

pub fn main(input: &Input) -> Result<u128> {
    let mut cards = BTreeMap::from_iter(input.iter().enumerate().map(|n| (n.0, 1)));

    input
        .iter()
        .enumerate()
        .for_each(|(index, (winning_numbers, numbers))| {
            let to_add = cards.get(&(index)).unwrap_or(&1).to_owned();
            for (card, _) in winning_numbers
                .iter()
                .filter(|n| numbers.contains(n))
                .enumerate()
            {
                cards
                    .entry(card + index + 1)
                    .and_modify(|previous| *previous += to_add);
            }
        });

    Ok(cards.values().sum())
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    use super::*;

    const EXAMPLE_DATA: &'static str = "
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    ";

    #[test]
    fn it_gets_the_example_correct() -> Result<()> {
        assert_eq!(main(&parse_input(&EXAMPLE_DATA))?, 30);
        Ok(())
    }
}
