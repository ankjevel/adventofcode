use std::collections::HashMap;

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

pub type Passport = HashMap<String, String>;

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

pub fn valid(passport: &Passport) -> bool {
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
