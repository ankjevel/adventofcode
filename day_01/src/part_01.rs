use std::io::Result;

use crate::Input;

pub fn main(input: &Input) -> Result<u32> {
    let mut max = 0;
    let mut current = 0;
    for &val in input {
        match val {
            None => {
                if current > max {
                    max = current;
                }
                current = 0;
            }
            Some(num) => {
                current += num;
            }
        }
    }
    Ok(max)
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    use super::*;

    const EXAMPLE_DATA: &'static str = "
        1000
        2000
        3000
        
        4000
        
        5000
        6000
        
        7000
        8000
        9000
        
        10000
    ";

    #[test]
    fn it_gets_the_example_correct() -> Result<()> {
        assert_eq!(main(&parse_input(&EXAMPLE_DATA))?, 24000);
        Ok(())
    }
}
