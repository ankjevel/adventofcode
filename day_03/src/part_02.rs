use std::io::Result;

use crate::Input;

pub const DIRECTIONS: [(i32, i32); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

pub fn main(input: &Input) -> Result<u32> {
    Ok(input
        .rows
        .clone()
        .into_iter()
        .enumerate()
        .fold(vec![], |mut v: Vec<_>, (y, row)| {
            row.symbols.iter().for_each(|n| {
                if n.1 == '*' {
                    v.push((n.0, y));
                }
            });
            v
        })
        .into_iter()
        .map(|(x, y)| {
            DIRECTIONS
                .iter()
                .fold(vec![], |mut dirs: Vec<(usize, usize)>, (dx, dy)| {
                    let x = i32::try_from(x).unwrap();
                    let y = i32::try_from(y).unwrap();
                    let nx = x.to_owned() + dx.to_owned();
                    let ny = y.to_owned() + dy.to_owned();

                    if nx == -1 || nx == i32::try_from(input.max_x).unwrap() {
                        return dirs;
                    }

                    if ny == -1 || ny == i32::try_from(input.max_y).unwrap() {
                        return dirs;
                    }

                    dirs.push((usize::try_from(nx).unwrap(), usize::try_from(ny).unwrap()));
                    dirs
                })
        })
        .fold(vec![], |mut matches: Vec<u32>, positions| {
            let m = positions
                .clone()
                .into_iter()
                .fold(vec![], |mut m: Vec<u32>, (x, y)| {
                    let matched = input
                        .rows
                        .get(y)
                        .unwrap()
                        .numbers
                        .iter()
                        .find(|n| n.0.contains(&x));

                    if matched.is_none() {
                        return m;
                    }

                    let number = matched.unwrap().1;

                    if m.contains(&number) {
                        return m;
                    }

                    m.push(number);

                    m
                });
            if m.len() == 2 {
                matches.push(m.iter().product());
            }

            matches
        })
        .iter()
        .sum())
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    use super::*;

    const EXAMPLE_DATA: &'static str = "
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..
    ";

    #[test]
    fn it_gets_the_example_correct() -> Result<()> {
        assert_eq!(main(&parse_input(&EXAMPLE_DATA))?, 467835);
        Ok(())
    }
}
