use std::collections::HashMap;
use std::io;
use std::iter::Iterator;

use record::{Action, Record};

pub struct Guard {
    pub minutes_slept: [u32; 60],
    pub sleep: u32,
    pub id: u32,
}

pub fn guards(records: &Vec<Record>) -> io::Result<Vec<Guard>> {
    let mut guards_map: HashMap<u32, Guard> = HashMap::new();

    let mut guard_id = 0;
    let mut sleep = 0;

    records.iter().for_each(|&record| match record.action {
        Action::BeginShift(id) => {
            guard_id = id;
            if !guards_map.contains_key(&guard_id) {
                guards_map.insert(
                    guard_id,
                    Guard {
                        minutes_slept: [0; 60],
                        sleep: 0,
                        id,
                    },
                );
            }
        }
        Action::Sleep => {
            sleep = record.time.minutes();
        }
        Action::Woke => {
            let woke = record.time.minutes();
            let snore = woke - sleep;

            guards_map.entry(guard_id).and_modify(|guard| {
                guard.sleep += snore;

                (sleep..woke)
                    .into_iter()
                    .for_each(|i| guard.minutes_slept[i as usize] += 1);
            });
        }
    });

    Ok(guards_map.into_iter().map(|(_id, guard)| guard).collect())
}
