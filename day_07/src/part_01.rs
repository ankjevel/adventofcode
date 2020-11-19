use crate::parse_input;
use std::io::Result;

pub fn main(input: &str) -> Result<String> {
    let mut steps = parse_input(input.clone());
    let mut output_order = String::new();

    while !steps.is_empty() {
        let mut available: Vec<char> = steps
            .iter()
            .filter(|(_, required)| required.is_empty())
            .map(|(step, _)| *step)
            .collect();

        available.sort();

        let step = available.first().unwrap();

        output_order.push(*step);
        steps.remove(&step);

        for (_, required) in steps.iter_mut() {
            if !required.contains(&step) {
                continue;
            }

            required.remove(&step);
        }
    }

    Ok(output_order)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &'static str = include_str!("test_fixture.txt");

    #[test]
    fn it_should_get_result_as_example_01() {
        assert_eq!(main(EXAMPLE_DATA).unwrap(), "CABDFE")
    }
}
