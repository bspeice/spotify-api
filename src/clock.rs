use std::time::SystemTime;

pub trait Clock {
    fn now(&self) -> SystemTime;
}

struct SystemClock {}

impl Clock for SystemClock {
    fn now(&self) -> SystemTime {
        SystemTime::now()
    }
}
