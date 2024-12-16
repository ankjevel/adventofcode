pub mod part_01;
pub mod part_02;

type Number = i32;

pub type Input = Vec<Vec<Number>>;

pub fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(str::trim)
        .filter(|string| !string.is_empty())
        .map(str::to_owned)
        .map(|line| {
            line.split_whitespace()
                .map(|string| string.parse().unwrap())
                .collect()
        })
        .collect()
}

pub fn is_safe(row: &&Vec<Number>) -> bool {
    let mut index = 1;
    while index < row.len() - 1 {
        if check_if_safe(row, index) {
            index += 1;
            continue;
        }
        return false;
    }
    true
}

fn check_if_safe(row: &[Number], index: usize) -> bool {
    let current = row.get(index).unwrap();
    if let Some(left) = row.get(index - 1) {
        if let Some(right) = row.get(index + 1) {
            let change_right = current - right;
            let change_left = left - current;

            if is_unsafe_value(change_left) || is_unsafe_value(change_right) {
                return false;
            }

            if is_unsafe_change(change_left, change_right) {
                return false;
            }
        }
    }
    true
}

fn is_unsafe_value(change: Number) -> bool {
    change == 0 || change.abs() > 3
}

fn is_unsafe_change(left: Number, right: Number) -> bool {
    let increase_left = left > 0;
    let increase_right = right > 0;

    increase_left != increase_right
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = "
        94 94 95 93 95 94
        4 4 6 3 8
    ";

    #[test]
    fn it_parses_example() {
        assert_eq!(
            parse_input(EXAMPLE_DATA),
            vec![vec![94, 94, 95, 93, 95, 94], vec![4, 4, 6, 3, 8]]
        );
    }

    #[test]
    fn it_handles_increased_changes() {
        assert!(
            !check_if_safe(&[2, 7, 8], 1,),
            "Unsafe because 2 7 is an increase of 5"
        );
    }

    #[test]
    fn it_handles_decreased_changes() {
        assert!(
            !check_if_safe(&[7, 6, 2], 1,),
            "Unsafe because 6 2 is a decrease of 4"
        );
    }

    #[test]
    fn it_handles_left_inc_and_right_dec() {
        assert!(
            !check_if_safe(&[7, 6, 2], 1,),
            "Unsafe because 1 3 is increasing but 3 2 is decreasing"
        );
    }

    #[test]
    fn it_handles_no_change() {
        assert!(
            !check_if_safe(&[6, 4, 4], 1,),
            "Unsafe because 4 4 is neither an increase or a decrease"
        );
    }
}
