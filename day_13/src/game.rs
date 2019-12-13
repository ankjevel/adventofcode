use crate::Tile::{self, *};
use std::{
    collections::HashMap,
    sync::mpsc::{Receiver, Sender},
    time::Duration,
};
use termion::{clear::All, cursor::Restore};

fn to_tuple(input: &[i64]) -> (i64, i64, i64) {
    let mut iter = input.into_iter();
    let x = iter.next().unwrap();
    let y = iter.next().unwrap();
    let item = iter.next().unwrap();

    (x.to_owned(), y.to_owned(), item.to_owned())
}

fn parse_instructions(input: &Vec<i64>) -> (Vec<(usize, usize, Tile)>, Option<u32>) {
    let mut score = None;

    let instructions = input
        .clone()
        .chunks(3)
        .into_iter()
        .filter(|instruction| {
            let (x, y, id) = to_tuple(instruction);
            let new_score = x == -1 && y == 0;
            if new_score {
                score = Some(id.clone() as u32);
            }
            !new_score
        })
        .map(|instruction| {
            let (x, y, id) = to_tuple(instruction);
            let tile = Tile::from(&id);

            (x as usize, y as usize, tile)
        })
        .collect::<Vec<_>>();

    (instructions, score)
}

pub struct Game {
    input: Receiver<i64>,
    output: Sender<i64>,
    instructions: Vec<(usize, usize, Tile)>,
    grid: HashMap<(usize, usize), Tile>,
    score: u32,
    iterations: u32,
}

impl Game {
    pub fn new(receiver: Receiver<i64>, sender: Sender<i64>) -> Game {
        Game {
            input: receiver,
            output: sender,
            score: 0,
            iterations: 0,
            grid: HashMap::new(),
            instructions: vec![],
        }
    }

    fn print(&mut self) {
        let instructions = self.instructions.clone();
        let max_x = instructions
            .iter()
            .fold(0, |prev, curr| if prev < curr.0 { curr.0 } else { prev });
        let max_y = instructions
            .iter()
            .fold(0, |prev, curr| if prev < curr.1 { curr.1 } else { prev });
        let mut grid = vec![vec![' '; (max_x + 1) as usize]; (max_y + 1) as usize];

        for x in 0..=max_x {
            if let Some(elem) = grid.get_mut(0) {
                elem[x] = '#';
            }
        }

        for (x, y, tile) in self.instructions.clone() {
            if tile == Empty {
                continue;
            }

            if let Some(elem) = grid.get_mut(y) {
                elem[x] = match tile {
                    Wall => '#',
                    Block => '◻',
                    HorizontalPaddle => '=',
                    Ball => 'o',
                    _ => ' ',
                };
            }
        }

        print!("{}{}", All, Restore);
        println!("score: {}, iterations: {}", self.score, self.iterations);
        for row in grid {
            println!("{}", row.clone().iter().collect::<String>());
        }
    }

    fn add_initial_data(&mut self, input: Vec<i64>) {
        let (instructions, score) = parse_instructions(&input);

        if let Some(score) = score {
            self.score = score
        };

        for (x, y, item) in instructions.clone() {
            let grid = self.grid.entry((x, y)).or_insert(Empty);
            *grid = item;
        }

        self.instructions = instructions.clone();
    }

    fn update_data(&mut self, input: Vec<i64>) {
        let (instructions, score) = parse_instructions(&input);

        if let Some(score) = score {
            self.score = score
        };

        for (x, y, item) in instructions.clone() {
            let grid = self.grid.entry((x, y)).or_insert(Empty);
            *grid = item;
        }

        let mut instructions = vec![];
        for ((x, y), item) in self.grid.clone().iter() {
            instructions.push((x.to_owned(), y.to_owned(), item.to_owned()));
        }

        self.instructions.clear();
        self.instructions = instructions;
    }

    fn get_paddle_position(&mut self) -> i64 {
        let instructions = self.instructions.clone();
        let paddle = HorizontalPaddle;
        let paddle = instructions
            .iter()
            .fold(0, |acc, curr| if curr.2 == paddle { curr.0 } else { acc });

        let ball = instructions
            .iter()
            .fold(0, |acc, curr| if curr.2 == Ball { curr.0 } else { acc });

        if paddle > ball {
            -1
        } else if paddle < ball {
            1
        } else {
            0
        }
    }

    pub fn blocks_left(&mut self) -> i64 {
        self.instructions.iter().fold(
            0,
            |acc, (_x, _y, tile)| if tile == &Block { acc + 1 } else { acc },
        )
    }

    pub fn run(&mut self) -> u32 {
        let mut first_run = true;
        'main_loop: loop {
            let mut current_input = vec![];
            'collect_loop: loop {
                let curr = match self.input.recv_timeout(Duration::from_millis(25)) {
                    Ok(x) => x,
                    _ => break 'collect_loop,
                };

                current_input.push(curr.clone());
            }

            if first_run {
                self.add_initial_data(current_input.clone());
                first_run = false;
            } else {
                self.update_data(current_input.clone());
            }

            self.print();

            if self.blocks_left() <= 0 {
                break;
            }

            let position = self.get_paddle_position();
            match self.output.send(position) {
                _ => {}
            };

            self.iterations += 1;
        }

        self.score.to_owned()
    }
}
