use std::io::Result;

use day10::{board::Board, parse_input};

fn main() -> Result<()> {
    let input = parse_input(include_str!("../../input/day_10"));

    Board::new(&input).run();

    Ok(())
}
