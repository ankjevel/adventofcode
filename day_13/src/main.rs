use ::day_13::{get_instructions, into_chunks, parse_input, Tile::*};

fn print(input: &Vec<i64>) {
    let instructions = into_chunks(&get_instructions(input));
    let max_x = &instructions
        .iter()
        .fold(0, |prev, curr| if prev < curr.0 { curr.0 } else { prev });
    let max_y = &instructions
        .iter()
        .fold(0, |prev, curr| if prev < curr.1 { curr.1 } else { prev });
    let mut grid = vec![vec![' '; (max_x + 1) as usize]; (max_y + 1) as usize];
    for (x, y, tile) in instructions {
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
    let instructions = into_chunks(&get_instructions(input));
    instructions.iter().fold(
        0,
        |acc, (_x, _y, tile)| if tile == &Block { acc + 1 } else { acc },
    )
}

fn main() {
    let input = parse_input(&include_str!("../../input/day_13"));
    println!("part_01: {}", part_01(&input));
    print(&input);
}
