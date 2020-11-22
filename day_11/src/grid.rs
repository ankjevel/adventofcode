use crate::iterate_x_y::iterate_x_y;

pub struct Grid {
    pub grid_serial_number: u16,
    pub grid: Vec<Vec<i32>>,
}

impl Grid {
    fn max_growth(x: u16, y: u16, max_size: u16) -> u16 {
        let (x, y) = (max_size - x, max_size - y);
        if x < y {
            x
        } else {
            y
        }
    }

    fn power_level_for_cell(x: usize, y: usize, grid_serial_number: u32) -> i8 {
        let (x, y, serial_number) = (x as u32, y as u32, grid_serial_number);

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

    pub fn new(grid_serial_number: &u16) -> Self {
        let grid_serial_number = grid_serial_number.to_owned();

        let mut grid = Vec::new();
        for y in 1..=300 {
            let mut row = Vec::new();
            for x in 1..=300 {
                row.push(Grid::power_level_for_cell(x, y, grid_serial_number as u32) as i32)
            }
            grid.push(row);
        }

        Self {
            grid_serial_number,
            grid,
        }
    }

    fn max_size(&self) -> u16 {
        self.grid.len() as u16
    }

    fn get_power_level(&self, x: u16, y: u16, size: usize) -> i32 {
        let (x, y, max_size) = (x as usize - 1, y as usize - 1, self.max_size() as usize);
        if x + size > max_size || y + size > max_size {
            return 0;
        }

        self.grid[y..y + size].iter().fold(0, |acc, inner| {
            acc + inner[x..x + size].iter().sum::<i32>().to_owned()
        })
    }

    pub fn find_largest_power_source(&self) -> ((u16, u16), i32) {
        let mut power_source = ((0, 0), 0);
        iterate_x_y(1, self.max_size(), |x, y| {
            let power_level = self.get_power_level(x, y, 3);
            if power_level > power_source.1 {
                power_source = ((x, y), power_level);
            }
        });
        power_source
    }

    pub fn find_square_with_largest_total_power(&self) -> ((u16, u16), u16) {
        let mut max = ((0, 0), 0, 0);
        iterate_x_y(1, self.max_size(), |x, y| {
            match Grid::max_growth(x, y, self.max_size()) {
                0 | 1 => {}
                max_growth => {
                    for size in 1..=max_growth {
                        let power_level = self.get_power_level(x, y, size as usize);
                        if power_level > max.1 {
                            max = ((x, y), power_level, size);
                        }
                    }
                }
            }
        });
        (max.0, max.2)
    }

    #[cfg(test)]
    fn cell(&self, x: u16, y: u16) -> i32 {
        self.grid
            .get(y as usize - 1)
            .unwrap()
            .get(x as usize - 1)
            .unwrap_or(&0)
            .to_owned()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLES_FROM_PART_1: [((u16, u16), u16, i32); 3] = [
        ((122, 79), 57, -5),
        ((217, 196), 39, 0),
        ((101, 153), 71, 4),
    ];

    fn parse_example(input: &str) -> Vec<Vec<i32>> {
        input
            .lines()
            .map(|string| string.trim())
            .filter(|string| !string.is_empty())
            .map(|string| {
                string
                    .split(' ')
                    .map(|string| string.trim())
                    .filter(|string| !string.is_empty())
                    .map(|p| p.parse::<_>().unwrap())
                    .collect()
            })
            .collect()
    }

    #[test]
    fn it_gets_the_same_result_as_the_example() {
        let (x, y) = (3, 5);
        let grid_serial_number = 8;
        let power_level = 4;

        let grid = Grid::new(&grid_serial_number);

        assert_eq!(grid.get_power_level(x, y, 1), power_level);
        assert_eq!(grid.cell(x, y), power_level);
    }

    #[test]
    fn it_gets_the_extended_examples_correct() {
        for ((x, y), grid_serial_number, power_level) in EXAMPLES_FROM_PART_1.iter() {
            let (x, y, power_level) = (x.to_owned(), y.to_owned(), power_level.to_owned());

            let grid = Grid::new(grid_serial_number);

            assert_eq!(grid.cell(x, y), power_level);
        }
    }

    #[test]
    fn it_works() {
        assert_eq!(
            Grid {
                grid_serial_number: 18,
                grid: parse_example(
                    "
                    -2  -4   4   4   4
                    -4   4   4   4  -5
                     4   3   3   4  -4
                     1   1   2   4  -3
                    -1   0   2  -5  -2
                    "
                ),
            }
            .get_power_level(2, 2, 3),
            29
        );

        assert_eq!(
            Grid {
                grid_serial_number: 42,
                grid: parse_example(
                    "
                    -3   4   2   2   2
                    -4   4   3   3   4
                    -5   3   3   4  -4
                     4   3   3   4  -3
                     3   3   3  -5  -1
                    "
                ),
            }
            .get_power_level(2, 2, 3),
            30
        );
    }
}
