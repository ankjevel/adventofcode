use ::day_05::{parse_input, program::Program};
use std::io::Result;

#[allow(dead_code)]
fn main() -> Result<()> {
    let input = parse_input(include_str!("../../../input/day_05"));
    let mut program = Program::new(&input[0]);

    program.new_input(&vec![1]);
    program.run();
    println!("part_01: {:?}", program.output);

    program.new_input(&vec![5]);
    program.run();
    println!("part_02: {:?}", program.output);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA_FROM_DAY_02: &'static str = "
        1,0,0,0,99
        2,3,0,3,99
        2,4,4,5,99,0
        1,1,1,4,99,5,6,0,99
    ";

    const EXAMPLE_DATA_01: &'static str = "
        1101,100,-1,4,0
        1002,4,3,4,33
    ";

    #[test]
    fn it_still_works_with_example_data_from_day_02() {
        let input = parse_input(EXAMPLE_DATA_FROM_DAY_02);

        let results = input
            .iter()
            .map(|row| {
                let mut program = Program::new(&row);
                program.run();

                program.memory
            })
            .collect::<Vec<Vec<i32>>>();

        assert_eq!(results[0], [2, 0, 0, 0, 99]);
        assert_eq!(results[1], [2, 3, 0, 6, 99]);
        assert_eq!(results[2], [2, 4, 4, 5, 99, 9801]);
        assert_eq!(results[3], [30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    #[test]
    fn examples_for_part_1() {
        let input = parse_input(EXAMPLE_DATA_01);

        let results = input
            .iter()
            .map(|row| {
                let mut program = Program::new(&row);
                program.run();
                program
            })
            .collect::<Vec<Program>>();

        assert_eq!(results[0].memory, [1101, 100, -1, 4, 99]);
        assert_eq!(results[1].memory, [1002, 4, 3, 4, 99]);
        assert_eq!(results[0].output, []);
        assert_eq!(results[1].output, []);
    }
}
