use std::{collections::HashMap, io::Result};

use regex::Regex;

lazy_static! {
    static ref VALID_HEIGHT: Regex = Regex::new(r"(\d+)+((cm)|(in)){1}").unwrap();
    static ref VALID_HEX: Regex = Regex::new(r"#([a-f0-9]){6}").unwrap();
    static ref VALID_EYE_COLOR: Vec<String> = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
        .iter()
        .map(|string| string.to_string())
        .collect();
    static ref VALID_PIN: Regex = Regex::new(r"^\d{9}$").unwrap();
}

type Passport = HashMap<String, String>;

fn valid_birth_year(passport: &Passport) -> bool {
    match passport.get("byr") {
        Some(string) => (1920..=2002).contains(&string.parse::<u16>().unwrap_or(0)),
        _ => false,
    }
}
fn valid_issue_year(passport: &Passport) -> bool {
    match passport.get("iyr") {
        Some(string) => (2010..=2020).contains(&string.parse::<u16>().unwrap_or(0)),
        _ => false,
    }
}
fn valid_expiration_year(passport: &Passport) -> bool {
    match passport.get("eyr") {
        Some(string) => (2020..=2030).contains(&string.parse::<u16>().unwrap_or(0)),
        _ => false,
    }
}
fn valid_height(passport: &Passport) -> bool {
    let string = passport.get("hgt");
    if string.is_none() {
        return false;
    }

    let string = string.unwrap().to_owned();
    if !VALID_HEIGHT.is_match(&string) {
        return false;
    }

    let caps = VALID_HEIGHT.captures(&string).unwrap();
    let height = &caps.get(1).unwrap().as_str().parse::<u16>().unwrap_or(0);
    if let Some(_) = caps.get(3) {
        (150..=193).contains(height)
    } else {
        (59..=76).contains(height)
    }
}
fn valid_hair_color(passport: &Passport) -> bool {
    match passport.get("hcl") {
        None => false,
        Some(string) => VALID_HEX.is_match(&string),
    }
}
fn valid_eye_color(passport: &Passport) -> bool {
    match passport.get("ecl") {
        None => false,
        Some(string) => VALID_EYE_COLOR.contains(&string),
    }
}
fn valid_passport_id(passport: &Passport) -> bool {
    match passport.get("pid") {
        None => false,
        Some(string) => VALID_PIN.is_match(&string),
    }
}

fn valid(passport: &Passport) -> bool {
    vec![
        valid_birth_year(&passport),
        valid_issue_year(&passport),
        valid_expiration_year(&passport),
        valid_height(&passport),
        valid_hair_color(&passport),
        valid_eye_color(&passport),
        valid_passport_id(&passport),
    ]
    .into_iter()
    .filter(|valid| valid.to_owned())
    .count()
        == 7
}

pub fn main(input: &Vec<Vec<(String, String)>>) -> Result<usize> {
    Ok(input
        .iter()
        .map(|row| {
            let mut passport = HashMap::new();
            for (key, value) in row {
                passport.insert(key.to_string(), value.to_string());
            }
            passport.to_owned()
        })
        .filter(|passport| valid(passport))
        .collect::<Vec<Passport>>()
        .len())
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    use super::main;

    const EXAMPLE_DATA: &'static str = "
        eyr:1972 cid:100 hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926
        
        iyr:2019 hcl:#602927 eyr:1967 hgt:170cm ecl:grn pid:012533040 byr:1946
        
        hcl:dab227 iyr:2012 ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277
        
        hgt:59cm ecl:zzz eyr:2038 hcl:74454a iyr:2023 pid:3556412378 byr:2007
        
        pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f
        
        eyr:2029 ecl:blu cid:129 byr:1989 iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm
        
        hcl:#888785 hgt:164cm byr:2001 iyr:2015 cid:88 pid:545766238 ecl:hzl eyr:2022
        
        iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
    ";

    #[test]
    fn should_get_the_same_amount_of_valid_passports_as_example() {
        assert_eq!(main(&parse_input(&EXAMPLE_DATA)).unwrap(), 4);
    }
}
