use std::io::Result;

pub fn to_bytes(input: &str) -> Vec<u8> {
    input
        .chars()
        .map(|c| c.to_string().parse::<u8>().unwrap())
        .collect()
}

pub fn meet_criteria(input: &str) -> bool {
    let bytes = to_bytes(&input);
    let mut pairs = false;
    let mut never_decresed = true;
    for i in 1..bytes.len() {
        if !never_decresed {
            break;
        }
        let current = bytes[i];
        let prev = bytes[i - 1];
        never_decresed = current >= prev;
        pairs = pairs || current == prev;
    }
    pairs && never_decresed
}

pub fn main(input: &Vec<u32>) -> Result<u32> {
    Ok((input[0]..input[1])
        .collect::<Vec<u32>>()
        .iter()
        .filter(|i| meet_criteria(&i.to_string()))
        .count() as u32)
}
