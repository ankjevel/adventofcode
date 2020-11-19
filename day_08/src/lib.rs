pub mod part_01;
pub mod part_02;

pub fn parse_input(string: &str) -> Vec<u32> {
    string
        .lines()
        .map(|string| string.trim())
        .filter(|string| !string.is_empty())
        .next()
        .unwrap()
        .split(' ')
        .map(|part| part.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
}
