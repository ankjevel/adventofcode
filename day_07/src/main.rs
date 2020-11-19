use ::day_07::{part_01::main as part_01, part_02::main as part_02};
use std::io::Result;

fn main() -> Result<()> {
    let input = include_str!("../../input/day_07");
    let number_of_workers = 5;
    let duration_per_step = 60;

    println!("part_01: {}", part_01(input).unwrap());
    println!(
        "part_02: {}",
        part_02(input, number_of_workers, duration_per_step).unwrap()
    );

    Ok(())
}
