use std::fmt::{Display, Formatter, Result};
const MINUTES_DAY: i32 = 1440;
const MINUTES_HOUR: i32 = 60;

#[derive(Debug, PartialEq)]
pub struct Clock {
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self {
            minutes: ((hours * MINUTES_HOUR) + minutes).rem_euclid(MINUTES_DAY)
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(0, self.minutes + minutes)
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f,"{:02}:{:02}", self.minutes.div_euclid(MINUTES_HOUR), self.minutes.rem_euclid(MINUTES_HOUR))
    }
}