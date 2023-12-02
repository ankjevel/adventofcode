use std::io::Result;

use crate::{Bag, Input};

pub fn main(input: &Input) -> Result<u32> {
    Ok(input
        .iter()
        .map(|games| {
            games.iter().fold(Bag::new(), |mut bag, game| {
                bag.red = bag.red.max(game.red);
                bag.green = bag.green.max(game.green);
                bag.blue = bag.blue.max(game.blue);
                bag
            })
        })
        .map(|bag| bag.red * bag.green * bag.blue)
        .sum())
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    use super::*;

    const EXAMPLE_DATA: &'static str = "
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    ";

    #[test]
    fn it_gets_the_example_correct() -> Result<()> {
        assert_eq!(main(&parse_input(&EXAMPLE_DATA))?, 2286);
        Ok(())
    }
}
