use std::collections::HashSet;

use crate::direction::Direction::{self, *};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Knot {
    pub position: (isize, isize),
    pub visited: Vec<(isize, isize)>,
}

impl Knot {
    pub fn new() -> Self {
        Knot {
            position: (0, 0),
            visited: vec![],
        }
    }

    fn next_move(direction: &Direction) -> (Option<isize>, Option<isize>) {
        (
            match direction {
                Up | Down => Some(if direction == &Up { -1 } else { 1 }),
                _ => None,
            },
            match direction {
                Left | Right => Some(if direction == &Left { -1 } else { 1 }),
                _ => None,
            },
        )
    }

    fn store_position(&mut self) {
        self.visited.push(self.position.to_owned());
    }

    fn diff(&self, reference: &Knot) -> (isize, isize) {
        let horizontal_diff = (self.position.0 - reference.position.0).abs();
        let vertical_diff = (self.position.1 - reference.position.1).abs();
        (horizontal_diff, vertical_diff)
    }

    fn should_move(&self, reference: &Knot) -> bool {
        let (horizontal_diff, vertical_diff) = self.diff(reference);
        horizontal_diff > 1 || vertical_diff > 1
    }

    fn move_towards(&mut self, reference: &Knot) {
        let (ref_x, ref_y) = reference.position.to_owned();
        let (pos_x, pos_y) = self.position.to_owned();

        self.store_position();

        let (horizontal_diff, vertical_diff) = self.diff(reference);

        let x = if ref_x > pos_x { 1 } else { -1 };
        let y = if ref_y > pos_y { 1 } else { -1 };

        if horizontal_diff == 0 || vertical_diff == 0 {
            if vertical_diff == 0 {
                self.position.0 += x;
            } else {
                self.position.1 += y;
            }
        } else {
            self.position.0 += x;
            self.position.1 += y;
        }
    }

    pub fn goto(&mut self, direction: &Direction) {
        let (x, y) = Knot::next_move(direction);

        self.store_position();

        self.position.0 += x.unwrap_or(0);
        self.position.1 += y.unwrap_or(0);
    }

    pub fn visited(&self) -> usize {
        let hash_set: HashSet<(isize, isize)> =
            HashSet::from_iter(self.visited.clone().to_owned().into_iter());
        hash_set.len()
    }

    pub fn maybe_move(&mut self, reference: &Knot) {
        if self.should_move(reference) {
            self.move_towards(reference)
        }
    }
}
