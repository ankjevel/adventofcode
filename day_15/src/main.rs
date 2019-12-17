use ::day_15::{parse_input, repair_droid::RepairDroid};
use day_09::program::run;

fn main() {
    let input = parse_input(&include_str!("../../input/day_15"));

    run(&input, |reciever, sender| -> usize {
        RepairDroid::new(reciever, sender).run()
    });
}
