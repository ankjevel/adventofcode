use std::io;

mod part_01;

fn main() -> io::Result<()> {
    let input = parse_input(include_str!("../../input/day_02"));

    let mut input_for_part_01 = input[0].clone();
    input_for_part_01[1] = 12;
    input_for_part_01[2] = 2;

    let output_for_part_01 = part_01::main(&input_for_part_01).unwrap();

    println!("part01: {}", output_for_part_01[0]);

    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut input_for_part_02 = input[0].clone();
            input_for_part_02[1] = noun;
            input_for_part_02[2] = verb;
            let output = part_01::main(&input_for_part_02).unwrap()[0];
            if output == 19690720 {
                println!("part02: {}", (100 * noun + verb));
                break;
            }
        }
    }

    Ok(())
}

fn parse_input(string: &str) -> Vec<Vec<u32>> {
    string
        .lines()
        .map(|string| string.trim())
        .filter(|string| !string.is_empty())
        .map(|string| {
            string
                .split(",")
                .map(|part| part.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA_01: &'static str = "
    1,0,0,0,99
    2,3,0,3,99
    2,4,4,5,99,0
    1,1,1,4,99,5,6,0,99";

    #[test]
    fn examples_for_part_1() {
        let input = parse_input(EXAMPLE_DATA_01);

        let results = input
            .iter()
            .map(|row| part_01::main(&row).unwrap())
            .collect::<Vec<Vec<u32>>>();

        assert_eq!(results[0], [2, 0, 0, 0, 99]);
        assert_eq!(results[1], [2, 3, 0, 6, 99]);
        assert_eq!(results[2], [2, 4, 4, 5, 99, 9801]);
        assert_eq!(results[3], [30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }
}
