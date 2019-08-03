use chrono::prelude::*;

use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::pwm::{Channel, Polarity, Pwm};

mod schedule;
mod servo;

const PERIOD_MS: u64 = 20;
const PULSE_OPEN_US: u64 = 1850;
const PULSE_CLOSED_US: u64 = 2400;
const PULSE_CLOSED_1_US: u64 = 1440;
const PULSE_OPEN_1_US: u64 = 860;

fn feed_cat(servo: &servo::Servo, feed_time: u64) -> Result<(), Box<dyn Error>> {
    match servo.pwm {
        Some(pwm) => {
            pwm.set_pulse_width(Duration::from_micros(servo.pulse_open))?;
            thread::sleep(Duration::from_millis(feed_time));
            pwm.set_pulse_width(Duration::from_micros(servo.pulse_closed))?;
        }
        None => {
            println!("Was gonna feed the cat!");
        }
    }
    Ok(())
}


fn main() -> Result<(), Box<dyn Error>> {
    let mut schedule = schedule::Schedule::new();
    schedule.push(Local.ymd(1970, 1, 1).and_hms(7, 30, 0));
    schedule.push(Local.ymd(1970, 1, 1).and_hms(11, 30, 0));
    schedule.push(Local.ymd(1970, 1, 1).and_hms(15, 30, 0));
    schedule.push(Local.ymd(1970, 1, 1).and_hms(19, 30, 0));
    
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
        Ok(ref p) => servo::Servo { pulse_closed: PULSE_CLOSED_US, pulse_open: PULSE_OPEN_US, pwm: Some(p) },
        Err(_) => {
            println!("Failed to create servo1, using dummy");
            servo::Servo { pulse_closed: PULSE_CLOSED_US, pulse_open: PULSE_OPEN_US, pwm: None }
        }
    };

    let servo2 = match pwm1 {
        Ok(ref p) => servo::Servo { pulse_closed: PULSE_CLOSED_1_US, pulse_open: PULSE_OPEN_1_US, pwm: Some(p) },
        Err(_) => {
            println!("Failed to create servo2, using dummy");
            servo::Servo { pulse_closed: PULSE_CLOSED_US, pulse_open: PULSE_OPEN_US, pwm: None }
        }
    };
    
    loop {
        let local = Local::now();

        if schedule.contains(local)  {
            match feed_cat(&servo2, 700) {
                Ok(_) => println!("Fed the cat with servo 2"),
                Err(_) => println!("Failed to feed the cat with servo 2")
            }
            thread::sleep(Duration::from_millis(6000));

            match feed_cat(&servo1, 350) {
                Ok(_) => println!("Fed the cat with servo 1"),
                Err(_) => println!("Failed to feed the cat with servo 1")
            }
            thread::sleep(Duration::from_millis(60000)) // make sure we never hit it the same minute
        }
        println!("Now {}", local);

        thread::sleep(Duration::from_millis(30000))
    }
}
