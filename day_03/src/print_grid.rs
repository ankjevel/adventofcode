use colored::*;
use std::char::from_u32;
use std::{cmp, io, iter};

use claim::Claim;

struct Grid {
    width: u32,
    height: u32,
    rows: Vec<Vec<String>>,
}

impl Grid {
    fn new() -> Self {
        Grid {
            width: 0,
            height: 0,
            rows: vec![],
        }
    }

    fn valid_num(character: u32) -> u32 {
        cmp::min(cmp::max(character, 49), 126)
    }

    fn colored(num: u32, group: u32) -> String {
        let string = from_u32(num as u32).unwrap().to_string();

        let colored = match group {
            2 => string.red().to_string(),
            3 => string.green().to_string(),
            4 => string.yellow().to_string(),
            5 => string.blue().to_string(),
            6 => string.magenta().to_string(),
            7 => string.cyan().to_string(),
            8 => string.white().bold().to_string(),
            9 => string.red().bold().to_string(),
            10 => string.green().bold().to_string(),
            11 => string.yellow().bold().to_string(),
            12 => string.blue().bold().to_string(),
            13 => string.magenta().bold().to_string(),
            14 => string.cyan().bold().to_string(),
            15 => string.white().italic().to_string(),
            16 => string.red().italic().to_string(),
            17 => string.green().italic().to_string(),
            18 => string.yellow().italic().to_string(),
            19 => string.blue().italic().to_string(),
            20 => string.magenta().italic().to_string(),
            21 => string.cyan().italic().to_string(),
            22 => string.white().dimmed().to_string(),
            23 => string.red().dimmed().to_string(),
            24 => string.green().dimmed().to_string(),
            25 => string.yellow().dimmed().to_string(),
            26 => string.blue().dimmed().to_string(),
            27 => string.magenta().dimmed().to_string(),
            29 => string.cyan().dimmed().to_string(),

            _ => string,
        };

        colored.to_string()
    }

    fn character(character: u32) -> String {
        let mut num = 48 + character;
        let rem = num % 126;

        if num != rem {
            num = Grid::valid_num(rem + 48);
        }

        if num >= 88 {
            num = Grid::valid_num(num + 1);
        }

        if num >= 120 {
            num = Grid::valid_num(num + 1);
        }

        let round = ((character as f32) / 126_f32).ceil() as u32;

        Grid::colored(num, round)
    }

    fn append(&mut self, claim: &Claim) {
        let claim_width = claim.width() + 1;
        let claim_height = claim.height() + 1;

        if self.width <= claim_width {
            self.width = claim_width;
        }

        if self.height <= claim_height {
            self.height = claim_height;
        }

        for _ in (self.rows.len() as u32)..self.height {
            self.rows.push(
                iter::repeat('.'.to_string())
                    .take(self.width as usize)
                    .collect(),
            );
        }

        for _ in (self.rows[0].len() as u32)..self.width {
            for y in 0..self.height {
                self.rows[(y as usize)].push('.'.to_string());
            }
        }

        for y in (claim.rectangle.top_left.y)..(claim.rectangle.bottom_right.y) {
            for x in (claim.rectangle.top_left.x)..(claim.rectangle.bottom_right.x) {
                let xx = x as usize;
                let yy = y as usize;

                if self.rows[yy][xx] == "." {
                    self.rows[yy][xx] = Grid::character(claim.id);
                } else {
                    self.rows[yy][xx] = 'x'.to_string();
                }
            }
        }
    }

    fn size(&self) -> (usize, usize) {
        let w = self.rows[0].len();
        let h = self.rows.len();
        (w, h)
    }

    fn print(&self) -> String {
        let mut string = String::new();

        let (width, height) = self.size();

        for h in 0..height {
            for w in 0..width {
                let character = &self.rows[h as usize][w as usize];
                string.push_str(&character);
            }

            if h != height - 1 {
                string.push('\n');
            }
        }

        string
    }
}

pub fn main(claims: &Vec<Claim>) -> io::Result<String> {
    let mut grid = Grid::new();

    claims.iter().for_each(|claim| grid.append(&claim));

    Ok(grid.print())
}
