use chrono::prelude::*;

use std::error::Error;
use std::thread;
use std::time::Duration;
use std::env;

use rppal::pwm::{Channel, Polarity, Pwm};

mod schedule;
mod servo;

const PERIOD_MS: u64 = 20;
const PULSE_OPEN_US: u64 = 1850;
const PULSE_CLOSED_US: u64 = 2400;
const PULSE_PASSED_US: u64 = 2650;
const PULSE_CLOSED_1_US: u64 = 1440;
const PULSE_OPEN_1_US: u64 = 860;
const PULSE_PASSED_1_US: u64 = 1700;

fn feed_cat(servo: &servo::Servo, feed_time: u64) -> Result<(), Box<dyn Error>> {
    match servo.pwm {
        Some(pwm) => {
            pwm.set_pulse_width(Duration::from_micros(servo.pulse_open))?;
            thread::sleep(Duration::from_millis(feed_time));

            for _ in 1..4 {
                pwm.set_pulse_width(Duration::from_micros(servo.pulse_passed))?;
                thread::sleep(Duration::from_millis(150));
                pwm.set_pulse_width(Duration::from_micros(servo.pulse_closed))?;
            }
        }
        None => {
            println!("Was gonna feed the cat!");
        }
    }
    Ok(())
}

fn main_feeder_loop() -> Result<(), Box<dyn Error>> {
    let mut schedule = schedule::Schedule::new();
    schedule.push(schedule::Occasion {
        time: Local.ymd(1970, 1, 1).and_hms(4, 25, 0),
        enabled_weekdays: vec![
            Weekday::Mon,
            Weekday::Tue,
            Weekday::Wed,
            Weekday::Thu,
            Weekday::Fri,
            Weekday::Sat,
            Weekday::Sun,
        ],
    });
    schedule.push(schedule::Occasion {
        time: Local.ymd(1970, 1, 1).and_hms(7, 30, 0),
        enabled_weekdays: vec![
            Weekday::Mon,
            Weekday::Tue,
            Weekday::Wed,
            Weekday::Thu,
            Weekday::Fri,
            Weekday::Sat,
            Weekday::Sun,
        ],
    });
    schedule.push(schedule::Occasion {
        time: Local.ymd(1970, 1, 1).and_hms(11, 30, 0),
        enabled_weekdays: vec![
            Weekday::Mon,
            Weekday::Tue,
            Weekday::Wed,
            Weekday::Thu,
            Weekday::Fri,
            Weekday::Sat,
            Weekday::Sun,
        ],
    });
    schedule.push(schedule::Occasion {
        time: Local.ymd(1970, 1, 1).and_hms(15, 10, 0),
        enabled_weekdays: vec![
            Weekday::Mon,
            Weekday::Tue,
            Weekday::Wed,
            Weekday::Thu,
            Weekday::Fri,
            Weekday::Sat,
            Weekday::Sun,
        ],
    });
    schedule.push(schedule::Occasion {
        time: Local.ymd(1970, 1, 1).and_hms(19, 30, 0),
        enabled_weekdays: vec![
            Weekday::Mon,
            Weekday::Tue,
            Weekday::Wed,
            Weekday::Thu,
            Weekday::Fri,
            Weekday::Sat,
            Weekday::Sun,
        ],
    });

    loop {
        let local = Local::now();

        if schedule.contains(local) {
            let pwm = Pwm::with_period(
                Channel::Pwm0,
                Duration::from_millis(PERIOD_MS),
                Duration::from_micros(PULSE_CLOSED_US),
                Polarity::Normal,
                true,
            );

            // let pwm1 = Pwm::with_period(
            //     Channel::Pwm1,
            //     Duration::from_millis(PERIOD_MS),
            //     Duration::from_micros(PULSE_CLOSED_1_US),
            //     Polarity::Normal,
            //     true,
            // );

            let servo1 = match pwm {
                Ok(ref p) => servo::Servo {
                    pulse_closed: PULSE_CLOSED_US,
                    pulse_open: PULSE_OPEN_US,
                    pulse_passed: PULSE_PASSED_US,
                    pwm: Some(p),
                },
                Err(_) => {
                    println!("Failed to create servo1, using dummy");
                    servo::Servo {
                        pulse_closed: PULSE_CLOSED_US,
                        pulse_open: PULSE_OPEN_US,
                        pulse_passed: PULSE_PASSED_US,
                        pwm: None,
                    }
                }
            };

            // let servo2 = match pwm1 {
            //     Ok(ref p) => servo::Servo {
            //         pulse_closed: PULSE_CLOSED_1_US,
            //         pulse_open: PULSE_OPEN_1_US,
            //         pulse_passed: PULSE_PASSED_1_US,
            //         pwm: Some(p),
            //     },
            //     Err(_) => {
            //         println!("Failed to create servo2, using dummy");
            //         servo::Servo {
            //             pulse_closed: PULSE_CLOSED_US,
            //             pulse_open: PULSE_OPEN_US,
            //             pulse_passed: PULSE_PASSED_1_US,
            //             pwm: None,
            //         }
            //     }
            // };

            // match feed_cat(&servo2, 3100 / schedule.occasions(local.weekday()) as u64) {
            //     // 2900 tot
            //     Ok(_) => println!("Fed the cat with servo 2"),
            //     Err(_) => println!("Failed to feed the cat with servo 2"),
            // }
            // thread::sleep(Duration::from_millis(3000));

            match feed_cat(&servo1, 2300 / schedule.occasions(local.weekday()) as u64) {
                // 2150 tot
                Ok(_) => println!("Fed the cat with servo 1"),
                Err(_) => println!("Failed to feed the cat with servo 1"),
            }
            thread::sleep(Duration::from_millis(60000)) // make sure we never hit it the same minute
        }
        println!("Now {}", local);

        thread::sleep(Duration::from_millis(30000))
    }
}

