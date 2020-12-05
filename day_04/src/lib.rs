#[macro_use(lazy_static)]
extern crate lazy_static;
extern crate regex;

pub mod part_01;
pub mod part_02;

pub fn parse_input(input: &str) -> Vec<Vec<(String, String)>> {
    input
        .lines()
        .map(|string| string.trim())
        .into_iter()
        .fold(vec![vec![]], |mut list, string| {
            if string.is_empty() {
                list.push(vec![]);
            } else {
                let last = list.last_mut().unwrap();
                last.push(string.to_string());
            }
            list
        })
        .into_iter()
        .filter(|list| list.len() > 0)
        .map(|passport| {
            passport
                .join(" ")
                .split(" ")
                .map(|part| {
                    let mut split = part.split(":").into_iter();
                    (
                        split.next().unwrap().to_owned(),
                        split.next().unwrap().to_owned(),
                    )
                })
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

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
    fn it_parses_the_example_input() {
        assert_eq!(parse_input(&EXAMPLE_DATA).len(), 4);
    }
}
