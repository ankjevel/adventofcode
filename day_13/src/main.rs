use ::day_13::{get_instructions, parse_input};

fn part_01(input: &Vec<i64>) {
    let instructions = get_instructions(input);

    let chunked: Vec<(i64, i64, i64)> = instructions
        .chunks(3)
        .into_iter()
        .map(|instructions| {
            let mut iter = instructions.into_iter();
            let x = iter.next().unwrap();
            let y = iter.next().unwrap();
            let id = iter.next().unwrap();
            (x.to_owned(), y.to_owned(), id.to_owned())
        })
        .collect::<Vec<_>>();

    println!("chunked: {:?}", chunked);
}

fn main() {
    let input = parse_input(&include_str!("../../input/day_13"));
    part_01(&input);
}
