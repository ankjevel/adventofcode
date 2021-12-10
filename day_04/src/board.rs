use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct N {
    pub pos: (usize, usize),
    pub marked: bool,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Board {
    pub values: HashMap<u32, N>,
}

impl Board {
    pub fn new(input: &Vec<Vec<u32>>) -> Self {
        let mut values = HashMap::new();

        for (i, n) in input.into_iter().enumerate() {
            for (j, nn) in n.into_iter().enumerate() {
                values.insert(
                    nn.to_owned(),
                    N {
                        pos: (i, j),
                        marked: false,
                    },
                );
            }
        }

        Self { values }
    }

    pub fn new_draw(&mut self, n: &u32) {
        if !self.values.contains_key(&n) {
            return;
        }

        self.values.get_mut(n).unwrap().marked = true;
    }

    pub fn get_unmarked(&self) -> u32 {
        let mut sum = 0u32;
        for b in self.values.iter() {
            if !b.1.marked {
                sum += b.0;
            }
        }
        sum
    }

    pub fn completed_row_or_column(&self) -> bool {
        let mut completed_row = vec![0, 0, 0, 0, 0];
        let mut completed_column = vec![0, 0, 0, 0, 0];
        for n in self.values.values() {
            if n.marked {
                completed_row[n.pos.0] += 1;
                completed_column[n.pos.1] += 1;
            }
        }

        for row in &completed_row {
            if row == &5 {
                return true;
            }

            for column in &completed_column {
                if column == &5 {
                    return true;
                }
            }
        }

        false
    }
}
