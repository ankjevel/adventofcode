use std::io::Result;

use crate::Input;

#[derive(Debug, Eq, PartialEq)]
enum Seat {
    Empty,
    Taken,
}

use Seat::*;

fn get_seats(input: &Input) -> Vec<Vec<Seat>> {
    let mut seats: Vec<Vec<_>> = (0..=127)
        .map(|_| (0..=7).map(|_| Empty).collect())
        .collect();

    input.iter().for_each(|string| {
        let chars = string.chars().collect::<Vec<_>>();
        let row = &chars[0..7]
            .into_iter()
            .fold(0..=127, |range, letter| {
                let min = range.to_owned().min().unwrap();
                let max = range.to_owned().max().unwrap();
                let range = (max - min) / 2;
                match letter.to_owned() {
                    'F' => min..=min + range,
                    'B' => max - range..=max,
                    _ => min..=max,
                }
            })
            .min()
            .unwrap()
            .to_owned();
        let column = &chars[7..]
            .into_iter()
            .fold(0..=7, |range, letter| {
                let min = range.to_owned().min().unwrap();
                let max = range.to_owned().max().unwrap();
                let range = (max - min) / 2;
                match letter.to_owned() {
                    'L' => min..=min + range,
                    'R' => max - range..=max,
                    _ => min..=max,
                }
            })
            .min()
            .unwrap()
            .to_owned();
        match seats.get_mut(row.to_owned()) {
            Some(row) => match row.get_mut(column.to_owned()) {
                Some(seat) => *seat = Taken,
                _ => {}
            },
            _ => {}
        }
    });

    seats
}

pub fn main(input: &Input) -> Result<usize> {
    let seats = get_seats(input);
    let mut result = 0;
    let mut rows = seats.iter().to_owned().enumerate();
    'main: while let Some((i, row)) = rows.next() {
        let mut iter = row.iter().to_owned().enumerate();
        while let Some((j, seat)) = iter.next() {
            if seat != &Empty || (1..6).contains(&j) == false {
                continue;
            }
            let prev = row.get(j - 1).unwrap();
            let next = row.get(j + 1).unwrap();

            if prev == &Taken && next == &Taken {
                result = (i * 8) + j;
                break 'main;
            }
        }
    }

    Ok(result)
}
