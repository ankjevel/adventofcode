use std::{io::Result, thread};

use crate::{Input, Integer};

pub fn main(input: &Input) -> Result<Integer> {
    let input = input.clone();
    let seeds = input.seeds.to_owned();

    Ok(thread::scope(move |s| -> _ {
        let chunks = seeds.chunks(2);

        let to_check = chunks.fold(vec![], |mut chunks, chunk| {
            let (start, length) = (chunk[0], chunk[1]);
            chunks.extend(Vec::from_iter((start..(start + length)).into_iter()));
            chunks
        });

        let ranged_chunks = to_check.chunks(100_000);
        let mut threads = vec![];

        for chunks in ranged_chunks {
            let chunks = chunks.to_owned();
            let input = input.to_owned();
            threads.push(s.spawn(move || -> _ {
                let mut min = Integer::MAX;
                let value = chunks.clone();
                let input = input.to_owned();
                for seed in value {
                    min = min.min(Input::get_location(&input, seed));
                }
                min
            }));
        }

        let threads = threads.into_iter().map(|t| t.join().unwrap());
        threads.min().unwrap()
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_input;

    const EXAMPLE_DATA: &'static str = include_str!("../../input/day_05_example");

    #[test]
    fn it_gets_the_example_correct() -> Result<()> {
        assert_eq!(main(&parse_input(&EXAMPLE_DATA))?, 46);
        Ok(())
    }
}
