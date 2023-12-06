use std::io::Result;

use crate::Input;

pub fn main(input: &Input) -> Result<usize> {
    let (time, distance) = input.to_owned().into_iter().fold(
        ("".to_owned(), "".to_owned()),
        |mut n, (time, distance)| {
            n.0 = format!("{}{}", n.0, time);
            n.1 = format!("{}{}", n.1, distance);
            n
        },
    );

    let (time, distance) = (
        time.parse::<u128>().unwrap(),
        distance.parse::<u128>().unwrap(),
    );

    let n_times = (0..=time)
        .into_iter()
        .filter(|warmup| {
            let runtime = time - warmup;
            let total_time = runtime * warmup;
            total_time > distance
        })
        .collect::<Vec<_>>()
        .len();

    Ok(n_times)
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    use super::*;

    const EXAMPLE_DATA: &'static str = include_str!("../../input/day_06_example");

    #[test]
    fn it_gets_the_example_correct() -> Result<()> {
        assert_eq!(main(&parse_input(&EXAMPLE_DATA))?, 71503);
        Ok(())
    }
}
