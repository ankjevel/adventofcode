use ::day_13::{get_instructions, parse_input};

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
enum Tile {
    Empty,
    Wall,
    Block,
    HorizontalPaddle,
    Ball,
}

use Tile::*;

impl Tile {
    pub fn from(input: &i64) -> Tile {
        match input {
            0 => Empty,
            1 => Wall,
            2 => Block,
            3 => HorizontalPaddle,
            4 => Ball,
            _ => panic!("enum not defined {}", input),
        }
    }
}

fn print(input: &Vec<i64>) {
    let instructions = get_instructions(input);

    let chunked: Vec<(usize, usize, Tile)> = instructions
        .chunks(3)
        .into_iter()
        .map(|instructions| {
            let mut iter = instructions.into_iter();
            let x = iter.next().unwrap();
            let y = iter.next().unwrap();
            let id = iter.next().unwrap();
            (x.to_owned() as usize, y.to_owned() as usize, Tile::from(id))
        })
        .collect::<Vec<_>>();

    let max_x = &chunked
        .iter()
        .fold(0, |prev, curr| if prev < curr.0 { curr.0 } else { prev });

    let max_y = &chunked
        .iter()
        .fold(0, |prev, curr| if prev < curr.1 { curr.1 } else { prev });

    let mut grid = vec![vec![' '; (max_x + 1) as usize]; (max_y + 1) as usize];

    for (x, y, tile) in chunked {
        if tile == Empty {
            continue;
        }

        if let Some(elem) = grid.get_mut(y) {
            elem.insert(
                x,
                match tile {
                    Wall => '#',
                    Block => '◻',
                    HorizontalPaddle => '=',
                    Ball => 'o',
                    _ => ' ',
                },
            );
        }
    }

    for row in grid.into_iter() {
        println!("{}", row.into_iter().collect::<String>())
    }
}

fn part_01(input: &Vec<i64>) -> usize {
    let instructions = get_instructions(input);

    let chunked: Vec<(usize, usize, Tile)> = instructions
        .chunks(3)
        .into_iter()
        .map(|instructions| {
            let mut iter = instructions.into_iter();
            let x = iter.next().unwrap();
            let y = iter.next().unwrap();
            let id = iter.next().unwrap();
            (x.to_owned() as usize, y.to_owned() as usize, Tile::from(id))
        })
        .collect::<Vec<_>>();

    chunked.iter().fold(
        0,
        |acc, (_x, _y, tile)| if tile == &Block { acc + 1 } else { acc },
    )
}

fn main() {
    let input = parse_input(&include_str!("../../input/day_13"));
    println!("part_01: {}", part_01(&input));
    print(&input);
}
