use std::io::Result;

use crate::Input;

pub fn main(input: &Input) -> Result<usize> {
    Ok(input
        .into_iter()
        .filter(
            |(first_index, second_index, char_to_match, password_string)| {
                let chars: Vec<char> = password_string.chars().collect();
                let match_count = vec![first_index, second_index]
                    .into_iter()
                    .map(|index| index.to_owned() as usize - 1)
                    .filter(|index| match chars.get(index.to_owned()) {
                        Some(char) if char == char_to_match => true,
                        _ => false,
                    })
                    .count();
                match_count == 1
            },
        )
        .count())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_input;

    const EXAMPLE_DATA: &'static str = "
        1-3 a: abcde
        1-3 b: cdefg
        2-9 c: ccccccccc
    ";

    #[test]
    fn it_gets_the_example_correct() -> Result<()> {
        assert_eq!(main(&parse_input(EXAMPLE_DATA))?, 1);
        Ok(())
    }
}
