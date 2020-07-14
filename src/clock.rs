use std::time::SystemTime;

pub trait Clock {
    fn now(&self) -> SystemTime;
}

pub struct SystemClock {}

impl SystemClock {
    pub fn new() -> SystemClock {
        SystemClock {}
    }
}

impl Clock for SystemClock {
    fn now(&self) -> SystemTime {
        SystemTime::now()
    }
}
