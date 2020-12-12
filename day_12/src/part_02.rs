use std::io::Result;

use crate::{manhattan::manhattan, point::Point, ship::Ship, Input};

pub fn main(input: &Input) -> Result<u64> {
    let point = Ship::default().follow_waypoint(&input);
    Ok(manhattan(Point { x: 0, y: 0 }, point))
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    use super::*;

    const EXAMPLE_DATA: &'static str = "
        F10
        N3
        F7
        R90
        F11
    ";

    #[test]
    fn it_gets_the_example_correct() -> Result<()> {
        assert_eq!(main(&parse_input(&EXAMPLE_DATA))?, 286);
        Ok(())
    }
}
