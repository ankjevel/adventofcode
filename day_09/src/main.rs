extern crate day_05;

mod program;

use day_05::parse_input;

use program::exec;

fn main() {
    let input = parse_input(include_str!("../../input/day_09"));
    println!("part_01, {:?}", exec(&input[0], Some(vec![1])));
    println!("part_02, {:?}", exec(&input[0], Some(vec![2])));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLES_ON_PART_01: &'static str = "
        1102,34915192,34915192,7,4,7,99,0
        104,1125899906842624,99
    ";

    #[test]
    #[ignore]
    fn it_gets_the_examples_right_on_part_01() {
        let input = parse_input(&EXAMPLES_ON_PART_01);

        assert_eq!(exec(&input[1], None), 1219070632396864);
        assert_eq!(exec(&input[2], None), 1125899906842624);
    }

    #[test]
    fn quine() {
        let input = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let mut program = program::Program::new(&input);
        program.input = vec![];
        assert_eq!(program.run(), 99);
        assert_eq!(program.output, input.clone());
    }
}
