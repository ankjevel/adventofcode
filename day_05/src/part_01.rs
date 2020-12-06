use std::io::Result;

use crate::Input;

pub fn main(input: &Input) -> Result<u32> {
    Ok(input
        .iter()
        .map(|string| {
            let chars = string.chars().collect::<Vec<_>>();
            let row = &chars[0..7]
                .into_iter()
                .fold(0..=127, |range, letter| {
                    let min = range.to_owned().min().unwrap();
                    let max = range.to_owned().max().unwrap();
                    let range = (max - min) / 2;
                    match letter.to_owned() {
                        'F' => min..=min + range,
                        'B' => max - range..=max,
                        _ => min..=max,
                    }
                })
                .min()
                .unwrap()
                .to_owned();
            let column = &chars[7..]
                .into_iter()
                .fold(0..=7, |range, letter| {
                    let min = range.to_owned().min().unwrap();
                    let max = range.to_owned().max().unwrap();
                    let range = (max - min) / 2;
                    match letter.to_owned() {
                        'L' => min..=min + range,
                        'R' => max - range..=max,
                        _ => min..=max,
                    }
                })
                .min()
                .unwrap()
                .to_owned();
            (row * 8) + column
        })
        .max()
        .unwrap())
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    use super::*;

    const EXAMPLE_DATA: &'static str = "
        BFFFBBFRRR
        FFFBBBFRRR
        BBFFBBFRLL
    ";

    #[test]
    fn it_gets_the_first_example_correct() -> Result<()> {
        assert_eq!(main(&vec!["FBFBBFFRLR".to_string()])?, 357);
        Ok(())
    }

    #[test]
    fn it_gets_the_example_correct() -> Result<()> {
        assert_eq!(main(&parse_input(&EXAMPLE_DATA))?, 820);
        Ok(())
    }
}
