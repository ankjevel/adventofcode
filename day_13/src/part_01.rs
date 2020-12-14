use std::io::Result;

use crate::Input;

pub fn main((earliest, busses): &Input) -> Result<u64> {
    let earliest = *earliest;
    let busses = busses
        .iter()
        .filter(|bus| bus.is_some())
        .map(|bus| bus.unwrap())
        .collect::<Vec<_>>();

    let mut result = 0;
    for timestamp in earliest.. {
        if let Some(bus_id) = busses.iter().find(|bus_id| timestamp % **bus_id == 0) {
            result = *bus_id * (timestamp - earliest);
            break;
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    use super::*;

    const EXAMPLE_DATA: &'static str = "
        939
        7,13,x,x,59,x,31,19
    ";

    #[test]
    fn it_gets_the_example_correct() -> Result<()> {
        assert_eq!(main(&parse_input(&EXAMPLE_DATA))?, 295);
        Ok(())
    }
}
