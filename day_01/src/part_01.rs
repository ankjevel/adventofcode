use std::io::Result;

use crate::Input;

pub fn main(input: &Input) -> Result<u32> {
    Ok(input
        .into_iter()
        .map(|row| {
            let digits: Vec<u32> = row
                .chars()
                .into_iter()
                .map(|c| {
                    if c.is_digit(10) {
                        return Some(c.to_digit(10).unwrap());
                    } else {
                        return None;
                    }
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
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
    ";

    #[test]
    fn it_gets_the_example_correct() -> Result<()> {
        assert_eq!(main(&parse_input(&EXAMPLE_DATA))?, 142);
        Ok(())
    }
}
