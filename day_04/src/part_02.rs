use std::io;
use std::iter::Iterator;

use guards::guards;
use record::Record;

pub fn main(records: &Vec<Record>) -> io::Result<u32> {
    let mut guards = guards(&records).unwrap();

    guards.sort_unstable_by(|a, b| {
        let a_max = a.minutes_slept.iter().max().unwrap();
        let b_max = b.minutes_slept.iter().max().unwrap();
        a_max.cmp(b_max)
    });

    let selected_guard = guards.last().unwrap();

    let (minute, _) = selected_guard
        .minutes_slept
        .iter()
        .enumerate()
        .max_by_key(|(_i, minute)| *minute)
        .unwrap();

    Ok(selected_guard.id * minute as u32)
}
