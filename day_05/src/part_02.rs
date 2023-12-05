use std::{io::Result, sync::mpsc, thread};

use crate::{Input, Integer};

pub fn main(input: &Input) -> Result<Integer> {
    let (tx, rx) = mpsc::channel::<_>();

    let input = input.clone();
    let seeds = input.seeds.to_owned();

    Ok(thread::scope(move |s| -> _ {
        let chunks = seeds.chunks(2);

        let to_check = chunks.fold(vec![], |mut chunks, chunk| {
            let (start, length) = (chunk[0], chunk[1]);
            chunks.extend(Vec::from_iter((start..(start + length)).into_iter()));
            chunks
        });

        let ranged_chunks = to_check.chunks(100);
        let ranged_chunks_max = ranged_chunks.len();
        let mut i = 0;

        for chunks in ranged_chunks {
            let chunks = chunks.to_owned();
            let input = input.to_owned();
            let handle = s.spawn(move || -> _ {
                let mut min = Integer::MAX;
                let value = chunks.clone();
                let input = input.to_owned();
                for seed in value {
                    let location = Input::get_location(&input, seed);
                    if location < min {
                        min = location;
                    }
                }
                min
            });

            let result = handle.join().unwrap();
            let _ = tx.send(result);

            i = i + 1;
            if i == ranged_chunks_max {
                let _ = tx.send(0); // I needed to break on something...
            }
        }

        let handle = s.spawn(move || -> _ {
            let mut min = Integer::MAX;
            while let Ok(min_from_message) = rx.recv() {
                if min_from_message == 0 {
                    break;
                }

                if min_from_message < min {
                    min = min_from_message;
                }
            }
            min
        });

        handle.join().unwrap()
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
