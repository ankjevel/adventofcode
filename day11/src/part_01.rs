use std::io::Result;

use crate::grid::Grid;

pub fn main(input: &u16) -> Result<String> {
    let mut grid = Grid::new(&input);
    grid.generate();
    let ((x, y), _) = grid.find_largest_power_source();

    Ok(format!("{},{}", x, y))
}
