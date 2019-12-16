use crate::enums::{
    Direction::{self, Down, Left, Right, Up},
    Status::{self, MovedAndOnOxygen, Wall},
    Tile,
};
use crate::paths;
use rand::{
    self,
    distributions::{Distribution, Uniform},
};
use std::{
    collections::HashMap,
    sync::mpsc::{Receiver, Sender},
    thread::sleep,
    time::Duration,
};
use termion::{clear::All, cursor::Restore};

type Point = (i64, i64);

pub struct RepairDroid {
    input: Receiver<i64>,
    output: Sender<i64>,
    grid: HashMap<Point, Tile>,
    end: Option<Point>,
    end_route: Option<Vec<Point>>,
    position: Point,
    direction: Direction,
    iterations: u64,
    steps: Vec<Direction>,
    visited: Vec<Point>,
}

impl RepairDroid {
    pub fn new(receiver: Receiver<i64>, sender: Sender<i64>) -> RepairDroid {
        let mut grid = HashMap::new();
        grid.insert((0, 0), Tile::Current);

        vec![(0, 1), (1, 0), (-1, 0), (0, -1)]
            .iter()
            .for_each(|pos| {
                grid.insert(pos.to_owned(), Tile::Unknown);
            });

        RepairDroid {
            input: receiver,
            output: sender,
            grid,
            position: (0, 0),
            steps: vec![Up],
            visited: vec![(0, -1)],
            direction: Up,
            iterations: 0,
            end: None,
            end_route: None,
        }
    }

    fn get_position(&mut self, direction: Direction) -> Point {
        let (x, y) = self.position.clone();
        match direction {
            Up => (x, y - 1),
            Right => (x + 1, y),
            Down => (x, y + 1),
            Left => (x - 1, y),
        }
    }

    fn get_grid(&mut self) -> (i64, i64, i64, i64) {
        let mut max_x = 0;
        let mut max_y = 0;
        let mut min_x = 0;
        let mut min_y = 0;

        for ((x, y), _tile) in self.grid.clone() {
            if x > max_x {
                max_x = x.to_owned();
            }
            if x < min_x {
                min_x = x.to_owned();
            }
            if y > max_y {
                max_y = y.to_owned();
            }
            if y < min_y {
                min_y = y.to_owned();
            }
        }

        let x_padding = 0 - min_x;
        let y_padding = 2 - min_y;

        (max_x + x_padding, x_padding, max_y + y_padding, y_padding)
    }

    fn print(&mut self) {
        let (max_x, x_padding, max_y, y_padding) = self.get_grid();

        print!("{}{}", All, Restore);
        println!("iterations: {}", self.iterations);

        let mut grid = vec![vec![' '; (max_x + 1) as usize]; (max_y + 1) as usize];

        for ((x, y), tile) in self.grid.clone() {
            if tile == Tile::Unknown {
                continue;
            }

            let start = x == 0 && y == 0;
            let end = !self.end.is_none() && self.end.unwrap() == (x, y);

            let x = (x + x_padding) as usize;
            let y = (y + y_padding) as usize;

            if let Some(elem) = grid.get_mut(y) {
                elem[x] = if start {
                    'S'
                } else if end {
                    'E'
                } else {
                    match tile {
                        Tile::Current => 'C',
                        Tile::Wall => '#',
                        Tile::Visited => '.',
                        _ => ' ',
                    }
                };
            }
        }

        if let Some(vec) = &self.end_route {
            for (x, y) in vec {
                let (x, y) = (x.to_owned(), y.to_owned());
                let start = x == 0 && y == 0;
                let end = self.end.unwrap() == (x, y);
                let x = (x + x_padding) as usize;
                let y = (y + y_padding) as usize;

                if let Some(elem) = grid.get_mut(y) {
                    elem[x] = if start {
                        'S'
                    } else if end {
                        'E'
                    } else {
                        ' '
                    };
                }
            }
        }

        for row in grid {
            println!("{}", row.clone().iter().collect::<String>());
        }
    }

    fn set_direction_of_closest_unknown(&mut self) {
        if let Some(steps) = paths::best_match(&self.grid, &self.position, &self.visited) {
            let iter = steps.iter();
            let mut steps = Vec::new();
            let mut prev = self.position.to_owned();
            for (x, y) in iter.rev() {
                if x == &self.position.0 && y == &self.position.1 {
                    continue;
                }

                let direction = if x == &prev.0 {
                    if y < &prev.1 {
                        Up
                    } else {
                        Down
                    }
                } else {
                    if x < &prev.0 {
                        Left
                    } else {
                        Right
                    }
                };

                steps.push(direction.to_owned());
                prev = (x.to_owned(), y.to_owned());
            }

            self.steps = steps.to_owned();
        } else {
            println!("no path found :(");
        }
    }

    fn new_last_goal(&mut self, direction: Option<Direction>) {
        if let Some(direction) = direction {
            let point = self.get_position(direction.to_owned());
            self.visited.push(point.to_owned())
        }
    }

    fn next_step(&mut self) -> Option<Direction> {
        let mut steps = self.steps.clone().into_iter();

        let new_last_goal = self.steps.len() == 1;

        let next = steps.next();

        if new_last_goal {
            self.new_last_goal(next);
        }

        self.steps = steps.collect();

        next
    }

    fn next_direction(&mut self) -> i64 {
        self.direction = match &self.next_step() {
            Some(dir) => dir.to_owned(),
            _ => {
                let mut rng = rand::thread_rng();
                let range = Uniform::from(1..=4);
                let dir = range.sample(&mut rng);
                Direction::new(dir)
            }
        };

        Direction::dir(self.direction)
    }

    fn move_droid(&mut self) -> bool {
        let next_direction = self.next_direction();

        self.output.send(next_direction.to_owned()).unwrap();

        let status = match self.input.recv() {
            Ok(curr) => Status::new(curr),
            Err(error) => panic!("error: {}", error),
        };

        if status == MovedAndOnOxygen {
            self.end = Some(self.position.clone());
        }

        match status {
            Wall => {
                let wall_position = self.get_position(self.direction);
                self.grid.insert(wall_position, Tile::Wall);
                self.set_direction_of_closest_unknown();
            }
            _ => {
                self.grid.insert(self.position.clone(), Tile::Visited);
                self.position = self.get_position(self.direction);
                self.grid.insert(self.position.clone(), Tile::Current);
                vec![(0, 1), (1, 0), (-1, 0), (0, -1)]
                    .iter()
                    .for_each(|(x, y)| {
                        let pos = self.position;
                        let pos = (pos.0 + x, pos.1 + y).to_owned();

                        if !self.grid.contains_key(&pos) {
                            self.grid.insert(pos.to_owned(), Tile::Unknown);
                        }
                    });

                if self.steps.len() == 0 {
                    self.set_direction_of_closest_unknown();
                }
            }
        };

        let unknown_left = self.grid.iter().fold(0, |acc, curr| {
            if curr.1 == &Tile::Unknown {
                acc + 1
            } else {
                acc
            }
        });

        unknown_left == 0
    }

    pub fn run(&mut self) -> usize {
        while !self.move_droid() {
            sleep(Duration::from_millis(10));
            self.iterations += 1;
            self.print();
        }

        if let Some(result) = paths::find_path(&self.grid.to_owned(), (0, 0), self.end.unwrap()) {
            self.end_route = Some(result.to_owned());
            self.print();
            result.len()
        } else {
            0
        }
    }
}
