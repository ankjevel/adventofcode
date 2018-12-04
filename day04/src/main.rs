extern crate regex;

mod record;

use record::Record;
use std::io;

fn main() -> io::Result<()> {
    println!("Hello, world!");

    Ok(())
}

fn parse_input(string: &str) -> Vec<Record> {
    let mut data = string
        .lines()
        .map(|string| string.trim())
        .filter(|string| !string.is_empty())
        .map(|string| Record::new(&string))
        .collect::<Vec<Record>>();

    data.sort_by_key(|record| record.time);

    data
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &'static str = "
[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up
    ";

    #[test]
    fn id_multiplied_by_minute() {
        let data = parse_input(&EXAMPLE_DATA);

        for line in data {
            println!(
                "
year: {}
month: {}
day: {}
hour: {}
minute: {}",
                &line.time.year,
                &line.time.month,
                &line.time.day,
                &line.time.hour,
                &line.time.minute
            );
        }

        assert_eq!(10 * 24, 240)
    }
}
