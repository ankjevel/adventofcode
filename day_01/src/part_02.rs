use std::io::Result;

use crate::{Input, Number};

pub fn main((left, right): &Input) -> Result<Number> {
    Ok(left.iter().fold(0, |sum, left_value| {
        let occurrences = right
            .iter()
            .filter(|right_value| right_value == &left_value)
            .count() as Number;
        let similarity_score = left_value * occurrences;
        sum + similarity_score
    }))
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    use super::*;

    const EXAMPLE_DATA: &str = "
        3   4
        4   3
        2   5
        1   3
        3   9
        3   3
    ";

    #[test]
    fn it_gets_the_example_correct() -> Result<()> {
        assert_eq!(main(&parse_input(EXAMPLE_DATA))?, 31);
        Ok(())
    }
}
