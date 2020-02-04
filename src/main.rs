use chrono::prelude::*;

use std::env;
use std::error::Error;
use std::thread;
use std::time::Duration;

// use actix_web::{web, App, HttpServer, Responder};
use rppal::pwm::{Channel, Polarity, Pwm};

mod persistant_schedule_storage;
mod schedule;
mod servo;

const PERIOD_MS: u64 = 20;
const PULSE_OPEN_US: u64 = 1850;
const PULSE_CLOSED_US: u64 = 2400;
const PULSE_PASSED_US: u64 = 2650;
const PULSE_CLOSED_1_US: u64 = 1440;
const PULSE_OPEN_1_US: u64 = 860;
const PULSE_PASSED_1_US: u64 = 1700;
const SCHEDULE_FILE_NAME: &str = "schedule.json";

fn feed_cat(servo: &servo::Servo, feed_time: u64) -> Result<(), Box<dyn Error>> {
    match servo.pwm {
        Some(pwm) => {
            pwm.set_pulse_width(Duration::from_micros(servo.pulse_open))?;
            thread::sleep(Duration::from_millis(feed_time));

            for _ in 1..4 {
                pwm.set_pulse_width(Duration::from_micros(servo.pulse_passed))?;
                thread::sleep(Duration::from_millis(200));
                pwm.set_pulse_width(Duration::from_micros(servo.pulse_closed))?;
                thread::sleep(Duration::from_millis(200));
            }
        }
        None => {
            println!("Was gonna feed the cat!");
        }
    }
    Ok(())
}

fn create_default_schedule(schedule: &mut schedule::Schedule) {
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
        opened_time_servo1: 320,
        opened_time_servo2: 280,
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
        opened_time_servo1: 320,
        opened_time_servo2: 280,
    });
    schedule.push(schedule::Occasion {
        time: Local.ymd(1970, 1, 1).and_hms(8, 30, 0),
        enabled_weekdays: vec![
            Weekday::Mon,
            Weekday::Tue,
            Weekday::Wed,
            Weekday::Thu,
            Weekday::Fri,
            Weekday::Sat,
            Weekday::Sun,
        ],
        opened_time_servo1: 320,
        opened_time_servo2: 280,
    });
    schedule.push(schedule::Occasion {
        time: Local.ymd(1970, 1, 1).and_hms(9, 30, 0),
        enabled_weekdays: vec![
            Weekday::Mon,
            Weekday::Tue,
            Weekday::Wed,
            Weekday::Thu,
            Weekday::Fri,
            Weekday::Sat,
            Weekday::Sun,
        ],
        opened_time_servo1: 320,
        opened_time_servo2: 280,
    });
    schedule.push(schedule::Occasion {
        time: Local.ymd(1970, 1, 1).and_hms(10, 30, 0),
        enabled_weekdays: vec![
            Weekday::Mon,
            Weekday::Tue,
            Weekday::Wed,
            Weekday::Thu,
            Weekday::Fri,
            Weekday::Sat,
            Weekday::Sun,
        ],
        opened_time_servo1: 320,
        opened_time_servo2: 280,
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
        opened_time_servo1: 320,
        opened_time_servo2: 280,
    });
    schedule.push(schedule::Occasion {
        time: Local.ymd(1970, 1, 1).and_hms(12, 30, 0),
        enabled_weekdays: vec![
            Weekday::Mon,
            Weekday::Tue,
            Weekday::Wed,
            Weekday::Thu,
            Weekday::Fri,
            Weekday::Sat,
            Weekday::Sun,
        ],
        opened_time_servo1: 320,
        opened_time_servo2: 280,
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
        opened_time_servo1: 320,
        opened_time_servo2: 280,
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
        opened_time_servo1: 320,
        opened_time_servo2: 280,
    });
    schedule.push(schedule::Occasion {
        time: Local.ymd(1970, 1, 1).and_hms(15, 30, 0),
        enabled_weekdays: vec![
            Weekday::Mon,
            Weekday::Tue,
            Weekday::Wed,
            Weekday::Thu,
            Weekday::Fri,
            Weekday::Sat,
            Weekday::Sun,
        ],
        opened_time_servo1: 320,
        opened_time_servo2: 280,
    });
    schedule.push(schedule::Occasion {
        time: Local.ymd(1970, 1, 1).and_hms(16, 30, 0),
        enabled_weekdays: vec![
            Weekday::Mon,
            Weekday::Tue,
            Weekday::Wed,
            Weekday::Thu,
            Weekday::Fri,
            Weekday::Sat,
            Weekday::Sun,
        ],
        opened_time_servo1: 320,
        opened_time_servo2: 280,
    });
    schedule.push(schedule::Occasion {
        time: Local.ymd(1970, 1, 1).and_hms(17, 30, 0),
        enabled_weekdays: vec![
            Weekday::Mon,
            Weekday::Tue,
            Weekday::Wed,
            Weekday::Thu,
            Weekday::Fri,
            Weekday::Sat,
            Weekday::Sun,
        ],
        opened_time_servo1: 320,
        opened_time_servo2: 280,
    });
    schedule.push(schedule::Occasion {
        time: Local.ymd(1970, 1, 1).and_hms(18, 30, 0),
        enabled_weekdays: vec![
            Weekday::Mon,
            Weekday::Tue,
            Weekday::Wed,
            Weekday::Thu,
            Weekday::Fri,
            Weekday::Sat,
            Weekday::Sun,
        ],
        opened_time_servo1: 320,
        opened_time_servo2: 280,
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
        opened_time_servo1: 320,
        opened_time_servo2: 280,
    });
    schedule.push(schedule::Occasion {
        time: Local.ymd(1970, 1, 1).and_hms(20, 30, 0),
        enabled_weekdays: vec![
            Weekday::Mon,
            Weekday::Tue,
            Weekday::Wed,
            Weekday::Thu,
            Weekday::Fri,
            Weekday::Sat,
            Weekday::Sun,
        ],
        opened_time_servo1: 320,
        opened_time_servo2: 280,
    });
}

