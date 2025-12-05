use std::io::Result;
use crate::Input;

pub fn main(input: &Input) -> Result<usize> {
    let mut ranges: Vec<_> = vec![];
    let mut iter = input.ranges.clone();

    iter.sort_by(|a, b| a.start().cmp(b.start()));

    let mut iter = iter.into_iter();
    let first = iter.next().unwrap();
    let mut current_range = (*first.start(), *first.end());

    while iter.len() > 0 {
        let next = iter.next().unwrap().to_owned();
        let (start, end) = (*next.start(), *next.end());

        let range = current_range.0..=current_range.1;
        if range.contains(&start) && range.contains(&end) {
            continue;
        } else if !range.contains(&start) && !range.contains(&end) {
            ranges.push(current_range);
            current_range = (start, end);
        } else if !range.contains(&start) {
            current_range.0 = start;
        } else if !range.contains(&end) {
            current_range.1 = end;
        }
    }

    if !ranges.last().eq(&Some(&current_range)) {
        ranges.push(current_range);
    }
    
    Ok(ranges.iter().map(|(start, end)| end - start + 1).sum())
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    use super::*;

    const EXAMPLE_DATA: &str = include_str!("../../input/day_05_example");

    #[test]
    fn it_gets_the_example_correct() -> Result<()> {
        assert_eq!(main(&parse_input(EXAMPLE_DATA))?, 14);
        Ok(())
    }
}

