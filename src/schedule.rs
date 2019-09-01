use chrono::prelude::*;
pub struct Schedule {
    times: Vec<Occasion>,
}

pub struct Occasion {
    pub time: DateTime<Local>,
    pub enabled_weekdays: Vec<Weekday>,
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

    pub fn contains(&self, time: DateTime<Local>) -> bool {
        for elem in self.times.iter() {
            if elem.is_enabled(time.weekday())
                && elem.time.hour() == time.hour()
                && elem.time.minute() == time.minute()
            {
                return true;
            }
        }
        false
    }

    pub fn occasions(&self, weekday: Weekday) -> i32 {
        self.times
            .iter()
            .filter(|&x| (*x).is_enabled(weekday))
            .count() as i32
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
    });

    assert_eq!(
        schedule.contains(Local.ymd(1970, 1, 1).and_hms(7, 30, 10)),
        true
    );
}

#[test]
fn contains_false() {
    let mut schedule = Schedule::new();
    schedule.push(Occasion {
        time: Local.ymd(1970, 1, 1).and_hms(7, 30, 0),
        enabled_weekdays: vec![Weekday::Mon, Weekday::Tue],
    });

    assert_eq!(
        schedule.contains(Local.ymd(1970, 1, 1).and_hms(7, 31, 0)),
        false
    );
}

#[test]
fn occasion_enabled_true() {
    let occasion = Occasion {
        time: Local.ymd(1970, 1, 1).and_hms(7, 30, 0),
        enabled_weekdays: vec![Weekday::Mon],
    };

    assert_eq!(occasion.is_enabled(Weekday::Mon), true);
}

#[test]
fn occasion_enabled_false() {
    let occasion = Occasion {
        time: Local.ymd(1970, 1, 1).and_hms(7, 30, 0),
        enabled_weekdays: vec![Weekday::Mon],
    };

    assert_eq!(occasion.is_enabled(Weekday::Tue), false);
}

#[test]
fn schedule_occasion_count() {
    let mut schedule = Schedule::new();
    schedule.push(Occasion {
        time: Local.ymd(1970, 1, 1).and_hms(7, 30, 0),
        enabled_weekdays: vec![Weekday::Mon, Weekday::Tue],
    });
    schedule.push(Occasion {
        time: Local.ymd(1970, 1, 1).and_hms(8, 30, 0),
        enabled_weekdays: vec![Weekday::Mon, Weekday::Tue],
    });
    schedule.push(Occasion {
        time: Local.ymd(1970, 1, 1).and_hms(8, 30, 0),
        enabled_weekdays: vec![Weekday::Fri],
    });

    assert_eq!(schedule.occasions(Weekday::Mon), 2);

    assert_eq!(schedule.occasions(Weekday::Tue), 2);

    assert_eq!(schedule.occasions(Weekday::Wed), 0);

    assert_eq!(schedule.occasions(Weekday::Thu), 0);

    assert_eq!(schedule.occasions(Weekday::Fri), 1);

    assert_eq!(schedule.occasions(Weekday::Sat), 0);

    assert_eq!(schedule.occasions(Weekday::Sun), 0);

    assert_eq!(2400 / schedule.occasions(Weekday::Mon), 1200)
}
