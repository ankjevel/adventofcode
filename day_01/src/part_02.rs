use std::io::Result;

pub fn main(input: &Vec<u32>) -> Result<u32> {
    let mut entries = (0, 0, 0);
    'outer: for a in input {
        for b in input.into_iter() {
            for c in input.into_iter() {
                if a + b + c == 2020 {
                    entries = (a.to_owned(), b.to_owned(), c.to_owned());
                    break 'outer;
                }
            }
        }
    }

    Ok(entries.0 * entries.1 * entries.2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_input;

    const EXAMPLE_DATA: &'static str = "
        1721
        979
        366
        299
        675
        1456
    ";

    #[test]
    fn it_gets_the_example_correct() {
        assert_eq!(main(&parse_input(EXAMPLE_DATA)).unwrap(), 241861950)
    }
}
