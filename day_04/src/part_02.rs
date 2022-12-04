use std::io::Result;

use crate::Input;

pub fn main(input: &Input) -> Result<u32> {
    Ok(input.into_iter().fold(0u32, |sum, (n0, n1, n2, n3)| {
        let r0 = n0..=n1;
        let r1 = n2..=n3;
        if r0.contains(&n2) || r0.contains(&n3) || r1.contains(&n0) || r1.contains(&n1) {
            sum + 1
        } else {
            sum
        }
    }))
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    use super::*;

    const EXAMPLE_DATA: &'static str = "
        2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8
    ";

    #[test]
    fn it_gets_the_example_correct() -> Result<()> {
        assert_eq!(main(&parse_input(&EXAMPLE_DATA))?, 4);
        Ok(())
    }
}
