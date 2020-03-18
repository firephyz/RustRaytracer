// libsdl2 rust wrapper doesn't not provide gfx module

use std::time;

pub struct FramerateRegulator {
    framerate_duration: time::Duration,
    start_time: time::Instant,
    slack: time::Duration,
}

impl FramerateRegulator {
    pub fn new(framerate: u32) -> Self {
        let delay_nanos = (1e9 as u32) / framerate;
        FramerateRegulator {
            framerate_duration: time::Duration::new(0, delay_nanos),
            start_time: time::Instant::now(),
            slack: time::Duration::new(0, delay_nanos),
        }
    }

    // Return slack time, time delay waited
    pub fn delay(&mut self) -> time::Duration {
        let delay_start_time = time::Instant::now();

        let mut now_time = delay_start_time.clone();
        while (now_time - self.start_time) < self.framerate_duration {
            now_time = time::Instant::now();
        }
        self.start_time = time::Instant::now();

        now_time - delay_start_time
    }
}
