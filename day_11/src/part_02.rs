use std::{io::Result, mem::replace};

use crate::{
    tile::Tile::*,
    x_y::{iterate_x_y, max_x_y, DIRECTIONS},
    Input,
};

fn occupied_adjacent_seats(tiles: &Input, x: usize, y: usize) -> usize {
    let (x, y) = (x as isize, y as isize);
    let as_usize = |x, y| (x as usize, y as usize);
    let is_floor = |x, y| match tiles.get(&as_usize(x, y)) {
        Some(Floor) => true,
        _ => false,
    };
    let is_occupied = |x, y| match tiles.get(&as_usize(x, y)) {
        Some(Occupied) => true,
        _ => false,
    };
    DIRECTIONS
        .iter()
        .filter(|(dx, dy)| {
            let mut nx = x + dx;
            let mut ny = y + dy;
            while is_floor(nx, ny) {
                nx += dx;
                ny += dy;
            }
            is_occupied(nx, ny)
        })
        .count()
}

pub fn main(input: &Input) -> Result<usize> {
    let mut tiles = input.clone();
    let (max_x, max_y) = max_x_y(&tiles);

    loop {
        let mut modified_tiles = tiles.clone();

        iterate_x_y(max_x, max_y, |x, y| {
            let current_tile = tiles.get(&(x, y)).unwrap();
            if current_tile == &Floor {
                return;
            }

            let occupied_seats = occupied_adjacent_seats(&tiles, x, y);
            if current_tile == &Occupied && occupied_seats >= 5 {
                modified_tiles.insert((x, y), Empty);
            } else if current_tile != &Occupied && occupied_seats == 0 {
                modified_tiles.insert((x, y), Occupied);
            }
        });

        if modified_tiles == tiles {
            break;
        }

        let _ = replace(&mut tiles, modified_tiles);
    }

    Ok(tiles.iter().filter(|(_, tile)| tile == &&Occupied).count())
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    use super::*;

    const EXAMPLE_DATA: &'static str = "
        L.LL.LL.LL
        LLLLLLL.LL
        L.L.L..L..
        LLLL.LL.LL
        L.LL.LL.LL
        L.LLLLL.LL
        ..L.L.....
        LLLLLLLLLL
        L.LLLLLL.L
        L.LLLLL.LL
    ";

    #[test]
    fn it_gets_the_example_correct() -> Result<()> {
        assert_eq!(main(&parse_input(&EXAMPLE_DATA))?, 26);
        Ok(())
    }
}
