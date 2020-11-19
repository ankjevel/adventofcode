use day_09::{parse_input, Game};
use std::io::Result;

fn main() -> Result<()> {
    let (players, run_until, _) = parse_input(include_str!("../../input/day_09"))[0];

    println!("part_01: {}", Game::new(players, run_until).run());
    println!("part_02: {}", Game::new(players, run_until * 100).run());

    Ok(())
}
