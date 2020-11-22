use std::io::Result;

use crate::grid::Grid;

pub fn main(input: &u16) -> Result<String> {
    let grid = Grid::new(&input);
    let ((x, y), size) = grid.find_square_with_largest_total_power();

    Ok(format!("{},{},{}", x, y, size))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_gets_the_first_example_correct() {
        assert_eq!(
            main(&18).unwrap(),
            format!("{x},{y},{size}", x = 90, y = 269, size = 16)
        );
    }

    #[test]
    fn it_gets_the_second_example_correct() {
        assert_eq!(
            main(&42).unwrap(),
            format!("{x},{y},{size}", x = 232, y = 251, size = 12)
        );
    }
}
