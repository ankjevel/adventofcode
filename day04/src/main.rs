#![feature(label_break_value)]

use std::io::Result;

mod part_01;

use part_01::main as part_01;

const PUZZLE_INPUT: &'static str = "347312-805915";

fn main() -> Result<()> {
    'part_01: {
        println!("part_01: {}", part_01(&PUZZLE_INPUT).unwrap());
    }

    'part_02: {}
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_gets_the_right_answer_for_examples() {
        assert_eq!(true, part_01::meet_criteria("111111"));
        assert_eq!(false, part_01::meet_criteria("223450"));
        assert_eq!(false, part_01::meet_criteria("123789"));
    }
}
