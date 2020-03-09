// libsdl2 rust wrapper doesn't not provide gfx module

use std::time;

pub struct FramerateRegulator {
    framerate_duration: time::Duration,
    start_time: time::Instant,
}

impl FramerateRegulator {
    pub fn new(framerate: u32) -> Self {
        let delay_nanos = (1e9 as u32) / framerate;
        FramerateRegulator {
            framerate_duration: time::Duration::new(0, delay_nanos),
            start_time: time::Instant::now(),
        }
    }

    pub fn delay(&mut self) {
        let mut now_time = time::Instant::now();
        while (now_time - self.start_time) < self.framerate_duration {
            now_time = time::Instant::now();
        }
        self.start_time = time::Instant::now();
    }
}
