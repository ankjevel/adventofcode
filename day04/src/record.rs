use regex::Regex;
use std::str::FromStr;

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub struct Time {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
}

#[derive(Clone, Copy)]
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

        let year = u16::from_str(&dates[1]).unwrap_or(0);
        let month = u8::from_str(&dates[2]).unwrap_or(0);
        let day = u8::from_str(&dates[3]).unwrap_or(0);
        let hour = u8::from_str(&dates[4]).unwrap_or(0);
        let minute = u8::from_str(&dates[5]).unwrap_or(0);

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
                let capture = &id_re.captures(&string).unwrap();
                let guard_id = u32::from_str(&capture[1]).unwrap();

                Action::BeginShift(guard_id)
            },
        }
    }
}
