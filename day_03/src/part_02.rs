use std::io::Result;

use crate::Input;

pub fn main(input: &Input) -> Result<u32> {
    let process = |cmp: fn((Vec<usize>, Vec<usize>)) -> Vec<usize>| -> String {
        let mut res = "".to_owned();
        let line_len = input.first().unwrap().len();
        let mut lines = input.clone();
        let mut sum: (Vec<usize>, Vec<usize>) = (vec![], vec![]);
        for bit_n in 0..line_len {
            lines.clone().iter().enumerate().for_each(|(i, row)| {
                match row.chars().collect::<Vec<_>>()[bit_n] {
                    '1' => sum.1.push(i),
                    _ => sum.0.push(i),
                }
            });

            let entries = cmp(sum);

            if entries.len() == 1 {
                res = lines[entries[0]].to_owned();
                break;
            }

            lines = lines
                .clone()
                .iter()
                .enumerate()
                .filter(|(i, _n)| entries.contains(&i))
                .map(|(_i, line)| line.to_owned())
                .collect::<Vec<_>>();

            sum = (vec![], vec![]);
        }
        res
    };

    let oxygen_generator_rating = process(|sum| {
        let (sum_0, sum_1) = (sum.0.len(), sum.1.len());
        if sum_1 == sum_0 || sum_1 > sum_0 {
            sum.1.to_owned()
        } else {
            sum.0.to_owned()
        }
    });

    let co2_scrubber_rating = process(|sum| {
        let (sum_0, sum_1) = (sum.0.len(), sum.1.len());
        if sum_1 == sum_0 || sum_1 > sum_0 {
            sum.0.to_owned()
        } else {
            sum.1.to_owned()
        }
    });

    Ok(u32::from_str_radix(&oxygen_generator_rating, 2).unwrap()
        * u32::from_str_radix(&co2_scrubber_rating, 2).unwrap())
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    use super::*;

    const EXAMPLE_DATA: &'static str = "
        00100
        11110
        10110
        10111
        10101
        01111
        00111
        11100
        10000
        11001
        00010
        01010
    ";

    #[test]
    fn it_gets_the_example_correct() -> Result<()> {
        assert_eq!(main(&parse_input(&EXAMPLE_DATA))?, 230);
        Ok(())
    }
}
