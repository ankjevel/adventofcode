use std::collections::HashMap;

use crate::knot::{Knot, Position};

fn join<'a>(a: String, b: String) -> String {
    let (a, b) = (a.to_owned(), b.to_owned());
    let c = [a, b].concat();

    c.to_string()
}

pub fn combine_head_and_tails(head: &Knot, tails: &Vec<Knot>) -> Vec<Position> {
    let mut points = vec![head.position.clone()];
    let mut tails: Vec<_> = tails
        .clone()
        .iter_mut()
        .map(|tail| tail.position.clone())
        .collect::<_>();
    points.append(&mut tails);
    points
}

fn min_max(points: &Vec<Position>) -> (Position, Position) {
    let first = points.clone().first().unwrap_or(&(0, 0)).to_owned();
    let (mut min_x, mut max_x, mut min_y, mut max_y) = (first.0, first.0, first.1, first.1);
    for (x, y) in points.to_owned() {
        if x > max_x {
            max_x = x;
        } else if x < min_x {
            min_x = x
        }

        if y > max_y {
            max_y = y
        } else if y < min_y {
            min_y = y
        }
    }
    ((min_x - 2, min_y - 2), (max_x + 2, max_y + 2))
}

pub fn print_grid(points: &Vec<Position>, visited: &Vec<Position>) {
    let mut all = points.to_owned();
    all.extend(&visited.to_owned());

    let ((min_x, min_y), (max_x, max_y)) = min_max(&all);

    let board: Vec<_> = (min_x..=max_x)
        .flat_map(|x| {
            (min_y..=max_y)
                .map(|y| ((x, y), ".".to_string()))
                .collect::<Vec<_>>()
        })
        .collect::<_>();

    let mut board: HashMap<_, _> = HashMap::from_iter(board.into_iter());
    for (i, (x, y)) in points.to_owned().into_iter().enumerate() {
        let pos = board.get_mut(&(x, y)).unwrap();
        if pos != "." {
            continue;
        }
        *pos = if i == 0 {
            "H".to_string()
        } else {
            i.to_string()
        };
    }

    for (i, (x, y)) in visited.to_owned().into_iter().enumerate() {
        let pos = board.get_mut(&(x, y)).unwrap();
        if pos != "." {
            continue;
        }
        *pos = if i == 0 {
            "s".to_string()
        } else {
            "#".to_string()
        };
    }

    let mut out = Vec::new();
    for y in min_y..=max_y {
        let mut string = "".to_string();
        for x in min_x..=max_x {
            string = join(string, board.get(&(x, y)).unwrap().to_owned())
        }
        out.push(string.to_owned());
    }

    println!("\r\n{}", out.join("\r\n"));
}
