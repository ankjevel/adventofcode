use std::collections::{BTreeMap, LinkedList};

use crate::knot::Knot;

fn join<'a>(a: String, b: String) -> String {
    let (a, b) = (a.to_owned(), b.to_owned());
    let c = [a, b].concat();

    c.to_string()
}

pub fn combine_head_and_tails(head: &Knot, tails: &LinkedList<Knot>) -> Vec<(isize, isize)> {
    let mut points = vec![head.position.clone()];
    let mut tails: Vec<_> = tails
        .clone()
        .iter_mut()
        .map(|tail| tail.position.clone())
        .collect::<_>();
    points.append(&mut tails);
    points
}

pub fn print_grid(points: Vec<(isize, isize)>) {
    let board: Vec<((isize, isize), String)> = (-10isize..10isize)
        .flat_map(|y| {
            (-20isize..20isize)
                .map(|x| ((y, x), ".".to_string()))
                .collect::<Vec<_>>()
        })
        .collect::<_>();

    let mut board: BTreeMap<(isize, isize), String> = BTreeMap::from_iter(board.into_iter());
    for (i, (x, y)) in points.to_owned().into_iter().enumerate() {
        let pos = board.get_mut(&(y, x)).unwrap();
        if pos != "." {
            continue;
        }
        *pos = if i == 0 {
            "H".to_string()
        } else {
            i.to_string()
        };
    }

    let mut string = "".to_string();
    let mut out = Vec::new();
    let mut current = -5;
    for (pos, key) in board {
        if pos.0 != current {
            out.push(string.to_owned());
            string = "".to_string();
            current = pos.0.to_owned();
        }
        string = join(string, key)
    }

    // print!("{}c{}\r\n", 27 as char, out.join("\r\n"));
    print!("\n{}\r\n", out.join("\r\n"));
}
