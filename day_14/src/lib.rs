pub mod part_01;
pub mod part_02;
pub mod program;

use program::Key::{self, *};

pub type Input = Vec<(Key, String)>;

pub fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(str::trim)
        .filter(|string| !string.is_empty())
        .map(|string| {
            let mut split = string.split(&" = ");
            let key = split.next().unwrap_or("");
            let value = split.next().unwrap_or("").to_string();
            (
                if key.starts_with(&"mem") {
                    let key = key.replace("]", "");
                    let (_, key) = key.split_at(4);
                    Mem(key.parse::<usize>().unwrap())
                } else {
                    Mask
                },
                value,
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &'static str = "
        mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
        mem[8] = 11
        mem[7] = 101
        mem[8] = 0
    ";

    #[test]
    fn it_parses_example() {
        assert_eq!(
            parse_input(&EXAMPLE_DATA),
            vec![
                (Mask, "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".to_string()),
                (Mem(8), "11".to_string()),
                (Mem(7), "101".to_string()),
                (Mem(8), "0".to_string())
            ]
        );
    }
}