fn test_servo_loop() -> Result<(), Box<dyn Error>> {
    let pwm = Pwm::with_period(
                Channel::Pwm0,
                Duration::from_millis(PERIOD_MS),
                Duration::from_micros(PULSE_CLOSED_US),
                Polarity::Normal,
                true,
            );

            let pwm1 = Pwm::with_period(
                Channel::Pwm1,
                Duration::from_millis(PERIOD_MS),
                Duration::from_micros(PULSE_CLOSED_1_US),
                Polarity::Normal,
                true,
            );

            let servo1 = match pwm {
                Ok(ref p) => servo::Servo {
                    pulse_closed: PULSE_CLOSED_US,
                    pulse_open: PULSE_OPEN_US,
                    pulse_passed: PULSE_PASSED_US,
                    pwm: Some(p),
                },
                Err(_) => {
                    println!("Failed to create servo1, using dummy");
                    servo::Servo {
                        pulse_closed: PULSE_CLOSED_US,
                        pulse_open: PULSE_OPEN_US,
                        pulse_passed: PULSE_PASSED_US,
                        pwm: None,
                    }
                }
            };

            let servo2 = match pwm1 {
                Ok(ref p) => servo::Servo {
                    pulse_closed: PULSE_CLOSED_1_US,
                    pulse_open: PULSE_OPEN_1_US,
                    pulse_passed: PULSE_PASSED_1_US,
                    pwm: Some(p),
                },
                Err(_) => {
                    println!("Failed to create servo2, using dummy");
                    servo::Servo {
                        pulse_closed: PULSE_CLOSED_US,
                        pulse_open: PULSE_OPEN_US,
                        pulse_passed: PULSE_PASSED_1_US,
                        pwm: None,
                    }
                }
            };
    
    loop {
        match feed_cat(&servo2, 1000) {
            // 2900 tot
            Ok(_) => println!("Fed the cat with servo 2"),
            Err(_) => println!("Failed to feed the cat with servo 2"),
        }
        thread::sleep(Duration::from_millis(3000));

        match feed_cat(&servo1, 1000) {
            // 2150 tot
            Ok(_) => println!("Fed the cat with servo 1"),
            Err(_) => println!("Failed to feed the cat with servo 1"),
        }
        thread::sleep(Duration::from_millis(60000))
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => {
            main_feeder_loop();
        },
        _ => {
            test_servo_loop();
        }
    }
}
