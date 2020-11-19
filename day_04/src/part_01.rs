use std::io;
use std::iter::Iterator;

use guards::guards;
use record::Record;

pub fn main(records: &Vec<Record>) -> io::Result<u32> {
    let mut guards = guards(&records).unwrap();

    guards.sort_by_key(|guard| guard.sleep);

    let selected_guard = guards.last().unwrap();

    let (minute, _) = selected_guard
        .minutes_slept
        .into_iter()
        .enumerate()
        .max_by_key(|(_i, minute)| *minute)
        .unwrap();

    Ok(selected_guard.id * minute as u32)
}
