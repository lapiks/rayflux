use std::time::{Duration, Instant};

/// Keep track of time
#[derive(Debug)]
pub struct Time {
    delta_time: f32,
    last_time: Instant,
    last_sec: Instant,
    frame_count: u64,
    fps: u64,
}

impl Default for Time {
    fn default() -> Self {
        Self { 
            delta_time: 0.0,
            last_time: Instant::now(),
            last_sec: Instant::now(),
            frame_count: 0,
            fps: 0,
        }
    }
}

impl Time {
    pub fn new() -> Self {
        Self::default()
    }

    /// Return delta time in secs
    pub fn tick(&mut self) -> f32 {
        let current_time = Instant::now();
        let delta = current_time.duration_since(self.last_time).as_secs_f32();
        self.last_time = current_time;
        self.delta_time = delta;

        self.frame_count += 1;
        if current_time.duration_since(self.last_sec) >= Duration::from_secs(1) {
            self.last_sec = current_time;
            self.fps = self.frame_count;
            self.frame_count = 0;
        }

        delta
    }

    pub fn fps(&self) -> u64 {
        self.fps
    }

    pub fn delta_time(&self) -> f32 {
        self.delta_time
    }

    pub fn frame_count(&self) -> u64 {
        self.frame_count
    }
}