use chrono::prelude::*;
pub struct Schedule {
    times: Vec<DateTime<Local>>
}

impl Schedule {
    pub fn new() -> Schedule {
        Schedule { times: Vec::<DateTime<Local>>::new() }
    }

    pub fn push(&mut self, time: DateTime<Local>) {
        self.times.push(time);
    }

    pub fn contains(&self, time: DateTime<Local>) -> bool {
        for elem in self.times.iter() {
            if elem.hour() == time.hour() && elem.minute() == time.minute() {
                return true;
            }
        }
        return false;
    }
}

#[test]
fn contains_true() {
    let mut schedule = Schedule::new();
    schedule.push(Local.ymd(1970, 1, 1).and_hms(7, 30, 0));

    assert_eq!(schedule.contains(Local.ymd(1970, 1, 1).and_hms(7, 30, 10)), true);
}

#[test]
fn contains_false() {
    let mut schedule = Schedule::new();
    schedule.push(Local.ymd(1970, 1, 1).and_hms(7, 30, 0));

    assert_eq!(schedule.contains(Local.ymd(1970, 1, 1).and_hms(7, 31, 0)), false);
}
