use ::day_15::{parse_input, repair_droid::RepairDroid};
use day_09::program::run;

fn main() {
    let input = parse_input(&include_str!("../../input/day_15"));
    println!("part_01: {}", part_01(&input));
}

fn part_01(input: &Vec<i64>) -> usize {
    run(&input, |reciever, sender| -> usize {
        RepairDroid::new(reciever, sender).run()
    })
}
