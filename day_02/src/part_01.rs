use std::io::Result;

use crate::Input;

pub fn main(input: &Input) -> Result<usize> {
    Ok(input
        .into_iter()
        .filter(|(min, max, char_to_match, password_string)| {
            let matches = password_string
                .chars()
                .into_iter()
                .filter(|c| c == char_to_match)
                .count() as u16;
            &matches >= min && &matches <= max
        })
        .count())
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    use super::*;

    const EXAMPLE_DATA: &'static str = "
        1-3 a: abcde
        1-3 b: cdefg
        2-9 c: ccccccccc
    ";

    #[test]
    fn it_gets_the_example_correct() -> Result<()> {
        assert_eq!(main(&parse_input(EXAMPLE_DATA))?, 2);
        Ok(())
    }
}
