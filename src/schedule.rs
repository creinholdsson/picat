use chrono::prelude::*;
pub struct Schedule {
    times: Vec<Occasion>,
}

pub struct Occasion {
    pub time: DateTime<Local>,
    pub enabled_weekdays: Vec<Weekday>,
    pub opened_time_servo1: u64,
    pub opened_time_servo2: u64
}

impl Occasion {
    pub fn is_enabled(&self, weekday: Weekday) -> bool {
        self.enabled_weekdays
            .iter()
            .filter(|&x| *x == weekday)
            .count()
            > 0
    }
}

impl Schedule {
    pub fn new() -> Schedule {
        Schedule {
            times: Vec::<Occasion>::new(),
        }
    }

    pub fn push(&mut self, time: Occasion) {
        self.times.push(time);
    }

    pub fn contains(&self, time: DateTime<Local>) -> Option<&Occasion> {
        for elem in self.times.iter() {
            if elem.is_enabled(time.weekday())
                && elem.time.hour() == time.hour()
                && elem.time.minute() == time.minute()
            {
                return Some(elem);
            }
        }
        None
    }

    pub fn get_times(&self) -> &Vec<Occasion> {
        &self.times
    }
}

#[test]
fn contains_true() {
    let mut schedule = Schedule::new();
    schedule.push(Occasion {
        time: Local.ymd(1970, 1, 1).and_hms(7, 30, 0),
        enabled_weekdays: vec![Weekday::Thu],
        opened_time_servo1: 300,
        opened_time_servo2: 300,
    });

    assert_eq!(
        schedule.contains(Local.ymd(1970, 1, 1).and_hms(7, 30, 10)).is_some(), 
        true);
}

#[test]
fn contains_false() {
    let mut schedule = Schedule::new();
    schedule.push(Occasion {
        time: Local.ymd(1970, 1, 1).and_hms(7, 30, 0),
        enabled_weekdays: vec![Weekday::Mon, Weekday::Tue],
        opened_time_servo1: 300,
        opened_time_servo2: 300
    });

    assert_eq!(
        schedule.contains(Local.ymd(1970, 1, 1).and_hms(7, 31, 0)).is_some(),
        false);
}

#[test]
fn occasion_enabled_true() {
    let occasion = Occasion {
        time: Local.ymd(1970, 1, 1).and_hms(7, 30, 0),
        enabled_weekdays: vec![Weekday::Mon],
        opened_time_servo1: 300,
        opened_time_servo2: 300
    };

    assert_eq!(occasion.is_enabled(Weekday::Mon), true);
}

#[test]
fn occasion_enabled_false() {
    let occasion = Occasion {
        time: Local.ymd(1970, 1, 1).and_hms(7, 30, 0),
        enabled_weekdays: vec![Weekday::Mon],
        opened_time_servo1: 300,
        opened_time_servo2: 300
    };

    assert_eq!(occasion.is_enabled(Weekday::Tue), false);
}