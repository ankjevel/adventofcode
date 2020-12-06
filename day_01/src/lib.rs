pub mod part_01;
pub mod part_02;

pub type Input = Vec<u32>;

pub fn parse_input(string: &str) -> Input {
    string
        .lines()
        .map(str::trim)
        .filter(|string| !string.is_empty())
        .into_iter()
        .map(|string| string.parse::<u32>().unwrap())
        .collect()
}
