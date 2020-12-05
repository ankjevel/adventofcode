use std::io::Result;

use day_02::{parse_input, part_01::main as part_01, part_02::main as part_02};

fn main() -> Result<()> {
    let input = parse_input(include_str!("../../input/day_02"));

    println!("part_01: {:?}", part_01(&input).unwrap());
    println!("part_02: {:?}", part_02(&input).unwrap());

    Ok(())
}
