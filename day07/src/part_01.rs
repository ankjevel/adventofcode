use std::io::Result;

use Steps;

pub fn main(input: &Steps) -> Result<String> {
    let mut output_order = String::new();
    let mut steps = input.clone();

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
