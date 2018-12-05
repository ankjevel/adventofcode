use part_01;
use std::io;

pub fn main(units: &Vec<char>) -> io::Result<u32> {
    let chars = part_01::main(&units).unwrap();

    Ok((65..=90)
        .map(|c: u8| c as char)
        .map(|c| {
            let filtered: Vec<char> = chars
                .iter()
                .cloned()
                .filter(|a| a.to_ascii_uppercase() != c)
                .collect();
            part_01::main(&filtered).unwrap()
        })
        .map(|c| c.len())
        .min()
        .unwrap() as u32)
}
