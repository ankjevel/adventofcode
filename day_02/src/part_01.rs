use crate::Line;
use std::io::Result;

pub fn main(input: &Vec<Line>) -> Result<u16> {
    let mut count = 0;
    for (min, max, char_to_match, password_string) in input {
        let matches = password_string
            .chars()
            .into_iter()
            .filter(|c| c == char_to_match)
            .count() as u16;

        if &matches >= min && &matches <= max {
            count += 1;
        }
    }
    Ok(count)
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
    fn it_gets_the_example_correct() {
        assert_eq!(main(&parse_input(EXAMPLE_DATA)).unwrap(), 2)
    }
}