fn main_feeder_loop() -> Result<(), Box<dyn Error>> {
    let mut schedule = schedule::Schedule::new();

    let created_default = match persistant_schedule_storage::load(SCHEDULE_FILE_NAME) {
        Ok(x) => {
            println!("Persisted schedule found, using it");
            schedule = x;
            false
        },
        Err(_) =>  { 
            println!("Persisted schedule doesnt exist, creating new");
            create_default_schedule(&mut schedule);
            true
        }
    };

    if created_default {
        match persistant_schedule_storage::save(SCHEDULE_FILE_NAME, &schedule) {
            Ok(_) => {},
            Err(_) => println!("Failed to persist schedule")
        }
    }

    loop {
        let local = Local::now();

        let result = schedule.contains(local);

        if result.is_some() {
            // let pwm1 = Pwm::with_period(
            //     Channel::Pwm1,
            //     Duration::from_millis(PERIOD_MS),
            //     Duration::from_micros(PULSE_CLOSED_1_US),
            //     Polarity::Normal,
            //     true,
            // );
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

            // match feed_cat(&servo2, result.unwrap().opened_time_servo2) {
            //     // 2900 tot
            //     Ok(_) => println!("Fed the cat with servo 2"),
            //     Err(_) => println!("Failed to feed the cat with servo 2"),
            // }

            // pwm1?.disable().unwrap_or(());
            // thread::sleep(Duration::from_millis(3000));

            let pwm = Pwm::with_period(
                Channel::Pwm0,
                Duration::from_millis(PERIOD_MS),
                Duration::from_micros(PULSE_CLOSED_US),
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
            match feed_cat(&servo1, result.unwrap().opened_time_servo1) {
                // 2150 tot
                Ok(_) => println!("Fed the cat with servo 1"),
                Err(_) => println!("Failed to feed the cat with servo 1"),
            }
            pwm?.disable().unwrap_or(());

            thread::sleep(Duration::from_millis(60000)) // make sure we never hit it the same minute
        }
        println!("Now {}", local);

        thread::sleep(Duration::from_millis(30000))
    }
}

fn test_servo_loop(delay_time: u64) -> Result<(), Box<dyn Error>> {
    // let pwm1 = Pwm::with_period(
    //     Channel::Pwm1,
    //     Duration::from_millis(PERIOD_MS),
    //     Duration::from_micros(PULSE_CLOSED_1_US),
    //     Polarity::Normal,
    //     true,
    // );
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

    // match feed_cat(&servo2, 1000) {
    //     // 2900 tot
    //     Ok(_) => println!("Fed the cat with servo 2"),
    //     Err(_) => println!("Failed to feed the cat with servo 2"),
    // }
    // pwm1?.disable().unwrap_or(());
    // thread::sleep(Duration::from_millis(3000));
    let pwm = Pwm::with_period(
        Channel::Pwm0,
        Duration::from_millis(PERIOD_MS),
        Duration::from_micros(PULSE_CLOSED_US),
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

    match feed_cat(&servo1, delay_time) {
        // 2150 tot
        Ok(_) => println!("Fed the cat with servo 1"),
        Err(_) => println!("Failed to feed the cat with servo 1"),
    }
    pwm?.disable().unwrap_or(());
    Ok(())
}

// fn index(info: web::Path<(u32, String)>) -> impl Responder {
//     format!("Hello {}! id: {}!", info.1, info.0)
// }

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => {
            let delay_time = u64::from_str_radix(&args[1], 10).unwrap();
            println!("Running servo test with delay time {}", delay_time);
            match test_servo_loop(delay_time) {
                Ok(_) => println!("Exited successfully"),
                Err(_) => println!("Error happened"),
            };
            Ok(())
        }
        _ => {
            // thread::spawn(|| {
            println!("Running feeder loop");
            match main_feeder_loop() {
                Ok(_) => println!("Exited successdully"),
                Err(_) => println!("Error happened"),
            }
            // });
            // HttpServer::new(|| {
            //     App::new().service(web::resource("/{id}/{name}/index.html").to(index))
            // })
            // .bind("192.168.1.187:8080")?
            // .run()
            Ok(())
        }
    }
}
