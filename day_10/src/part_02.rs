use std::{collections::BTreeMap, io::Result};

use crate::{cpu::CPU, Input};

fn print(crt: &BTreeMap<i64, String>) -> String {
    let x = crt.to_owned().into_values().collect::<Vec<_>>();

    let y = x
        .chunks(40)
        .map(|chunk| chunk.to_owned().into_iter().collect::<String>())
        .collect::<Vec<_>>();

    y.join("\n")
}

pub fn main(input: &Input) -> Result<String> {
    let mut cpu = CPU::new();
    cpu.parse(input);

    let mut crt: BTreeMap<i64, String> =
        BTreeMap::from_iter((0..240).map(|n| (n as i64, ".".to_string())));

    for (position, (_, strength)) in cpu.stack.into_iter().enumerate() {
        let position = position as i64;
        let horizontal_position = position % 40;
        if (strength - horizontal_position).abs() < 2 {
            if let Some(pixel) = crt.get_mut(&position) {
                *pixel = "#".to_string();
            }
        }
    }

    Ok(print(&crt))
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    use super::*;

    #[test]
    fn it_gets_the_example_correct() -> Result<()> {
        let input = parse_input(include_str!("../../input/day_10_extended_example"));
        assert_eq!(
            main(&input)?,
            "
                ##..##..##..##..##..##..##..##..##..##..
                ###...###...###...###...###...###...###.
                ####....####....####....####....####....
                #####.....#####.....#####.....#####.....
                ######......######......######......####
                #######.......#######.......#######.....
            "
            .lines()
            .map(str::trim)
            .filter(|string| !string.is_empty())
            .map(str::to_owned)
            .collect::<Vec<_>>()
            .join("\n")
        );

        Ok(())
    }
}
