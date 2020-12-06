use std::io::Result;

use crate::Input;

pub fn main(input: &Input) -> Result<usize> {
    Ok(input.iter().fold(0, |sum, group| {
        sum + ('a'..='z')
            .map(char::from)
            .filter(|c| {
                group
                    .iter()
                    .filter(|p| p.contains(&c.to_string()))
                    .collect::<Vec<_>>()
                    .len()
                    > 0
            })
            .collect::<Vec<char>>()
            .len()
    }))
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    use super::*;

    const EXAMPLE_DATA: &'static str = "
        abc

        a
        b
        c
        
        ab
        ac
        
        a
        a
        a
        a
        
        b
    ";

    #[test]
    fn it_gets_the_example_correct() -> Result<()> {
        assert_eq!(main(&parse_input(&EXAMPLE_DATA))?, 21);
        Ok(())
    }
}
