use std::io::Result;

use crate::grid::Grid;

pub fn main(input: &u16) -> Result<String> {
    let mut grid = Grid::new(&input);
    grid.generate();
    let ((x, y), _) = grid.find_largest_power_source();

    Ok(format!("{},{}", x, y))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_gets_the_field_with_highest_power_level() {
        let (x, y) = (33, 45);
        let grid_serial_number = 18;

        assert_eq!(main(&grid_serial_number).unwrap(), format!("{},{}", x, y));
    }
}
