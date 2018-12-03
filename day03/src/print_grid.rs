use std::{io, iter};

use claim::Claim;

struct Grid {
    width: i32,
    height: i32,
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

    fn append(&mut self, claim: &Claim) {
        let claim_width = claim.width() + 1;
        let claim_height = claim.height() + 1;

        if self.width <= claim_width {
            self.width = claim_width;
        }

        if self.height <= claim_height {
            self.height = claim_height;
        }

        for _ in (self.rows.len() as i32)..self.height {
            self.rows.push(
                iter::repeat('.'.to_string())
                    .take(self.width as usize)
                    .collect(),
            );
        }

        for _ in (self.rows[0].len() as i32)..self.width {
            for y in 0..self.height {
                self.rows[(y as usize)].push('.'.to_string());
            }
        }

        for y in (claim.rectangle.top_left.y)..(claim.rectangle.bottom_right.y) {
            for x in (claim.rectangle.top_left.x)..(claim.rectangle.bottom_right.x) {
                let xx = x as usize;
                let yy = y as usize;

                if self.rows[yy][xx] == "." {
                    self.rows[yy][xx] = claim.id.to_string();
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

    for claim in claims.iter() {
        grid.append(&claim);
    }

    Ok(grid.print())
}
