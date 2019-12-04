use std::io::Result;

use super::part_01::to_bytes;

pub fn meet_criteria(input: &str) -> bool {
    let bytes = to_bytes(&input);
    let mut ordered_bytes = to_bytes(&input);
    ordered_bytes.sort();

    if bytes != ordered_bytes {
        return false;
    }

    let mut repeats = vec![0; 10];
    for i in 0..bytes.len() {
        repeats[bytes[i] as usize] += 1;
    }

    &2 == repeats.iter().max().unwrap() || repeats.contains(&2)
}

pub fn main(input: &Vec<u32>) -> Result<u32> {
    Ok((input[0]..input[1])
        .collect::<Vec<u32>>()
        .iter()
        .filter(|i| meet_criteria(&i.to_string()))
        .count() as u32)
}
