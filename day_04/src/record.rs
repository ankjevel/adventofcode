use regex::Regex;
use std::str::FromStr;

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Debug)]
pub struct Time {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
}

impl Time {
    pub fn minutes(&self) -> u32 {
        let hours = self.hour as u32;
        let minutes = self.minute as u32;

        (hours * 60) + minutes
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Action {
    BeginShift(u32),
    Sleep,
    Woke,
}

#[derive(Clone, Copy)]
pub struct Record {
    pub action: Action,
    pub time: Time,
}

impl Record {
    pub fn new(string: &str) -> Self {
        let date_re = Regex::new(r"(\d{4})-(\d{2})-(\d{2}) (\d{2}):(\d{2})").unwrap();
        let id_re = Regex::new(r"#(\d*)").unwrap();

        let dates = date_re.captures(&string).unwrap();

        let year = dates
            .get(1)
            .map_or(0, |s| u16::from_str(s.as_str()).unwrap());
        let month = dates
            .get(2)
            .map_or(0, |s| u8::from_str(s.as_str()).unwrap());
        let day = dates
            .get(3)
            .map_or(0, |s| u8::from_str(s.as_str()).unwrap());
        let hour = dates
            .get(4)
            .map_or(0, |s| u8::from_str(s.as_str()).unwrap());
        let minute = dates
            .get(5)
            .map_or(0, |s| u8::from_str(s.as_str()).unwrap());

        let time = Time {
            year,
            month,
            day,
            hour,
            minute,
        };

        Record {
            time,
            action: if string.contains("wakes") {
                Action::Woke
            } else if string.contains("asleep") {
                Action::Sleep
            } else {
                let guard_id = id_re
                    .captures(&string)
                    .unwrap()
                    .get(1)
                    .map_or(0, |s| u32::from_str(s.as_str()).unwrap());

                Action::BeginShift(guard_id)
            },
        }
    }
}
