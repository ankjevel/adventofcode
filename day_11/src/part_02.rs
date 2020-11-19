use std::io::Result;

use crate::grid::Grid;

pub fn main(input: &u16) -> Result<String> {
    let _ = Grid::new(&input);

    Ok("".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_gets_the_examples_correct() {
        assert_eq!(
            main(&18).unwrap(),
            format!("{x},{y},{size}", x = 90, y = 269, size = 16)
        );
        assert_eq!(
            main(&42).unwrap(),
            format!("{x},{y},{size}", x = 232, y = 251, size = 12)
        );
    }
}
