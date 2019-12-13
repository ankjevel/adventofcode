use ::day_12::{gravity, new_moon, parse_input, Moon};
use num_integer::lcm;
use std::mem::replace;

fn part_01(input: &Vec<Moon>, steps: usize) -> i64 {
    let mut moons = input.clone();
    let mut new_moons = vec![];
    for _ in 0..steps {
        for (i, moon) in moons.iter().enumerate() {
            let mut ddx = 0;
            let mut ddy = 0;
            let mut ddz = 0;
            for (j, other) in moons.iter().enumerate() {
                if i == j {
                    continue;
                }
                let (x, y, z) = moon.gravity(&other);
                ddx += x;
                ddy += y;
                ddz += z;
            }
            new_moons.push(new_moon(&moon, &(ddx, ddy, ddz)))
        }
        moons = replace(&mut new_moons, vec![]);
    }
    moons.iter().map(Moon::total_energy).sum()
}

type SimplifiedMoon = (i64, i64);

fn part_02(input: &Vec<Moon>) -> u128 {
    let to_id = |input: &SimplifiedMoon| -> String {
        "pos=".to_string() + &input.0.to_string() + &",vel=" + &input.1.to_string()
    };

    let to_hash = |input: Vec<SimplifiedMoon>| -> String {
        input.iter().map(to_id).collect::<Vec<String>>().join("|")
    };

    let run_simulation = |input: Vec<SimplifiedMoon>| -> u128 {
        let mut moons = input.clone();
        let mut new_moons = vec![];
        let mut steps = 0;

        let initial_state = to_hash(input.clone());

        'outer: loop {
            steps += 1;

            for (i, moon) in moons.iter().enumerate() {
                let mut energy = 0;
                for (j, other) in moons.iter().enumerate() {
                    if i == j {
                        continue;
                    }
                    energy += gravity(moon.0, other.0);
                }
                let (position, velocity) = moon;
                let velocity = velocity + energy;
                let position = position + velocity;
                new_moons.push((position.to_owned(), velocity.to_owned()))
            }

            if to_hash(new_moons.clone()) == initial_state {
                break 'outer;
            };

            moons = replace(&mut new_moons, vec![]);
        }

        steps
    };

    let x = run_simulation(
        input
            .clone()
            .iter()
            .map(|moon| (moon.position.0, 0))
            .collect(),
    );
    let y = run_simulation(
        input
            .clone()
            .iter()
            .map(|moon| (moon.position.1, 0))
            .collect(),
    );
    let z = run_simulation(
        input
            .clone()
            .iter()
            .map(|moon| (moon.position.2, 0))
            .collect(),
    );

    lcm(lcm(x, y), z)
}

fn main() {
    let input = parse_input(&include_str!("../../input/day_12"));
    println!("part_01: {}", part_01(&input, 1_000));
    println!("part_02: {}", part_02(&input));
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;

    const EXAMPLE_DATA_01: &'static str = "
        <x=-1, y=0, z=2>
        <x=2, y=-10, z=-7>
        <x=4, y=-8, z=8>
        <x=3, y=5, z=-1>
    ";

    const EXAMPLE_DATA_02: &'static str = "
        <x=-8, y=-10, z=0>
        <x=5, y=5, z=10>
        <x=2, y=-7, z=3>
        <x=9, y=-8, z=-3>
    ";

    #[test]
    fn it_gets_the_examples_on_part_01_right() {
        assert_eq!(part_01(&parse_input(&EXAMPLE_DATA_01), 10), 179);
        assert_eq!(part_01(&parse_input(&EXAMPLE_DATA_02), 100), 1940);
    }

    #[test]
    fn it_gets_the_examples_on_part_02_right() {
        assert_eq!(part_02(&parse_input(&EXAMPLE_DATA_01)), 2772);
        assert_eq!(part_02(&parse_input(&EXAMPLE_DATA_02)), 4686774924);
    }
}
