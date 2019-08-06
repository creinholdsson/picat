use rppal::pwm::Pwm;
pub struct Servo<'a> {
    pub pulse_closed: u64,
    pub pulse_open: u64,
    pub pulse_passed: u64,
    pub pwm: Option<&'a Pwm>,
}
