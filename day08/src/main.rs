use ::day08::part_01::main as part_01;
use std::io::Result;

fn main() -> Result<()> {
    let input = include_str!("../../input/day_08");

    println!("part_01: {:?}", part_01(&input).unwrap());

    Ok(())
}
