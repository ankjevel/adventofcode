use std::cmp::{
    Eq,
    Ordering::{self, Equal, Greater, Less},
};

#[derive(Debug)]
pub struct Angle(f64);

impl Angle {
    pub fn new(input: &f64) -> Angle {
        Angle(*input)
    }
}

impl PartialEq for Angle {
    fn eq(&self, other: &Self) -> bool {
        approx_eq!(f64, self.0, other.0, ulps = 2)
    }
}

impl Eq for Angle {}

impl Ord for Angle {
    fn cmp(&self, other: &Angle) -> Ordering {
        if self.0 > other.0 {
            Greater
        } else if self.0 < other.0 {
            Less
        } else {
            Equal
        }
    }
}

impl PartialOrd for Angle {
    fn partial_cmp(&self, other: &Angle) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
