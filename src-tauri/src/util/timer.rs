use std::time::Instant;

pub struct Timer {
    time_print: Instant,
}

impl Timer {
    pub fn start() -> Self {
        Timer {
            time_print: Instant::now(),
        }
    }

    pub fn end(&self) -> u64 {
        (Instant::now() - self.time_print).as_secs()
    }
}
