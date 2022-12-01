use std::io::Result;

use crate::Input;

pub fn main(input: &Input) -> Result<u32> {
    let mut max = vec![0, 0, 0];
    let mut current = 0;

    let mut check = |current: &u32| {
        for (i, n) in max.iter().enumerate() {
            if current < n {
                continue;
            }

            let mut result = vec![];
            let split = max.split_at(i);

            result.extend(split.0.to_owned());
            result.extend(vec![current]);
            result.extend(split.1.to_owned());

            max = result[0..=2].to_owned();

            break;
        }
    };

    for &val in input {
        match val {
            Some(num) => {
                current += num;
            }
            None => {
                check(&current);
                current = 0;
            }
        }
    }
    check(&current);

    Ok(max.into_iter().reduce(|a, b| a + b).unwrap_or(0))
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
        assert_eq!(main(&parse_input(&EXAMPLE_DATA))?, 45000);
        Ok(())
    }
}
