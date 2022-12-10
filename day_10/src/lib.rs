use instruction::Instruction::{self, *};

pub mod cpu;
pub mod instruction;
pub mod part_01;
pub mod part_02;

pub type Input = Vec<Instruction>;

pub fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(str::trim)
        .filter(|string| !string.is_empty())
        .map(str::to_owned)
        .map(|row| {
            let part: Vec<_> = row.split(" ").into_iter().map(str::to_owned).collect::<_>();
            match &*part[0] {
                "addx" => AddX(part[1].parse::<_>().unwrap_or(0)),
                _ => Noop,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &'static str = "
        addx 1
        noop
        addx -1
    ";

    #[test]
    fn it_parses_example() {
        assert_eq!(parse_input(&EXAMPLE_DATA), vec![AddX(1), Noop, AddX(-1)]);
    }
}
