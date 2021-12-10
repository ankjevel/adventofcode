use std::{io::Result, sync::Mutex};

use crate::{board::Board, Input};

pub fn main(input: &Input) -> Result<u32> {
    let boards = Mutex::new(
        input
            .boards
            .to_owned()
            .into_iter()
            .map(|b| Board::new(&b.to_owned()))
            .collect::<Vec<Board>>(),
    );

    let mut result = 0u32;

    'main: for n in &input.numbers {
        let mutable_boards: &mut Vec<_> = &mut *boards.try_lock().unwrap();
        for board in mutable_boards {
            board.new_draw(n);

            if board.completed_row_or_column() {
                result = board.get_unmarked() * n;
                break 'main;
            }
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    use super::*;

    const EXAMPLE_DATA: &'static str = "
        7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

        22 13 17 11  0
        8  2 23  4 24
        21  9 14 16  7
        6 10  3 18  5
        1 12 20 15 19
        
        3 15  0  2 22
        9 18 13 17  5
        19  8  7 25 23
        20 11 10 24  4
        14 21 16 12  6
        
        14 21 17 24  4
        10 16 15  9 19
        18  8 23 26 20
        22 11 13  6  5
        2  0 12  3  7
    ";

    #[test]
    fn it_gets_the_example_correct() -> Result<()> {
        assert_eq!(main(&parse_input(&EXAMPLE_DATA))?, 4512);
        Ok(())
    }
}
