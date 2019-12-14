pub type Item = (String, u8);

fn split(input: String) -> Item {
    let mut iter = input.split(" ").map(str::trim).map(str::to_owned);
    let quantity = iter.next().unwrap();
    let chemical = iter.next().unwrap();
    (chemical.to_owned(), quantity.parse().unwrap())
}

pub fn parse_input(input: &str) -> Vec<(Item, Vec<Item>)> {
    input
        .lines()
        .map(str::trim)
        .filter(|string| !string.is_empty())
        .map(|part| {
            let mut iter = part.split("=>").map(str::trim);

            let input_chemicals = iter.next().unwrap();
            let output_chemical = split(iter.next().unwrap().to_owned());

            let input_chemicals = input_chemicals
                .split(", ")
                .map(str::trim)
                .map(str::to_owned)
                .map(split)
                .collect();

            (output_chemical, input_chemicals)
        })
        .collect()
}
