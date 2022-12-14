use std::{
    collections::{HashMap, VecDeque},
    io::Result,
};

use crate::{
    pathfinding::{adjacent, bfs},
    point::Point,
    print::print,
    Input, Map,
};

fn find_start(map: &Map, current_start: Point) -> Vec<Point> {
    let can_move = |point: &Point| match map.get(&point) {
        Some(tile) => *tile == 1,
        None => false,
    };
    let mut frontier = VecDeque::new();
    let mut visited = HashMap::new();

    frontier.push_front(current_start);
    visited.insert(current_start, None);
    while !frontier.is_empty() {
        let point = frontier.pop_front().unwrap();

        for edge in adjacent(&map, &point) {
            if visited.contains_key(&edge) || !can_move(&edge) {
                continue;
            }
            frontier.push_back(edge);
            visited.insert(edge, Some(point));
        }
    }

    visited
        .clone()
        .keys()
        .map(Point::to_owned)
        .collect::<Vec<_>>()
}

pub fn main(input: &Input) -> Result<usize> {
    let shortest_route = find_start(&input.map, input.start)
        .into_iter()
        .map(|start| {
            bfs(&input.map, start.to_owned(), input.end, |a, b| {
                Input::can_move(&input.map, a, b)
            })
        })
        .map(|path| {
            println!("\nlen: {}", path.len());
            print(&path, &input.map);
            path.len()
        })
        .min()
        .unwrap_or(0);

    Ok(shortest_route)
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    use super::*;

    const EXAMPLE_DATA: &'static str = "
        Sabqponm
        abcryxxl
        accszExk
        acctuvwj
        abdefghi
    ";

    #[test]
    fn it_gets_the_example_correct() -> Result<()> {
        assert_eq!(main(&parse_input(&EXAMPLE_DATA))?, 29);
        Ok(())
    }
}
