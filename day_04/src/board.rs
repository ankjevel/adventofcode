use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct N {
    pub pos: (usize, usize),
    pub marked: bool,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Board {
    pub values: HashMap<u32, N>,
    pub completed: bool,
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
        Self {
            values,
            completed: false,
        }
    }

    pub fn new_draw(&mut self, n: &u32) {
        if self.values.contains_key(&n) {
            self.values.get_mut(n).unwrap().marked = true;
        }
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

    pub fn completed_row_or_column(&mut self) -> bool {
        if self.completed {
            return false;
        }

        let mut completed_row = vec![0, 0, 0, 0, 0];
        let mut completed_column = vec![0, 0, 0, 0, 0];
        for n in self.values.values_mut() {
            if !n.marked {
                continue;
            }

            let (row, col) = n.pos;
            completed_row[row] += 1;
            completed_column[col] += 1;
            if completed_row[row] == 5 || completed_column[col] == 5 {
                self.completed = true;
                return true;
            }
        }

        false
    }
}
