use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
// use serde_json::Result;
use chrono::prelude::*;

#[derive(Serialize, Deserialize)]
struct PersistedSchedule {
    enabled: bool,
    time: String,
    enabled_weekdays: Vec<u32>,
    opened_time_servo1: u64,
    opened_time_servo2: u64
}

fn weekday_to_int(weekday: Weekday) -> u32 {
    match weekday {
        Weekday::Mon => 1,
        Weekday::Tue => 2,
        Weekday::Wed => 3,
        Weekday::Thu => 4,
        Weekday::Fri => 5,
        Weekday::Sat => 6,
        Weekday::Sun => 7,
    }
}

fn int_to_weekday(int: u32) -> Weekday {
    match int {
        1 => Weekday::Mon,
        2 => Weekday::Tue,
        3 => Weekday::Wed,
        4 => Weekday::Thu,
        5 => Weekday::Fri,
        6 => Weekday::Sat,
        _ => Weekday::Sun,
    }
}

pub fn serialize(schedule: &crate::schedule::Schedule) -> String {
    let mut persisted_schedule = Vec::<PersistedSchedule>::new();
    for time in schedule.get_times().iter() {
        let mut enabled_weekdays_int = Vec::<u32>::new();
        for wkday in time.enabled_weekdays.iter() {
            enabled_weekdays_int.push(weekday_to_int(*wkday));
        }

        persisted_schedule.push(PersistedSchedule {
            enabled: true,
            time: time.time.to_rfc3339(),
            enabled_weekdays: enabled_weekdays_int,
            opened_time_servo1: time.opened_time_servo1,
            opened_time_servo2: time.opened_time_servo2
        });
    }
    serde_json::to_string(&persisted_schedule).unwrap()
}

pub fn deserialize(json: &str, schedule: &mut crate::schedule::Schedule) -> Result<(), String> {
    let schedule_res: serde_json::Result<Vec<PersistedSchedule>> = serde_json::from_str(json);
    match schedule_res {
        Ok(x) => {
            for sched in x.iter() {
                let mut enabled_weekdays = Vec::<Weekday>::new();
                for wkday in sched.enabled_weekdays.iter() {
                    enabled_weekdays.push(int_to_weekday(*wkday));
                }
                let timestamp = sched.time.parse::<DateTime<Local>>().unwrap();
                schedule.push(crate::schedule::Occasion {
                    time: timestamp,
                    enabled_weekdays,
                    opened_time_servo1: sched.opened_time_servo1,
                    opened_time_servo2: sched.opened_time_servo2
                });
            }
            Ok(())
        }
        Err(_) => Err(String::from("Error deserializing")),
    }
}

pub fn save(file_path: &str, schedule: &crate::schedule::Schedule) -> Result<(), std::io::Error> {
    let mut file = File::create(file_path)?;
    file.write_all(&serialize(schedule).into_bytes())?;
    Ok(())
}

pub fn load(file_path: &str) -> Result<crate::schedule::Schedule, std::io::Error> {
    let mut file = File::open(file_path)?;
    let mut file_content = String::new();
    match file.read_to_string(&mut file_content) {
        Ok(_) => {
            let mut schedule = crate::schedule::Schedule::new();
            deserialize(file_content.as_str(), &mut schedule).unwrap();
            Ok(schedule)
        }
        Err(x) => Err(x),
    }
}

#[test]
fn test_serialization() -> Result<(), ()> {
    let mut schedule = crate::schedule::Schedule::new();

    schedule.push(crate::schedule::Occasion {
        time: Local::now(),
        enabled_weekdays: vec![Weekday::Mon, Weekday::Tue],
        opened_time_servo1: 300,
        opened_time_servo2: 300
    });

    let json = serialize(&schedule);
    let mut new_schedule = crate::schedule::Schedule::new();
    let sched = deserialize(json.as_str(), &mut new_schedule);

    println!("json: {}", json);
    assert_eq!(1, new_schedule.get_times().len());
    Ok(())
}

#[test]
fn test_save_load() -> Result<(), (std::io::Error)> {
    let mut schedule = crate::schedule::Schedule::new();

    schedule.push(crate::schedule::Occasion {
        time: Local::now(),
        enabled_weekdays: vec![Weekday::Mon, Weekday::Tue],
        opened_time_servo1: 300,
        opened_time_servo2: 300
    });

    save("test.json", &schedule);
    let loaded_schedule = load("test.json")?;

    assert_eq!(1, loaded_schedule.get_times().len());
    Ok(())
}
