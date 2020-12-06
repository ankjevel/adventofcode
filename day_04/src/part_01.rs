use std::{collections::HashMap, io::Result};

use crate::{passport::Passport, Input};

pub fn main(input: &Input) -> Result<usize> {
    Ok(input
        .iter()
        .map(|row| {
            let mut passport = HashMap::new();
            for (key, value) in row {
                passport.insert(key.to_string(), value.to_string());
            }
            passport.to_owned()
        })
        .filter(|passport| {
            if passport.keys().len() == 8 {
                true
            } else {
                passport.keys().len() == 7 && passport.contains_key("cid") == false
            }
        })
        .collect::<Vec<Passport>>()
        .len())
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    use super::*;

    const EXAMPLE_DATA: &'static str = "
        ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
        byr:1937 iyr:2017 cid:147 hgt:183cm
        
        iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
        hcl:#cfa07d byr:1929
        
        hcl:#ae17e1 iyr:2013
        eyr:2024
        ecl:brn pid:760753108 byr:1931
        hgt:179cm
        
        hcl:#cfa07d eyr:2025 pid:166559648
        iyr:2011 ecl:brn hgt:59in
    ";

    #[test]
    fn should_get_the_same_amount_of_valid_passports_as_example() -> Result<()> {
        assert_eq!(main(&parse_input(&EXAMPLE_DATA))?, 2);
        Ok(())
    }
}
