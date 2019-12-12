use crate::direction::{Dir::*, Direction};
use crate::point::Point;
use std::{
    collections::HashSet,
    sync::mpsc::{Receiver, Sender},
};

pub struct Robot {
    position: Point,
    direction: Direction,
    input: Receiver<i64>,
    output: Sender<i64>,
    visited: HashSet<Point>,
    whites: HashSet<Point>,
}

impl Robot {
    pub fn new(receiver: Receiver<i64>, sender: Sender<i64>, start_on_white: bool) -> Robot {
        let position = Point { x: 0, y: 0 };
        let mut visited = HashSet::new();
        let mut whites = HashSet::new();

        visited.insert(position.clone());

        if start_on_white {
            whites.insert(position.clone());
        }

        Robot {
            input: receiver,
            output: sender,
            direction: Direction::new(),
            position,
            visited,
            whites,
        }
    }

    fn move_position(&mut self) {
        let x = self.position.x;
        let y = self.position.y;

        self.position = match self.direction.0 {
            Up => Point { x, y: y + 1 },
            Right => Point { x: x + 1, y },
            Down => Point { x, y: y - 1 },
            Left => Point { x: x - 1, y },
        }
    }

    pub fn run(&mut self) -> usize {
        loop {
            let over_white_panel = self.whites.contains(&self.position);
            self.output
                .send(if over_white_panel { 1 } else { 0 })
                .unwrap();

            let next_color = match self.input.recv() {
                Ok(0) => 0u8,
                Ok(1) => 1u8,
                _ => break,
            };

            if next_color == 0 {
                self.whites.remove(&self.position);
            } else {
                self.whites.insert(self.position.clone());
            }

            self.visited.insert(self.position.clone());

            let next_position = match self.input.recv() {
                Ok(0) => 0u8,
                Ok(1) => 1u8,
                _ => break,
            };

            self.direction.turn(next_position);
            self.move_position();
        }

        self.visited.len()
    }
}
