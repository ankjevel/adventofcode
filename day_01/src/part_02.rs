use std::io::Result;

use crate::Input;

pub fn main(input: &Input) -> Result<u32> {
    Ok(input
        .into_iter()
        .map(|row| {
            let digits: Vec<u32> = row
                .chars()
                .enumerate()
                .into_iter()
                .map(|(i, c)| {
                    if c.is_digit(10) {
                        return Some(c.to_digit(10).unwrap());
                    }
                    let chars = row.chars().skip(i).collect::<String>();
                    if chars.starts_with("one") {
                        return Some(1);
                    } else if chars.starts_with("two") {
                        return Some(2);
                    } else if chars.starts_with("three") {
                        return Some(3);
                    } else if chars.starts_with("four") {
                        return Some(4);
                    } else if chars.starts_with("five") {
                        return Some(5);
                    } else if chars.starts_with("six") {
                        return Some(6);
                    } else if chars.starts_with("seven") {
                        return Some(7);
                    } else if chars.starts_with("eight") {
                        return Some(8);
                    } else if chars.starts_with("nine") {
                        return Some(9);
                    } else {
                        return None;
                    };
                })
                .filter(|o| match o {
                    Some(_val) => true,
                    None => false,
                })
                .map(|val| val.unwrap())
                .collect::<Vec<u32>>();
            let first = digits.first().unwrap_or(&0).to_owned();
            let last = digits.last().unwrap_or(&0).to_owned();

            format!("{}{}", first, last).as_str().parse().unwrap_or(0)
        })
        .sum())
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    use super::*;

    const EXAMPLE_DATA: &'static str = "
        two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen
    ";

    #[test]
    fn it_gets_the_example_correct() -> Result<()> {
        assert_eq!(main(&parse_input(&EXAMPLE_DATA))?, 281);
        Ok(())
    }
}
