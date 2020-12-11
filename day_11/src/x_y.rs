use crate::Input;

pub const DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

pub fn max_x_y(input: &Input) -> (usize, usize) {
    let max_x = *input.iter().map(|((x, _), _)| x).max().unwrap() + 1;
    let max_y = *input.iter().map(|((_, y), _)| y).max().unwrap() + 1;

    (max_x, max_y)
}

pub fn iterate_x_y<F>(max_x: usize, max_y: usize, mut call: F)
where
    F: FnMut(usize, usize) -> (),
{
    for y in 0..max_y {
        for x in 0..max_x {
            call(x, y)
        }
    }
}
