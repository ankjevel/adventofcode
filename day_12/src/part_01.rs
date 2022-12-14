use std::io::Result;

use crate::{pathfinding::bfs, print::print, Input};

pub fn main(input: &Input) -> Result<usize> {
    let path = bfs(&input.map, input.start, input.end, |a, b| {
        Input::can_move(&input.map, a, b)
    });
    print(&path, &input.map);
    Ok(path.len())
}
#[cfg(test)]
mod tests {
    use crate::parse_input;

    use super::*;

    const EXAMPLE_DATA: &'static str = "
        Sabqponm
        abcryxxl
        accszExk
        acctuvwj
        abdefghi
    ";

    #[test]
    fn it_gets_the_example_correct() -> Result<()> {
        assert_eq!(main(&parse_input(&EXAMPLE_DATA))?, 31);
        Ok(())
    }
}
