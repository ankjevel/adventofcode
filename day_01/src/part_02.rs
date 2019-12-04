use std::io;

fn calculate_fuel(input: i32, results: &mut Vec<i32>) {
    let fuel = ((input as f64 / 3.0).floor() as i32) - 2;

    if fuel <= 0 {
        return;
    }

    &results.push(fuel);

    calculate_fuel(fuel, results)
}

fn required_fuel(input: i32) -> i32 {
    let mut results = Vec::new();

    calculate_fuel(input, &mut results);

    results.iter().sum::<i32>()
}

pub fn main(input: &Vec<i32>) -> io::Result<i32> {
    let fuel: Vec<i32> = input
        .iter()
        .map(|&mass| required_fuel(mass.clone()))
        .collect();

    Ok(fuel.iter().sum::<i32>())
}
