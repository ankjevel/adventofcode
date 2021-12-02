use std::io::Result;

use crate::{Command::*, Input};

pub fn main(input: &Input) -> Result<i32> {
    let (mut x, mut y) = (0, 0);
    for command in input {
        match command {
            Forward(units) => x += units,
            Up(units) => y -= units,
            Down(units) => y += units,
        }
    }
    Ok(y * x)
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    use super::*;

    const EXAMPLE_DATA: &'static str = "
        forward 5
        down 5
        forward 8
        up 3
        down 8
        forward 2
    ";

    #[test]
    fn it_gets_the_example_correct() -> Result<()> {
        assert_eq!(main(&parse_input(&EXAMPLE_DATA))?, 150);
        Ok(())
    }
}
