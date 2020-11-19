use std::io::Result;

use day_10::{board::Board, parse_input};

fn main() -> Result<()> {
    let input = parse_input(include_str!("../../input/day_10"));

    println!("it took {} seconds to complete", Board::new(&input).run());

    Ok(())
}
