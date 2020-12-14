use std::io::Result;

use crate::{chinese_remainder::chinese_remainder, Input};

pub fn main((_, busses): &Input) -> Result<i64> {
    let busses = busses
        .iter()
        .enumerate()
        .filter(|(_, bus)| bus.is_some())
        .map(|(i, bus)| (i as i64, bus.unwrap_or(0) as i64))
        .collect::<Vec<_>>();
    let mods = busses.iter().map(|&(_, b)| b).collect::<Vec<_>>();
    let res = busses.iter().map(|&(i, b)| b - i).collect::<Vec<_>>();
    Ok(chinese_remainder(&res, &mods).unwrap())
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    use super::*;

    #[test]
    fn it_gets_the_example_correct() -> Result<()> {
        assert_eq!(main(&parse_input("0\n7,13,x,x,59,x,31,19"))?, 1068781);
        assert_eq!(main(&parse_input("0\n17,x,13,19"))?, 3417);
        assert_eq!(main(&parse_input("0\n67,7,59,61"))?, 754018);
        assert_eq!(main(&parse_input("0\n67,x,7,59,61"))?, 779210);
        assert_eq!(main(&parse_input("0\n67,7,x,59,61"))?, 1261476);
        assert_eq!(main(&parse_input("0\n1789,37,47,1889"))?, 1202161486);
        Ok(())
    }
}
