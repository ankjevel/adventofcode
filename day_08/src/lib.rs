#![feature(iter_advance_by)]
use instruction::Instruction;

pub mod instruction;
pub mod part_01;
pub mod part_02;

pub type Input = Vec<Instruction>;

pub fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(str::trim)
        .filter(|string| !string.is_empty())
        .map(|string| {
            let mut split = string.split(" ");
            let operation = split.next().unwrap_or("nop");
            let argument = split.next().unwrap_or("0").parse::<_>().unwrap_or(0);
            Instruction::new(operation, argument)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &'static str = "
        nop +0
        acc +1
        jmp +4
    ";

    #[test]
    fn it_parses_example() {
        assert_eq!(
            parse_input(&EXAMPLE_DATA),
            vec![
                Instruction::new("nop", 0),
                Instruction::new("acc", 1),
                Instruction::new("jmp", 4)
            ]
        );
    }
}
