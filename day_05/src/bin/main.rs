use ::day_05::{parse_input, program::exec_without_channels};
use std::io::Result;

fn main() -> Result<()> {
    let input = parse_input(include_str!("../../../input/day_05"));

    println!(
        "part_01: {:?}",
        exec_without_channels(input[0].clone(), None)
    );
    println!(
        "part_02: {:?}",
        exec_without_channels(input[0].clone(), Some(vec![5]))
    );

    Ok(())
}
