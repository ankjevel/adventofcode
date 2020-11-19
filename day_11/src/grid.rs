use std::collections::HashMap;

pub struct Grid {
    pub grid_serial_number: u16,
    pub grid: HashMap<(u16, u16), i8>,
}

impl Grid {
    pub fn new(grid_serial_number: &u16) -> Self {
        Self {
            grid_serial_number: grid_serial_number.to_owned(),
            grid: HashMap::new(),
        }
    }

    fn power_level_for_cell(&self, x: u16, y: u16) -> i8 {
        let (x, y, serial_number) = (x as u32, y as u32, self.grid_serial_number as u32);

        let rack_id = x + 10;
        let power_level = ((rack_id * y) + serial_number) * rack_id;

        let digit: i8 = power_level
            .to_string()
            .chars()
            .rev()
            .nth(2)
            .unwrap_or('0')
            .to_string()
            .parse::<_>()
            .unwrap();

        digit - 5
    }

    fn get_power_level(&self, grid_x: u16, grid_y: u16) -> i32 {
        let mut sum = 0;
        for x in (grid_x - 1)..=(grid_x + 1) {
            for y in (grid_y - 1)..=(grid_y + 1) {
                sum += self.grid.get(&(x, y)).unwrap_or(&0).to_owned() as i32;
            }
        }
        sum
    }

    pub fn generate(&mut self) {
        for x in 1..=300 {
            for y in 1..=300 {
                let power_level = self.power_level_for_cell(x, y);
                self.grid.insert((x, y), power_level);
            }
        }
    }

    pub fn find_largest_power_source(&self) -> ((u16, u16), i32) {
        let mut power_source = ((0, 0), 0);
        for x in 2..=299 {
            for y in 2..=299 {
                let power_level = self.get_power_level(x, y);
                if power_level > power_source.1 {
                    power_source = ((x - 1, y - 1), power_level);
                }
            }
        }
        power_source
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLES_FROM_PART_1: [((u16, u16), u16, i8); 3] = [
        ((122, 79), 57, -5),
        ((217, 196), 39, 0),
        ((101, 153), 71, 4),
    ];

    #[test]
    fn it_gets_the_same_result_as_the_example() {
        let (x, y) = (3, 5);
        let grid_serial_number = 8;
        let power_level = 4;

        let mut grid = Grid::new(&grid_serial_number);

        grid.generate();

        let cell = grid.grid.get(&(x, y));
        assert_eq!(cell, Some(&power_level));
    }

    #[test]
    fn it_gets_the_extended_examples_correct() {
        for ((x, y), grid_serial_number, power_level) in EXAMPLES_FROM_PART_1.iter() {
            let mut grid = Grid::new(grid_serial_number);

            grid.generate();

            let cell = grid.grid.get(&(x.to_owned(), y.to_owned()));
            let power_source = grid.find_largest_power_source();

            assert_eq!(cell, Some(power_level));
            assert_eq!(power_source.1, power_level.to_owned() as i32);
            assert_eq!(power_source.0, (x.to_owned(), y.to_owned()));
        }
    }
}
